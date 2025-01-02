use crate::gls::ac::{Ac, AcBmc};
use crate::gls::ann::{Ann, AnnBmc, AnnFilter};
use crate::gls::qmatrix::{QmatrixBmc, QmatrixForCreate};
use camino::Utf8PathBuf;
use iwf::ctx::Ctx;
use iwf::sql::base::{self, DbBmc, BTREE};
use iwf::sql::ModelManager;
use iwf::sql::Result;
use log::{debug, warn};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;
use std::collections::{HashMap, HashSet};
use tabled::Tabled;

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct QmForCreate {
    pub qmatrix_id: i32,
    pub ac_id: i32,
    pub ann_id: i32,
    pub value: f64,
    pub norm_value: f64,
    pub scaled_value: f64,
    pub measured: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct QmParse {
    #[serde(alias = "id", alias = "gene.id")]
    pub protein: String,
    pub variable: String,
    pub value: f64,
}

fn get_ac(protein: &str, acm: &HashMap<String, Ac>, acfm: &HashMap<String, Ac>) -> Option<Ac> {
    match acm.get(protein) {
        Some(s) => Some(s.clone()),
        None => acfm.get(protein).cloned(),
    }
}

fn read_and_normalize(
    input_path: &Utf8PathBuf,
    annm: &HashMap<String, Ann>,
    acm: &HashMap<String, Ac>,
    acfm: &HashMap<String, Ac>,
    not_found_samples: &mut HashSet<String>,
    not_found_proteins: &mut HashSet<String>,
    qmatrix_id: i32,
    ex: bool,
) -> Result<Vec<QmForCreate>> {
    let mut all_values = vec![];
    let mut present = HashSet::new();
    let mut present_acs = HashSet::new();
    let mut present_anns = HashSet::new();
    let mut norm_data: HashMap<i32, f64> = HashMap::new();

    let mut ret = vec![];
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(input_path)?;
    for line in rdr.deserialize() {
        let parsed: QmParse = line?;
        let to_match = parsed.variable.replace(',', "");
        match annm.get(&to_match) {
            None => {
                not_found_samples.insert(to_match);
                ();
            }
            Some(ann) => {
                let ann_id = ann.id;
                match get_ac(&parsed.protein, acm, acfm) {
                    None => {
                        not_found_proteins.insert(parsed.protein);
                        ();
                    }
                    Some(s) => {
                        if parsed.value > 0.0 {
                            let value = match ex {
                                false => parsed.value,
                                true => f64::exp2(parsed.value),
                            };
                            let record = QmForCreate {
                                qmatrix_id,
                                ac_id: s.id,
                                ann_id,
                                value,
                                norm_value: 0.,
                                scaled_value: 0.,
                                measured: true,
                            };
                            ret.push(record);
                            all_values.push(parsed.value);
                            present_acs.insert(s.id);
                            present_anns.insert(ann_id);
                            present.insert((s.id, ann_id));
                            norm_data
                                .entry(ann_id)
                                .and_modify(|curr| *curr += parsed.value)
                                .or_insert(parsed.value);
                        }
                    }
                }
            }
        }
    }
    normalize_reads(
        &mut ret,
        &present,
        &present_acs,
        &present_anns,
        &mut all_values,
        qmatrix_id,
        &mut norm_data,
    )?;

    Ok(ret)
}

fn normalize_reads(
    ret: &mut Vec<QmForCreate>,
    present: &HashSet<(i32, i32)>,
    present_acs: &HashSet<i32>,
    present_anns: &HashSet<i32>,
    all_values: &mut Vec<f64>,
    qmatrix_id: i32,
    norm_data: &mut HashMap<i32, f64>,
) -> Result<()> {
    all_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let one_percent_count = (all_values.len() as f64 * 0.01).ceil() as usize;
    let first_one_percent = &all_values[..one_percent_count];
    let mut n_have = 0;
    let mut n_missing = 0;
    let mut rng = thread_rng();
    for ann in present_anns {
        for ac in present_acs {
            if present.contains(&(*ac, *ann)) {
                n_have += 1;
                continue;
            }
            n_missing += 1;
            let value = *first_one_percent.choose(&mut rng).unwrap();
            let record = QmForCreate {
                qmatrix_id,
                ac_id: *ac,
                ann_id: *ann,
                value,
                norm_value: 0.,
                scaled_value: 0.,
                measured: false,
            };
            norm_data
                .entry(*ann)
                .and_modify(|curr| *curr += value)
                .or_insert(value);
            ret.push(record);
        }
    }
    debug!("distr {} {}", n_have, n_missing);
    let norm_sum = norm_data.values().sum::<f64>() / (norm_data.len() as f64);
    let mut norm_factors = HashMap::new();
    for (k, v) in norm_data {
        let factor = *v / norm_sum;
        debug!("Factor: {k} {v} {factor}");
        norm_factors.insert(k, factor);
    }
    for r in ret.iter_mut() {
        let nvalue = r.value * *norm_factors.get(&r.ann_id).unwrap();
        r.norm_value = nvalue;
    }

    Ok(())
}

fn scale(ret: &mut Vec<QmForCreate>, impute: bool) -> Result<()> {
    let mut max_scale_data: HashMap<i32, f64> = HashMap::new();
    let mut min_scale_data: HashMap<i32, f64> = HashMap::new();
    let mut sum_data: HashMap<i32, f64> = HashMap::new();
    for r in ret.iter() {
        let include = match impute {
            true => true,
            false => r.measured,
        };
        if include {
            sum_data
                .entry(r.ac_id)
                .and_modify(|current_sum| *current_sum += r.norm_value)
                .or_insert(r.norm_value);
            max_scale_data
                .entry(r.ac_id)
                .and_modify(|current_max| {
                    if r.norm_value > *current_max {
                        *current_max = r.norm_value;
                    }
                })
                .or_insert(r.norm_value);
            min_scale_data
                .entry(r.ac_id)
                .and_modify(|current_min| {
                    if r.norm_value < *current_min {
                        *current_min = r.norm_value;
                    }
                })
                .or_insert(r.norm_value);
        };
    }
    for r in ret.iter_mut() {
        if let Some(sum) = sum_data.get(&r.ac_id) {
            match r.measured {
                true => r.scaled_value = r.norm_value / sum,
                false => match impute {
                    false => r.scaled_value = 0.0,
                    true => r.scaled_value = r.norm_value / sum,
                },
            };
        }
    }
    Ok(())
}

async fn get_ac_maps(
    ctx: &Ctx,
    mm: &ModelManager,
) -> Result<(HashMap<String, Ac>, HashMap<String, Ac>)> {
    let op = ListOptions {
        limit: Some(50_000),
        ..Default::default()
    };

    let acs = AcBmc::list(ctx, mm, None, Some(op.clone())).await?;
    let acm: HashMap<String, Ac> = acs.into_iter().map(|e| (e.entry.clone(), e)).collect();
    let acs = AcBmc::list(ctx, mm, None, Some(op)).await?;
    let acfm: HashMap<String, Ac> = acs.into_iter().map(|e| (e.frm.clone(), e)).collect();
    Ok((acm, acfm))
}

async fn create_qmatrix(ctx: &Ctx, mm: &ModelManager, matrix: &str) -> Result<i32> {
    let qmatrix_id = QmatrixBmc::create(
        ctx,
        mm,
        QmatrixForCreate {
            typ: "qmatrix".into(),
            name: matrix.into(),
            idx_column: "qmatrix".into(),
            qmatrixdf_name: "qmatrix".into(),
        },
    )
    .await?;
    Ok(qmatrix_id)
}

async fn get_ann_map(ctx: &Ctx, mm: &ModelManager, matrix: &str) -> Result<HashMap<String, Ann>> {
    let op = ListOptions {
        limit: Some(50_000),
        ..Default::default()
    };
    let filters: Vec<AnnFilter> =
        serde_json::from_value(json!([{"qmatrix_type": {"$eq": matrix}}])).unwrap();
    let anns = AnnBmc::list(ctx, mm, Some(filters), Some(op)).await?;
    debug!("AnnLen: {}", anns.len());
    let annm: HashMap<String, Ann> = anns
        .into_iter()
        .map(|e| (e.measurement.clone(), e))
        .collect();
    Ok(annm)
}

impl QmBmc {
    pub async fn parse(
        ctx: &Ctx,
        mm: &ModelManager,
        input_path: &Utf8PathBuf,
        matrix: String,
        ex: bool,
        impute: bool,
    ) -> Result<Vec<QmForCreate>> {
        debug!("Input Path: {}", input_path);
        let qmatrix_id = create_qmatrix(ctx, mm, &matrix).await?;
        let (acm, acfm) = get_ac_maps(ctx, mm).await?;
        let annm = get_ann_map(ctx, mm, &matrix).await?;
        let mut not_found_proteins: HashSet<String> = HashSet::new();
        let mut not_found_samples: HashSet<String> = HashSet::new();
        let mut ret = read_and_normalize(
            input_path,
            &annm,
            &acm,
            &acfm,
            &mut not_found_samples,
            &mut not_found_proteins,
            qmatrix_id,
            ex,
        )?;
        scale(&mut ret, impute)?;
        warn!("N proteins not found: {}", not_found_proteins.len(),);
        warn!("N samples not found: {}:", not_found_samples.len());
        iwf::msg(
            json!({"src":file!(), "line":line!(), "file":input_path.to_string(), "samples_not_found":not_found_samples}),
        );
        iwf::msg(
            json!({"src":file!(), "line":line!(), "file":input_path.to_string(), "proteins_not_found":not_found_proteins}),
        );
        Ok(ret)
    }
    pub async fn bulk_import(mm: &ModelManager, entries: Vec<QmForCreate>) -> Result<()> {
        let mut tx = mm.db().begin().await?;
        for entry in entries {
            let _result = sqlx::query(&format!(
                "INSERT INTO {} (qmatrix_id,ac_id,ann_id,value,norm_value, scaled_value, measured)
        VALUES ($1, $2, $3, $4, $5, $6, $7)",
                Self::TABLE
            ))
            .bind(entry.qmatrix_id)
            .bind(entry.ac_id)
            .bind(entry.ann_id)
            .bind(entry.value)
            .bind(entry.norm_value)
            .bind(entry.scaled_value)
            .bind(entry.measured)
            .execute(&mut *tx)
            .await;
        }
        tx.commit().await?;
        Ok(())
    }
    #[must_use]
    pub fn get_create_sql(drop_table: bool) -> String {
        let table = Self::TABLE;
        format!(
            r##"{}
create table if not exists {table} (
  id serial primary key,
  qmatrix_id integer not null,
  ac_id integer not null,
  ann_id integer not null,
  value float not null,
  norm_value float not null,
  scaled_value float not null,
  measured bool not null
);
create index if not exists "IDX_{table}_qmatrix_id" ON {table} {BTREE} (qmatrix_id);
create index if not exists "IDX_{table}_ac_id" ON {table} {BTREE} (ac_id);
create index if not exists "IDX_{table}_ann_id" ON {table} {BTREE} (ann_id);
        "##,
            match drop_table {
                true => format!("drop table if exists {table};"),
                false => String::new(),
            }
        )
    }
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default, Tabled)]
pub struct Qm {
    pub id: i32,
    pub qmatrix_id: i32,
    pub ac_id: i32,
    pub ann_id: i32,
    pub value: f64,
    pub norm_value: f64,
    pub scaled_value: f64,
    pub measured: bool,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct QmForUpdate {
    pub ac: Option<String>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct QmFilter {
    id: Option<OpValsInt64>,
    qmatrix_id: Option<OpValsInt64>,
    ac_id: Option<OpValsInt64>,
    ann_id: Option<OpValsInt64>,
}

pub struct QmBmc;

impl DbBmc for QmBmc {
    const TABLE: &'static str = "qm";
}

#[derive(Debug, sqlx::FromRow, Default, Serialize, Deserialize, Tabled)]
pub struct HaatlasQmatrix {
    ac_id: i32,
    global_grp: String,
    value: f64,
    n: i64,
}

pub async fn create_haatlas_qmatrix(ctx: &Ctx, mm: &ModelManager) -> Result<()> {
    let qms = vec![
        "haatlas".to_string(),
        "hacells".to_string(),
        "mspatlas".into(),
        "msratlas".into(),
        "emblcells".into(),
    ];
    for qm in qms {
        let qmatrix_id = create_qmatrix(ctx, mm, &qm).await?;
        let annm = get_ann_map(ctx, mm, &qm).await?;
        let items: Vec<HaatlasQmatrix> = sqlx::query_as::<_, HaatlasQmatrix>(&format!("select ac_id,global_grp,avg(value) as value,count(*) as n from qm inner join ann on ann_id = ann.id inner join qmatrix on qmatrix.id=  qmatrix_id where qmatrix.name = '{qm}raw' and measured = 'true' group by ac_id,global_grp;"))
        .fetch_all(&mm.db)
        .await?;
        debug!("Item: {}", items.len());
        let mut ret = vec![];
        let mut all_values = vec![];
        let mut present = HashSet::new();
        let mut present_acs = HashSet::new();
        let mut present_anns = HashSet::new();
        let mut norm_data: HashMap<i32, f64> = HashMap::new();
        for item in items {
            let ann = annm.get(&item.global_grp).unwrap();
            let fc = QmForCreate {
                qmatrix_id,
                ac_id: item.ac_id,
                ann_id: ann.id,
                value: item.value,
                norm_value: 0.,
                scaled_value: 0.,
                measured: true,
            };
            all_values.push(fc.value);
            present_acs.insert(item.ac_id);
            present_anns.insert(ann.id);
            present.insert((item.ac_id, ann.id));
            norm_data
                .entry(ann.id)
                .and_modify(|curr| *curr += fc.value)
                .or_insert(fc.value);
            ret.push(fc);
        }
        all_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let one_percent_count = (all_values.len() as f64 * 0.01).ceil() as usize;
        let first_one_percent = &all_values[..one_percent_count];
        debug!(
            "LENS: {} {} {} {} {} {} {}",
            present_acs.len(),
            present_anns.len(),
            present.len(),
            all_values.len(),
            first_one_percent.len(),
            all_values[all_values.len() - 1],
            first_one_percent[first_one_percent.len() - 1],
        );
        let mut n_have = 0;
        let mut n_missing = 0;
        let mut rng = thread_rng();
        for ann in &present_anns {
            for ac in &present_acs {
                if present.contains(&(*ac, *ann)) {
                    n_have += 1;
                    continue;
                }
                n_missing += 1;
                let value = *first_one_percent.choose(&mut rng).unwrap();
                let record = QmForCreate {
                    qmatrix_id,
                    ac_id: *ac,
                    ann_id: *ann,
                    value,
                    norm_value: 0.,
                    scaled_value: 0.,
                    measured: false,
                };
                norm_data
                    .entry(*ann)
                    .and_modify(|curr| *curr += value)
                    .or_insert(value);
                ret.push(record);
            }
        }
        debug!("distr {} {}", n_have, n_missing);
        let norm_sum = norm_data.values().sum::<f64>() / (norm_data.len() as f64);
        let mut norm_factors = HashMap::new();
        for (k, v) in norm_data {
            let factor = v / norm_sum;
            debug!("{k} {v} {factor}");
            norm_factors.insert(k, factor);
        }
        for r in &mut ret {
            let nvalue = r.value * *norm_factors.get(&r.ann_id).unwrap();
            r.norm_value = nvalue;
        }
        scale(&mut ret, false)?;
        debug!("RET: {}", ret.len());
        QmBmc::bulk_import(mm, ret).await?;
    }
    Ok(())
}

impl QmBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, clone_c: QmForCreate) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, clone_c).await
    }
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Qm> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<QmFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Qm>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(ctx: &Ctx, mm: &ModelManager, id: i32, clone_u: QmForUpdate) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, clone_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}
