use crate::cluster::cluster::{ClusterInput, ClusterOutput, ClusterResult, ClusterTask};
use ddbtbl::cluster::cluster::{ClusterBmc, ClusterForCreate};
use ddbtbl::cluster::umap::{UmapBmc, UmapForCreate};
use iwf::ctx::Ctx;
use iwf::sql::ModelManager;
use iwf::{IwfExe, Result, WfParameters};
use log::debug;
use std::collections::HashMap;

#[derive(Clone)]
pub struct MiPatientUmapTask<'a, T>
where
    T: IwfExe + Clone,
{
    pub mm: &'a ModelManager,
    pub ctx: &'a Ctx,
    pub params: &'a WfParameters,
    pub gsea: T,
}

#[derive(sqlx::FromRow)]
struct PatientUmapQuery {
    ac_id: i32,
    ann_id: i32,
    value: f64,
}

impl<T> MiPatientUmapTask<'_, T>
where
    T: IwfExe + Clone,
{
    pub async fn execute(self) -> Result<MiPatientUmapResult> {
        let cluster_task = ClusterTask {
            input: "mi_patient_umap_input.tsv".into(),
            output: "mi_patient_umap_output.tsv".into(),
            min_cluster_size: 20,
            min_dist: 0.01,
            metric: "correlation".into(),
            ..Default::default()
        };
        let stmt = "select qm.ac_id,qm.ann_id,qm.scaled_value as value from qm inner join qmatrix on qmatrix_id = qmatrix.id inner join ac on ac_id = ac.id inner join labeldata on qm.ac_id = labeldata.ac::int where qmatrix.name = 'plmi' and global_label_string = 'heart'";
        let items: Vec<ClusterInput> = sqlx::query_as::<_, PatientUmapQuery>(stmt)
            .fetch_all(&self.mm.db)
            .await?
            .into_iter()
            .map(|e| ClusterInput {
                col: format!("{}", e.ac_id),
                idx: format!("c{}", e.ann_id),
                value: e.value,
            })
            .collect();
        debug!("Stmt: {stmt}; n items returned: {}", items.len());
        if items.is_empty() {
            return Ok(MiPatientUmapResult {});
        }
        cluster_task.write_input(&items).unwrap();
        if !cluster_task.output.is_file() {
            let cluster_result: ClusterResult = self.gsea.execute(self.params, &cluster_task)?;
            debug!("ClusterResult: {cluster_result}");
        }
        let results = cluster_task.read_output().unwrap();
        read_and_store_clusters(self.ctx, self.mm, &results, "fig4j").await?;

        Ok(MiPatientUmapResult {})
    }
}

async fn read_and_store_clusters(
    ctx: &Ctx,
    mm: &ModelManager,
    items: &Vec<ClusterOutput>,
    kind: &str,
) -> Result<()> {
    let mut cluster_map = HashMap::new();
    for item in items {
        let cm = if let Some(s) = cluster_map.get(&item.labels) {
            *s
        } else {
            let c = ClusterForCreate {
                label: format!("ucluster_{}_{}", kind, item.labels),
            };
            let id = ClusterBmc::create(ctx, mm, c).await?;
            cluster_map.insert(item.labels, id);
            id
        };
        let uc = UmapForCreate {
            cluster_id: cm,
            kind: kind.into(),
            idx: item.idx.clone(),
            y1: item.y1,
            y2: item.y2,
        };
        UmapBmc::create(ctx, mm, uc).await?;
    }
    Ok(())
}

#[derive(Debug)]
pub struct MiPatientUmapResult {}
