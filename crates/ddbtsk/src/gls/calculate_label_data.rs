use ddbtbl::gls::labeldata::{DataSet, FullLabelData, LabelDataBmc, LabelDataForCreate, Tissue};
use ddbtbl::gls::scoresetting::{ScoreSettingBmc, ScoreSettingForCreate};
use iwf::ctx::Ctx;
use iwf::sql::ModelManager;
use iwf::Result;
use log::debug;
use std::collections::{hash_map::Entry, HashMap, HashSet};

#[derive(Clone)]
pub struct CalculateLabelDataTask<'a> {
    pub ctx: &'a Ctx,
    pub mm: &'a ModelManager,
}

#[allow(dead_code)]
#[derive(sqlx::FromRow)]
struct LabelDataQuery {
    kind: String,
    tissues: String,
    entry: String,
    ac_id: i32,
    ann_id: i32,
}

impl CalculateLabelDataTask<'_> {
    pub async fn execute(self) -> Result<CalculateLabelDataResult> {
        let stmt= "select kind,global_grp as tissues,entry,ac_id,ann.id as ann_id from wkdelabel inner join ac on ac_id = ac.id inner join ann on ann.id = any(labels)";
        let mut lbldata = HashMap::new();
        let items: Vec<LabelDataQuery> = sqlx::query_as::<_, LabelDataQuery>(stmt)
            .fetch_all(&self.mm.db)
            .await?;
        debug!("N items returned from wkdelabel: {}", items.len());
        for item in items {
            let l = match lbldata.entry(item.ac_id) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => v.insert(FullLabelData {
                    ac: format!("{}", item.ac_id),
                    ..Default::default()
                }),
            };
            let t = match l.label.entry(DataSet::from_str(&item.kind)) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => v.insert(HashSet::new()),
            };
            t.insert(Tissue::from_str(&item.tissues).unwrap());
        }
        let score_setting = ScoreSettingForCreate::default();
        let score_id = ScoreSettingBmc::create(self.ctx, self.mm, score_setting.clone()).await?;
        let mut vv: Vec<LabelDataForCreate> = vec![];
        for (_protein, mut label) in lbldata {
            label.score_setting_id = score_id;
            label.count(&score_setting);
            label.create_labels(&score_setting);
            label.create_global_label(&score_setting);
            vv.push(label.into());
        }
        LabelDataBmc::bulk_import(self.mm, vv).await?;
        Ok(CalculateLabelDataResult {})
    }
}

#[derive(Debug)]
pub struct CalculateLabelDataResult {}
