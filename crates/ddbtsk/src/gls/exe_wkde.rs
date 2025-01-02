use crate::cluster::wkde::{WkdeInput, WkdeResult, WkdeTask};
use camino::Utf8PathBuf;
use ddbtbl::gls::wkdelabel::{WkdeLabelBmc, WkdeLabelForCreate};
use ddbtbl::gls::wkdetag::{WkdeTagBmc, WkdeTagForCreate};
use iwf::sql::ModelManager;
use iwf::IwfExe;
use iwf::Result;
use iwf::WfParameters;
#[allow(unused_imports)]
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use sqlx;

#[derive(Debug, sqlx::FromRow, Default, Serialize, Deserialize)]
struct WkdeQuery {
    idx: String,
    y1: f64,
    y2: f64,
    col: String,
    weights: f64,
}

#[derive(Clone)]
pub struct ExeWkdeTask<'a, T>
where
    T: IwfExe + Clone,
{
    pub mm: &'a ModelManager,
    pub params: &'a WfParameters,
    pub wkde: T,
    pub kind: &'a str,
    pub fraction: f64,
    pub bandwidth: f64,
    pub min_value: f64,
    pub no_class_criteria: f64,
}

impl<T> ExeWkdeTask<'_, T>
where
    T: IwfExe + Clone,
{
    pub async fn execute(self) -> Result<ExeWkdeResult> {
        let qstmt = format!("select idx,y1,y2,ann_id::text as col,scaled_value as weights from umap inner join qm on umap.idx::int4 = qm.ac_id inner join ann on ann_id = ann.id where kind = '{}' and qmatrix_type = '{}'", self.kind, self.kind);
        let quant_items: Vec<WkdeInput> = sqlx::query_as::<_, WkdeQuery>(&qstmt)
            .fetch_all(&self.mm.db)
            .await?
            .into_iter()
            .map(|e| WkdeInput {
                idx: e.idx,
                col: e.col,
                y1: e.y1,
                y2: e.y2,
                weights: e.weights,
            })
            .collect();

        let input_path = Utf8PathBuf::from(format!("wkde_{}.tsv", self.kind));
        println!(
            "wkde: N quant returned: {}; {input_path}",
            quant_items.len()
        );
        let task = WkdeTask {
            input_path,
            fraction: self.fraction,
            bandwidth: self.bandwidth,
            min_value: self.min_value,
            no_class_criteria: self.no_class_criteria,
            max_n_labels: 2,
            name: format!("wkde_{}", self.kind),
            ..Default::default()
        };
        task.write_input(&quant_items).unwrap();
        let result: WkdeResult = self.wkde.execute(self.params, &task)?;
        println!("Len RR: qunat: {}; {}", quant_items.len(), result);
        let (data1, data2) = match task.read_output() {
            Ok(o) => o,
            Err(e) => {
                warn!("Reading output failed: {e}; task: {task}");
                return Ok(ExeWkdeResult {});
            }
        };
        let in1: Vec<WkdeTagForCreate> = data1
            .into_iter()
            .map(|e| WkdeTagForCreate {
                idx: e.index,
                n_pixels: e.n_pixels,
                n_labels: e.n_labels,
                kind: self.kind.into(),
                label: e.label,
            })
            .collect();
        WkdeTagBmc::bulk_import(self.mm, in1).await?;
        let in2: Vec<WkdeLabelForCreate> = data2
            .into_iter()
            .map(|e| {
                let labels: Vec<i32> = match e.clss.as_ref() {
                    "none" => vec![-2],
                    "common" => vec![-1],
                    _ => e.clss.split('.').map(|s| s.parse().unwrap()).collect(),
                };
                WkdeLabelForCreate {
                    ac_id: e.idx.parse::<i32>().unwrap(),
                    kind: self.kind.into(),
                    labels,
                }
            })
            .collect();
        println!("wdke N returned create statements: {}", in2.len());
        WkdeLabelBmc::bulk_import(self.mm, in2).await?;

        Ok(ExeWkdeResult {})
    }
}

#[derive(Debug)]
pub struct ExeWkdeResult {}
