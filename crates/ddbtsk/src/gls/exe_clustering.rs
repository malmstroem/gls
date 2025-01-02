use crate::cluster::cluster::{ClusterInput, ClusterOutput, ClusterResult, ClusterTask};
use ddbtbl::cluster::cluster::{ClusterBmc, ClusterForCreate};
use ddbtbl::cluster::umap::{UmapBmc, UmapForCreate};
use iwf::ctx::Ctx;
use iwf::sql::ModelManager;
use iwf::IwfExe;
use iwf::Result;
use iwf::WfParameters;
use log::debug;
use sqlx;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ExeClusteringTask<'a, T>
where
    T: IwfExe + Clone,
{
    pub ctx: &'a Ctx,
    pub mm: &'a ModelManager,
    pub gsea: T,
    pub params: &'a WfParameters,
    pub name: &'a str,
}

#[derive(sqlx::FromRow)]
struct ClusterQuery {
    ac_id: i32,
    ann_id: i32,
    value: f64,
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

impl<T> ExeClusteringTask<'_, T>
where
    T: IwfExe + Clone,
{
    pub async fn execute(self) -> Result<ExeClusteringResult> {
        let cluster_task = ClusterTask {
            input: format!("{}_umap_input.tsv", self.name).into(),
            output: format!("{}_umap_output.tsv", self.name).into(),
            min_cluster_size: 20,
            min_dist: 0.01,
            metric: "correlation".into(),
            ..Default::default()
        };
        let stmt = format!("select ac_id, ann_id, scaled_value as value from qm inner join qmatrix on qmatrix_id = qmatrix.id where qmatrix.name = '{}'", self.name);
        debug!("Stmt: {stmt}");
        let items: Vec<ClusterInput> = sqlx::query_as::<_, ClusterQuery>(&stmt)
            .fetch_all(&self.mm.db)
            .await?
            .into_iter()
            .map(|e| ClusterInput {
                idx: format!("{}", e.ac_id),
                col: format!("c{}", e.ann_id),
                value: e.value,
            })
            .collect();
        println!("N {}: {}", self.name, items.len());
        cluster_task.write_input(&items).unwrap();
        if !cluster_task.output.is_file() {
            let cluster_result: ClusterResult = self.gsea.execute(self.params, &cluster_task)?;
            debug!("ClusterResult: {cluster_result}");
        }
        let results = cluster_task.read_output().unwrap();
        read_and_store_clusters(self.ctx, self.mm, &results, self.name).await?;

        Ok(ExeClusteringResult {})
    }
}

#[derive(Debug)]
pub struct ExeClusteringResult {}
