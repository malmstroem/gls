use crate::enrichment::gsea::{GseaInput, GseaResult, GseaTask};
use camino::Utf8PathBuf;
use ddbtbl::cluster::cluster_gsea::{GseaEnrBmc, GseaEnrForCreate};
use iwf::sql::ModelManager;
use iwf::{IwfExe, Result, WfParameters};
use log::warn;
use std::collections::{hash_map::Entry, HashMap};

#[derive(Clone)]
pub struct CalculateWkdeEnrichmentTask<'a, T>
where
    T: IwfExe + Clone,
{
    pub mm: &'a ModelManager,
    pub gmt: &'a Utf8PathBuf,
    pub params: &'a WfParameters,
    pub gsea: T,
}

#[derive(sqlx::FromRow)]
struct WkdeEnrichmentQuery {
    kind: String,
    gene: String,
    cluster: String,
}

impl<T> CalculateWkdeEnrichmentTask<'_, T>
where
    T: IwfExe + Clone,
{
    pub async fn execute(self) -> Result<CalculateWkdeEnrichmentResult> {
        let stmt = "select kind,labels as cluster,entry as gene from ac_labels inner join ac on ac.id = ac_id;".to_string();
        let items: Vec<WkdeEnrichmentQuery> = sqlx::query_as::<_, WkdeEnrichmentQuery>(&stmt)
            .fetch_all(&self.mm.db)
            .await?;
        let mut data: HashMap<String, HashMap<String, String>> = HashMap::new();
        for item in items {
            let knd = match data.entry(item.kind.clone()) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => v.insert(HashMap::new()),
            };
            knd.insert(item.gene.clone(), item.cluster.clone());
        }

        for (kind, entries) in data {
            let items = entries
                .into_iter()
                .map(|(k, v)| GseaInput {
                    gene: k,
                    cluster: v,
                })
                .collect();
            let gsea_task = GseaTask {
                input_path: format!("gsea_wkde_{kind}_input.tsv").into(),
                gmt_path: self.gmt.clone(),
                output_path: format!("gsea_wkde_{kind}_output.tsv").into(),
                ackind: "swissprot".into(),

                ..Default::default()
            };
            gsea_task.write_input(&items).unwrap();

            if !gsea_task.output_path.is_file() {
                let _gsea_result: GseaResult = self.gsea.execute(self.params, &gsea_task)?;
            }
            match gsea_task.read_output() {
                Ok(result) => {
                    let items = result
                        .into_iter()
                        .map(|e| GseaEnrForCreate {
                            kind: format!("wkde_{kind}"),
                            gene_set: e.gene_set,
                            term: e.term,
                            overlap: e.overlap,
                            pvalue: e.pvalue,
                            adjusted_pvalue: e.adjusted_pvalue,
                            odds_ratio: e.odds_ratio,
                            combined_score: e.combined_score,
                            genes: e.genes,
                            cluster_id: e.cluster_id,
                            n_query_genes: e.n_query_genes,
                        })
                        .collect();
                    GseaEnrBmc::bulk_import(self.mm, items).await?;
                }
                Err(err) => warn!("Could not read the wkde output file for {}: {}", kind, err),
            }
        }
        Ok(CalculateWkdeEnrichmentResult {})
    }
}

#[derive(Debug)]
pub struct CalculateWkdeEnrichmentResult {}
