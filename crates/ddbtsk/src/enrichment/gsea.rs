use crate::error::Result;
use camino::Utf8PathBuf;
use iwf::{IwfResult, IwfTask};
use iwfmacros::Iwfargs;
use serde::{Deserialize, Serialize};

#[derive(Iwfargs, Clone, Debug)]
#[iwfarg(executable = "run_gsea.py")]
pub struct GseaTask {
    #[iwfarg(flag = "--input-gene-tsv")]
    pub input_path: Utf8PathBuf,
    #[iwfarg(flag = "--output-cluster-gsea")]
    pub output_path: Utf8PathBuf,
    #[iwfarg(flag = "--gmt")]
    pub gmt_path: Utf8PathBuf,
    #[iwfarg(flag = "--ackind")]
    pub ackind: String,
    #[iwfarg(flag = "argument")]
    pub command: String,
}

impl Default for GseaTask {
    fn default() -> Self {
        Self {
            input_path: Utf8PathBuf::from("gsea_input.tsv"),
            output_path: Utf8PathBuf::from("gsea_output.tsv"),
            gmt_path: Utf8PathBuf::new(),
            ackind: String::from("symbol"),
            command: String::from("run"),
        }
    }
}

impl IwfTask for GseaTask {}

impl std::fmt::Display for GseaTask {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

#[derive(Debug)]
pub struct GseaResult {
    pub out: String,
}

impl IwfResult<GseaTask> for GseaResult {
    fn new(out: String, _err: String, _task: &GseaTask) -> Self {
        Self { out }
    }
}

impl std::fmt::Display for GseaResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct GseaInput {
    pub gene: String,
    pub cluster: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GseaOutput {
    #[serde(rename = "Gene_set")]
    pub gene_set: String,
    #[serde(rename = "Term")]
    pub term: String,
    #[serde(rename = "Overlap")]
    pub overlap: String,
    #[serde(rename = "P-value")]
    pub pvalue: f64,
    #[serde(rename = "Adjusted P-value")]
    pub adjusted_pvalue: f64,
    #[serde(rename = "Odds Ratio")]
    pub odds_ratio: f64,
    #[serde(rename = "Combined Score")]
    pub combined_score: f64,
    #[serde(rename = "Genes")]
    pub genes: String,
    pub cluster_id: String,
    pub n_query_genes: i32,
}

impl GseaTask {
    pub fn write_input(&self, input_data: &Vec<GseaInput>) -> Result<()> {
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b'\t')
            .from_path(&self.input_path)?;

        for item in input_data {
            wtr.serialize(item)?;
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
        Ok(())
    }

    pub fn read_output(&self) -> Result<Vec<GseaOutput>> {
        let mut ret = vec![];
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_path(&self.output_path)?;
        for rec in rdr.deserialize() {
            let item: GseaOutput = rec?;
            ret.push(item);
        }
        Ok(ret)
    }
}
