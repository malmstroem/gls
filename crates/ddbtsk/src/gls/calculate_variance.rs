use ddbtbl::gls::variance::{VarianceBmc, VarianceForCreate};
use iwf::sql::ModelManager;
use iwf::Result;
use log::debug;
use std::collections::{hash_map::Entry, HashMap};

#[derive(Clone)]
pub struct CalculateVarianceTask<'a> {
    pub mm: &'a ModelManager,
    pub mpr: &'a HashMap<String, (String, String)>,
}

#[derive(sqlx::FromRow)]
struct VarianceQuery {
    display_name: String,
    ac_id: i32,
    norm_value: f64,
}

impl CalculateVarianceTask<'_> {
    pub async fn execute(self) -> Result<CalculateVarianceResult> {
        let stmt = "select display_name,ac_id,norm_value from qm inner join qmatrix on qmatrix_id = qmatrix.id inner join ann on ann_id = ann.id where qmatrix.name = 'plnvar' and measured = true";
        let items = sqlx::query_as::<_, VarianceQuery>(stmt)
            .fetch_all(&self.mm.db)
            .await?;
        debug!("N rows returned {}", items.len());
        let mut data = HashMap::new();
        for item in items {
            match self.mpr.get(&item.display_name) {
                None => panic!("Cannot find {}", item.display_name),
                Some(enr) => {
                    let ky = match data.entry(item.ac_id) {
                        Entry::Vacant(v) => v.insert(HashMap::new()),
                        Entry::Occupied(o) => o.into_mut(),
                    };
                    let tm = match ky.entry(&enr.1) {
                        Entry::Vacant(v) => v.insert(vec![]),
                        Entry::Occupied(o) => o.into_mut(),
                    };
                    tm.push(item.norm_value);
                    let pt = match ky.entry(&enr.0) {
                        Entry::Vacant(v) => v.insert(vec![]),
                        Entry::Occupied(o) => o.into_mut(),
                    };
                    pt.push(item.norm_value);
                }
            }
        }
        let mut stats = HashMap::new();
        for (ac_id, vals1) in data {
            let (time, patient) = match stats.entry(ac_id) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => v.insert((vec![], vec![])),
            };
            for (seri, vals2) in vals1 {
                let mean: f64 = vals2.iter().sum::<f64>() / vals2.len() as f64;
                let variance: f64 = vals2
                    .iter()
                    .map(|value| (value - mean).powi(2))
                    .sum::<f64>()
                    / vals2.len() as f64;
                let std_dev = variance.sqrt();
                if seri.starts_with('t') {
                    time.push((mean, std_dev));
                } else {
                    patient.push((mean, std_dev));
                }
            }
        }
        let mut results = vec![];
        for (ac_id, (time, patient)) in stats {
            let time_means: Vec<f64> = time.iter().map(|e| e.0).collect();
            let time_std_devs: Vec<f64> = time.iter().map(|e| e.1).collect();
            let patient_means: Vec<f64> = patient.iter().map(|e| e.0).collect();
            let patient_std_devs: Vec<f64> = patient.iter().map(|e| e.1).collect();
            let time_fc = VarianceForCreate {
                ac_id,
                kind: "time".into(),
                mean: time_means.iter().sum::<f64>() / time_means.len() as f64,
                variance: time_std_devs.iter().sum::<f64>() / time_std_devs.len() as f64,
            };
            let patient_fc = VarianceForCreate {
                ac_id,
                kind: "patient".into(),
                mean: patient_means.iter().sum::<f64>() / patient_means.len() as f64,
                variance: patient_std_devs.iter().sum::<f64>() / patient_std_devs.len() as f64,
            };
            results.push(time_fc);
            results.push(patient_fc);
        }
        VarianceBmc::bulk_import(self.mm, results).await?;

        Ok(CalculateVarianceResult {})
    }
}

#[derive(Debug)]
pub struct CalculateVarianceResult {}
