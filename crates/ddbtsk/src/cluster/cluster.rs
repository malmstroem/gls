use crate::error::Result;
use camino::Utf8PathBuf;
use iwf::{IwfResult, IwfTask};
use iwfmacros::Iwfargs;
use serde::{Deserialize, Serialize};

#[derive(Iwfargs, Clone, Debug)]
#[iwfarg(executable = "umap_cluster.py")]
pub struct ClusterTask {
    #[iwfarg(flag = "-i")]
    pub input: Utf8PathBuf,
    #[iwfarg(flag = "-o")]
    pub output: Utf8PathBuf,
    #[iwfarg(flag = "--index_column")]
    pub index_column: String,
    #[iwfarg(flag = "--column")]
    pub column: String,
    #[iwfarg(flag = "--value")]
    pub value: String,
    #[iwfarg(flag = "--min_cluster_size")]
    pub min_cluster_size: i32,
    #[iwfarg(flag = "--random_state")]
    pub random_state: i32,
    #[iwfarg(flag = "--n_components")]
    pub n_components: i32,
    #[iwfarg(flag = "--min_dist")]
    pub min_dist: f64,
    #[iwfarg(flag = "--n_neighbors")]
    pub n_neighbors: i32,
    #[iwfarg(flag = "--metric")]
    pub metric: String,
    #[iwfarg(flag = "argument")]
    pub command: String,
}

#[derive(Serialize)]
pub struct ClusterInput {
    pub idx: String,
    pub col: String,
    pub value: f64,
}

#[derive(Deserialize)]
pub struct ClusterOutput {
    pub idx: String,
    pub y1: f64,
    pub y2: f64,
    pub labels: i32,
}

impl Default for ClusterTask {
    fn default() -> Self {
        Self {
            input: "cluster_input.tsv".into(),
            output: "cluster_output.tsv".into(),
            command: "run".into(),
            index_column: "idx".into(),
            column: "col".into(),
            value: "value".into(),
            min_cluster_size: 20,
            min_dist: 0.1,
            n_neighbors: 15,
            n_components: 2,
            random_state: 0,
            metric: "euclidean".into(),
        }
    }
}

impl ClusterTask {
    pub fn write_input(&self, items: &Vec<ClusterInput>) -> Result<()> {
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b'\t')
            .from_path(&self.input)?;
        for item in items {
            wtr.serialize(item)?;
        }
        Ok(())
    }
    pub fn read_output(&self) -> Result<Vec<ClusterOutput>> {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_path(&self.output)?;
        let mut items = vec![];
        for rec in rdr.deserialize() {
            let item: ClusterOutput = rec?;
            items.push(item);
        }
        Ok(items)
    }
}

impl IwfTask for ClusterTask {}

impl std::fmt::Display for ClusterTask {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

#[derive(Debug)]
pub struct ClusterResult {
    pub out: String,
}

impl IwfResult<ClusterTask> for ClusterResult {
    fn new(out: String, _err: String, _task: &ClusterTask) -> Self {
        Self { out }
    }
}

impl std::fmt::Display for ClusterResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}
