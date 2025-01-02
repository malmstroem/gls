use crate::enrichment::gsea::{GseaInput, GseaResult, GseaTask};
use camino::Utf8PathBuf;
use ddbtbl::cluster::cluster_gsea::{GseaEnrBmc, GseaEnrForCreate};
use iwf::sql::ModelManager;
use iwf::{IwfExe, Result, WfParameters};
#[allow(unused_imports)]
use log::{debug, warn};

#[derive(Clone)]
pub struct CalculateEnrichmentFig2eTask<'a, T>
where
    T: IwfExe + Clone,
{
    pub mm: &'a ModelManager,
    pub gsea: T,
    pub params: &'a WfParameters,
    pub gmt: &'a Utf8PathBuf,
}

#[derive(sqlx::FromRow)]
struct Fig2e {
    entry: String,
    cluster: String,
}

impl<T> CalculateEnrichmentFig2eTask<'_, T>
where
    T: IwfExe + Clone,
{
    pub async fn execute(self) -> Result<CalculateEnrichmentFig2eResult> {
        let kind = "brain_gls";
        let stmt = "select entry,(gls::int)::text as cluster from gls inner join ac on ac::int = ac.id where global_label_string = 'brain'";
        let items = sqlx::query_as::<_, Fig2e>(stmt)
            .fetch_all(&self.mm.db)
            .await?
            .into_iter()
            .map(|e| GseaInput {
                gene: e.entry.clone(),
                cluster: e.cluster,
            })
            .collect();
        let gsea_task = GseaTask {
            input_path: format!("gsea_{kind}_input.tsv").into(),
            gmt_path: self.gmt.clone(),
            output_path: format!("gsea_{kind}_output.tsv").into(),
            ackind: "swissprot".into(),

            ..Default::default()
        };
        gsea_task.write_input(&items).unwrap();

        if !gsea_task.output_path.is_file() {
            let gsea_result: GseaResult = self.gsea.execute(self.params, &gsea_task)?;
            warn!("GseaResult: {gsea_result}");
        }
        match gsea_task.read_output() {
            Ok(result) => {
                let items = result
                    .into_iter()
                    .map(|e| GseaEnrForCreate {
                        kind: kind.into(),
                        gene_set: e.gene_set,
                        term: e.term,
                        overlap: e.overlap,
                        pvalue: e.pvalue,
                        adjusted_pvalue: e.adjusted_pvalue,
                        odds_ratio: e.odds_ratio,
                        combined_score: e.combined_score,
                        genes: e.genes,
                        cluster_id: format!("{}_{}", kind, e.cluster_id),
                        n_query_genes: e.n_query_genes,
                    })
                    .collect();
                GseaEnrBmc::bulk_import(self.mm, items).await?;
            }
            Err(err) => warn!("Could not read the output file for {kind}: {err}"),
        }
        Ok(CalculateEnrichmentFig2eResult {})
    }
}

#[derive(Debug)]
pub struct CalculateEnrichmentFig2eResult {}
