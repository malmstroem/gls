use crate::enrichment::gsea::{GseaInput, GseaResult, GseaTask};
use camino::Utf8PathBuf;
use ddbtbl::cluster::cluster_gsea::{GseaEnrBmc, GseaEnrForCreate};
use iwf::sql::ModelManager;
use iwf::{IwfExe, Result, WfParameters};
use log::{debug, warn};

#[derive(Clone)]
pub struct CalculateEnrichmentTask<'a, T>
where
    T: IwfExe + Clone,
{
    pub mm: &'a ModelManager,
    pub gmt: &'a Utf8PathBuf,
    pub params: &'a WfParameters,
    pub gsea: T,
}

#[derive(sqlx::FromRow)]
struct GseaQmatrixQuery {
    qmatrix_id: i32,
}

#[derive(sqlx::FromRow)]
struct GseaQuery {
    gene: String,
    cluster: String,
}

impl<T> CalculateEnrichmentTask<'_, T>
where
    T: IwfExe + Clone,
{
    pub async fn execute(self) -> Result<CalculateEnrichmentResult> {
        let qmatrix_ids: Vec<GseaQmatrixQuery> =
            sqlx::query_as::<_, GseaQmatrixQuery>("SELECT DISTINCT qmatrix_id FROM glsna;")
                .fetch_all(&self.mm.db)
                .await?;
        for qmatrix_id in qmatrix_ids {
            let stmt = format!("select entry as gene, multilabel as cluster from glsna inner join ac on ac_id = ac.id where qmatrix_id = {};", qmatrix_id.qmatrix_id);
            let query_items: Vec<GseaQuery> = sqlx::query_as::<_, GseaQuery>(&stmt)
                .fetch_all(&self.mm.db)
                .await?;
            let items = query_items
                .into_iter()
                .map(|e| GseaInput {
                    gene: e.gene,
                    cluster: e.cluster,
                })
                .collect();
            let gsea_task = GseaTask {
                input_path: format!("gsea_{}_input.tsv", qmatrix_id.qmatrix_id).into(),
                gmt_path: self.gmt.clone(),
                output_path: format!("gsea_{}_output.tsv", qmatrix_id.qmatrix_id).into(),
                ackind: "swissprot".into(),

                ..Default::default()
            };
            gsea_task.write_input(&items).unwrap();

            if !gsea_task.output_path.is_file() {
                let gsea_result: GseaResult = self.gsea.execute(self.params, &gsea_task)?;
                debug!("GseaResult: {gsea_result}");
            }
            match gsea_task.read_output() {
                Ok(result) => {
                    let items = result
                        .into_iter()
                        .map(|e| GseaEnrForCreate {
                            kind: format!("{}", qmatrix_id.qmatrix_id),
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
                Err(err) => warn!(
                    "Could not read the output file for {}: {}",
                    qmatrix_id.qmatrix_id, err
                ),
            }
        }
        Ok(CalculateEnrichmentResult {})
    }
}

#[derive(Debug)]
pub struct CalculateEnrichmentResult {}
