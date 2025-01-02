use crate::error::Result;
use camino::Utf8PathBuf;
use iwf::{IwfResult, IwfTask};
use iwfmacros::Iwfargs;
use serde::{Deserialize, Serialize};

#[derive(Iwfargs, Clone, Debug)]
#[iwfarg(executable = "run_wkde.py")]
pub struct WkdeTask {
    #[iwfarg(flag = "--input-tsv")]
    pub input_path: Utf8PathBuf,
    #[iwfarg(flag = "--color-map")]
    pub color: String,
    #[iwfarg(flag = "--fraction")]
    pub fraction: f64,
    #[iwfarg(flag = "--bandwidth")]
    pub bandwidth: f64,
    #[iwfarg(flag = "--min-value")]
    pub min_value: f64,
    #[iwfarg(flag = "--no-class-criteria")]
    pub no_class_criteria: f64,
    #[iwfarg(flag = "--max-n-labels")]
    pub max_n_labels: i32,
    #[iwfarg(flag = "--name")]
    pub name: String,
    #[iwfarg(flag = "argument")]
    pub command: String,
}

impl Default for WkdeTask {
    fn default() -> Self {
        Self {
            input_path: "wkde_input.tsv".into(),
            color: "red.tsv".into(),
            command: "run".into(),
            fraction: 0.70,
            bandwidth: 0.10,
            min_value: 0.10,
            no_class_criteria: 0.05,
            max_n_labels: 2,
            name: "wkde".into(),
        }
    }
}

impl IwfTask for WkdeTask {}

impl std::fmt::Display for WkdeTask {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct WkdeInput {
    pub idx: String,
    pub y1: f64,
    pub y2: f64,
    pub col: String,
    pub weights: f64,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct WkdeOutput {
    pub label: String,
    pub index: i32,
    pub n_labels: i32,
    pub n_pixels: i32,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct WkdeLabelOutput {
    pub idx: String,
    pub y1: f64,
    pub y2: f64,
    pub clss: String,
    pub c1: i32,
    pub c2: i32,
    pub v1: f64,
    pub v2: f64,
}

impl WkdeTask {
    pub fn write_input(&self, input_data: &Vec<WkdeInput>) -> Result<()> {
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b'\t')
            .from_path(&self.input_path)?;

        for item in input_data {
            wtr.serialize(item)?;
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
        Ok(())
    }

    pub fn read_output(&self) -> Result<(Vec<WkdeOutput>, Vec<WkdeLabelOutput>)> {
        let mut labels = vec![];
        let mut output_path = self.input_path.parent().unwrap().to_owned();
        output_path.push(format!("{}_color_map.tsv", self.name,));
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_path(&output_path)?;
        for rec in rdr.deserialize() {
            let item: WkdeOutput = rec?;
            labels.push(item);
        }
        let mut assignments = vec![];
        let mut output_path = self.input_path.parent().unwrap().to_owned();
        output_path.push(format!("{}_classes.csv", self.name));
        println!("Output Path: {output_path}");
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_path(&output_path)?;
        for rec in rdr.deserialize() {
            let item: WkdeLabelOutput = rec?;
            assignments.push(item);
        }
        Ok((labels, assignments))
    }
}

#[derive(Debug)]
pub struct WkdeResult {
    pub out: String,
}

impl IwfResult<WkdeTask> for WkdeResult {
    fn new(out: String, _err: String, _task: &WkdeTask) -> Self {
        Self { out }
    }
}

impl std::fmt::Display for WkdeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
