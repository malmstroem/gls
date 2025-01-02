use crate::enrichment::gsea::{GseaInput, GseaResult, GseaTask};
use camino::Utf8PathBuf;
use ddbtbl::cluster::cluster_gsea::{GseaEnrBmc, GseaEnrForCreate};
use iwf::sql::ModelManager;
use iwf::{IwfExe, Result, WfParameters};
use log::{debug, warn};

#[derive(Clone)]
pub struct CalculateEnrichmentListTask<'a, T>
where
    T: IwfExe + Clone,
{
    pub mm: &'a ModelManager,
    pub kind: &'a str,
    pub gmt: &'a Utf8PathBuf,
    pub gsea: T,
    pub params: &'a WfParameters,
    pub protein_list: &'a Vec<String>,
}

impl<T> CalculateEnrichmentListTask<'_, T>
where
    T: IwfExe + Clone,
{
    pub async fn execute(self) -> Result<CalculateEnrichmentListResult> {
        let items = self
            .protein_list
            .iter()
            .map(|e| GseaInput {
                gene: e.clone(),
                cluster: "1".into(),
            })
            .collect();
        let gsea_task = GseaTask {
            input_path: format!("gsea_{}_input.tsv", self.kind).into(),
            gmt_path: self.gmt.clone(),
            output_path: format!("gsea_{}_output.tsv", self.kind).into(),
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
                        kind: self.kind.into(),
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
            Err(err) => warn!("Could not read the output file for {}: {err}", self.kind),
        }
        Ok(CalculateEnrichmentListResult {})
    }
}

#[derive(Debug)]
pub struct CalculateEnrichmentListResult {}
