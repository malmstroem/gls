use crate::gls::scoresetting::ScoreSettingForCreate;
use iwf::ctx::Ctx;
use iwf::sql::base::{self, DbBmc, BTREE};
use iwf::sql::ModelManager;
use iwf::sql::Result;
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsFloat64, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::fmt;
use tabled::Tabled;

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum DataSet {
    #[serde(rename = "haatlas_clusters")]
    HaAtlas,
    #[serde(rename = "msratlas_clusters")]
    MsrAtlas,
    #[serde(rename = "mspatlas_clusters")]
    MspAtlas,
    #[serde(rename = "emblatlas_clusters")]
    EmblAtlas,
    #[serde(rename = "emblcells_clusters")]
    EmblCells,
    #[serde(rename = "hacells_clusters")]
    HaCells,
}

impl fmt::Display for DataSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::HaAtlas => write!(f, "HaAtlas"),
            Self::HaCells => write!(f, "HaCells"),
            Self::EmblAtlas => write!(f, "EmblAtlas"),
            Self::MsrAtlas => write!(f, "MsrAtlas"),
            Self::MspAtlas => write!(f, "MspAtlas"),
            Self::EmblCells => write!(f, "EmblCells"),
        }
    }
}

impl DataSet {
    #[must_use]
    pub fn from_str(name: &str) -> Self {
        match name {
            "msratlas" => Self::MsrAtlas,
            "emblatlas" => Self::EmblAtlas,
            "mspatlas" => Self::MspAtlas,
            "haatlas" => Self::HaAtlas,
            "hacells" => Self::HaCells,
            "emblcells" => Self::EmblCells,
            _ => panic!("Unknown dataset: {name}"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Tissue {
    Brain,
    Muscle,
    Nerve,
    AdiposeTissue,
    AdrenalGland,
    Artery,
    Bcell,
    Bladder,
    Bonemarrow,
    Colon,
    Erythrocytes,
    Esophagus,
    Heart,
    Kidney,
    Liver,
    Lung,
    Macrophages,
    Monocytes,
    Neutrophils,
    Ovary,
    Pancreas,
    Platelets,
    Prostate,
    Skin,
    Spleen,
    Stomach,
    TcellCd4,
    TcellCd8,
    Thyroid,
    Common,
    NoTissue,
    Plasma,
    Cmi,
    Mi,
    Nvar,
    Cpancr,
    Pancr,
    Bsep,
    Vsep,
}

impl fmt::Display for Tissue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Brain => write!(f, "brain"),
            Self::Muscle => write!(f, "muscle"),
            Self::Nerve => write!(f, "nerve"),
            Self::AdiposeTissue => write!(f, "adiposetissue"),
            Self::AdrenalGland => write!(f, "adrenalgland"),
            Self::Artery => write!(f, "artery"),
            Self::Bcell => write!(f, "bcell"),
            Self::Bladder => write!(f, "bladder"),
            Self::Bonemarrow => write!(f, "bonemarrow"),
            Self::Colon => write!(f, "colon"),
            Self::Common => write!(f, "common"),
            Self::Erythrocytes => write!(f, "erythrocytes"),
            Self::Esophagus => write!(f, "esophagus"),
            Self::Heart => write!(f, "heart"),
            Self::Kidney => write!(f, "kidney"),
            Self::Liver => write!(f, "liver"),
            Self::Lung => write!(f, "lung"),
            Self::Macrophages => write!(f, "macrophages"),
            Self::Monocytes => write!(f, "monocytes"),
            Self::Neutrophils => write!(f, "neutrophils"),
            Self::NoTissue => write!(f, "no_tissue"),
            Self::Ovary => write!(f, "ovary"),
            Self::Pancreas => write!(f, "pancreas"),
            Self::Platelets => write!(f, "platelets"),
            Self::Prostate => write!(f, "prostate"),
            Self::Skin => write!(f, "skin"),
            Self::Spleen => write!(f, "spleen"),
            Self::Stomach => write!(f, "stomach"),
            Self::TcellCd4 => write!(f, "tcellcd4"),
            Self::TcellCd8 => write!(f, "tcellcd8"),
            Self::Thyroid => write!(f, "thyroid"),
            Self::Plasma => write!(f, "plasma"),
            Self::Cmi => write!(f, "cmi"),
            Self::Mi => write!(f, "mi"),
            Self::Nvar => write!(f, "nvar"),
            Self::Pancr => write!(f, "pancr"),
            Self::Cpancr => write!(f, "cpancr"),
            Self::Bsep => write!(f, "bsep"),
            Self::Vsep => write!(f, "vsep"),
        }
    }
}

impl Tissue {
    pub fn from_str(input: &str) -> Result<Self> {
        match input {
            "brain" => Ok(Self::Brain),
            "muscle" => Ok(Self::Muscle),
            "nerve" => Ok(Self::Nerve),
            "adiposetissue" => Ok(Self::AdiposeTissue),
            "adrenalgland" => Ok(Self::AdrenalGland),
            "artery" => Ok(Self::Artery),
            "bcell" => Ok(Self::Bcell),
            "bladder" => Ok(Self::Bladder),
            "bonemarrow" => Ok(Self::Bonemarrow),
            "colon" => Ok(Self::Colon),
            "common" => Ok(Self::Common),
            "erythrocytes" => Ok(Self::Erythrocytes),
            "esophagus" => Ok(Self::Esophagus),
            "heart" => Ok(Self::Heart),
            "kidney" => Ok(Self::Kidney),
            "liver" => Ok(Self::Liver),
            "lung" => Ok(Self::Lung),
            "macrophage" => Ok(Self::Macrophages),
            "macrophages" => Ok(Self::Macrophages),
            "monocytes" => Ok(Self::Monocytes),
            "neutrophils" => Ok(Self::Neutrophils),
            "none" => Ok(Self::NoTissue),
            "ovary" => Ok(Self::Ovary),
            "pancreas" => Ok(Self::Pancreas),
            "platelets" => Ok(Self::Platelets),
            "prostate" => Ok(Self::Prostate),
            "skin" => Ok(Self::Skin),
            "spleen" => Ok(Self::Spleen),
            "stomach" => Ok(Self::Stomach),
            "tcellcd4" => Ok(Self::TcellCd4),
            "tcellcd8" => Ok(Self::TcellCd8),
            "thyroid" => Ok(Self::Thyroid),
            "cmi" => Ok(Self::Cmi),
            "mi" => Ok(Self::Mi),
            "nvar" => Ok(Self::Nvar),
            "pancr" => Ok(Self::Pancr),
            "cpancr" => Ok(Self::Cpancr),
            "vsep" => Ok(Self::Bsep),
            "bsep" => Ok(Self::Vsep),
            _ => panic!("Cannot match (tissue) {input}"),
        }
    }
}

impl Tissue {
    #[must_use]
    pub fn from_tag(grp: &str) -> Self {
        match grp {
            "adipo" => Self::AdiposeTissue,
            "aorta" => Self::Artery,
            "bcell" => Self::Bcell,
            "bladd" => Self::Bladder,
            "bonem" => Self::Bonemarrow,
            "brain" => Self::Brain,
            "endon" => Self::NoTissue,
            "eryth" => Self::Erythrocytes,
            "fasci" => Self::NoTissue,
            "heart" => Self::Heart,
            "kidne" => Self::Kidney,
            "liver" => Self::Liver,
            "lung" => Self::Lung,
            "macro" => Self::Macrophages,
            "monoc" => Self::Monocytes,
            "muscl" => Self::Muscle,
            "nerve" => Self::Nerve,
            "neutr" => Self::Neutrophils,
            "ovary" => Self::Ovary,
            "pancr" => Self::Pancreas,
            "plate" => Self::Platelets,
            "prost" => Self::Prostate,
            "sigmo" => Self::Colon,
            "skin" => Self::Skin,
            "splee" => Self::Spleen,
            "stoma" => Self::Stomach,
            "tcell4" => Self::TcellCd4,
            "tcell8" => Self::TcellCd8,
            "cmi" => Self::Cmi,
            "mi" => Self::Mi,
            "nvar" => Self::Nvar,
            "cpancr" => Self::Cpancr,
            "vsep" => Self::Vsep,
            "bsep" => Self::Bsep,
            _ => panic!("Unknown group"),
        }
    }
    #[must_use]
    pub fn get_color(&self) -> String {
        match self {
            Self::AdrenalGland => String::from("grey"),
            Self::Artery => String::from("grey"),
            Self::Colon => String::from("grey"),
            Self::Esophagus => String::from("grey"),
            Self::Thyroid => String::from("grey"),
            Self::Common => String::from("grey"),
            Self::NoTissue => String::from("grey"),
            Self::Plasma => String::from("grey"),
            Self::AdiposeTissue => String::from("grey"),
            Self::Bcell => String::from("blue"),
            Self::Bladder => String::from("grey"),
            Self::Bonemarrow => String::from("grey"),
            Self::Brain => String::from("blue"),
            Self::Erythrocytes => String::from("grey"),
            Self::Heart => String::from("red"),
            Self::Kidney => String::from("grey"),
            Self::Liver => String::from("green"),
            Self::Lung => String::from("grey"),
            Self::Macrophages => String::from("grey"),
            Self::Monocytes => String::from("grey"),
            Self::Muscle => String::from("yellow"),
            Self::Nerve => String::from("cyan"),
            Self::Neutrophils => String::from("grey"),
            Self::Ovary => String::from("grey"),
            Self::Pancreas => String::from("black"),
            Self::Platelets => String::from("grey"),
            Self::Prostate => String::from("grey"),
            Self::Skin => String::from("grey"),
            Self::Spleen => String::from("grey"),
            Self::Stomach => String::from("grey"),
            Self::TcellCd4 => String::from("grey"),
            Self::TcellCd8 => String::from("grey"),
            Self::Cmi => String::from("grey"),
            Self::Mi => String::from("blue"),
            Self::Nvar => String::from("grey"),
            Self::Pancr => String::from("blue"),
            Self::Cpancr => String::from("grey"),
            Self::Bsep => String::from("orange"),
            Self::Vsep => String::from("cyan"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ScoreCount {
    pub n: i32,
    pub score: f64,
}

fn format_label(labels: &HashSet<Tissue>) -> Result<String> {
    let mut v = vec![];
    for ent in labels {
        v.push(ent.to_string());
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    Ok(v.join("."))
}

#[derive(Debug, Serialize)]
pub struct FullLabelData {
    #[serde(skip)]
    pub label: HashMap<DataSet, HashSet<Tissue>>,
    pub score_setting_id: i32,
    pub ac: String,
    pub description: String,
    pub n_measure: i32,
    pub n_tissue: i32,
    pub n_cell: i32,
    pub n_common: i32,
    pub max_atlas_score: f64,
    pub cummulative_atlas_score: f64,
    pub max_cell_score: f64,
    pub cummulative_cell_score: f64,
    #[serde(skip)]
    pub tissue: HashMap<Tissue, ScoreCount>,
    #[serde(skip)]
    pub cell: HashMap<Tissue, ScoreCount>,
    #[serde(skip)]
    pub tissue_label: HashSet<Tissue>,
    pub tissue_label_string: String,
    #[serde(skip)]
    pub cell_label: HashSet<Tissue>,
    pub cell_label_string: String,
    #[serde(skip)]
    pub global_label: HashSet<Tissue>,
    pub global_label_string: String,
    pub cell_entropy: f32,
    pub cell_entropy_label: String,
    #[serde(skip)]
    pub cell_entropy_labels: Vec<String>,
    pub tissue_entropy: f32,
    pub tissue_entropy_label: String,
    #[serde(skip)]
    pub tissue_entropy_labels: Vec<String>,
    pub entropy_label: String,
}

impl FullLabelData {
    pub fn count(&mut self, settings: &ScoreSettingForCreate) {
        self.n_measure = 0;
        self.n_tissue = 0;
        self.n_cell = 0;
        self.n_common = 0;
        let mut atlas = HashMap::new();
        let mut cell = HashMap::new();
        for (a, b) in &self.label {
            if a.to_string().ends_with("Atlas") {
                for tissue in b {
                    let n_per_label = 6 / b.len() as i32;
                    for _ in 0..n_per_label {
                        self.tissue_entropy_labels.push(tissue.to_string());
                    }
                    if tissue != &Tissue::Common {
                        let tval = match atlas.entry(tissue) {
                            Entry::Occupied(o) => o.into_mut(),
                            Entry::Vacant(v) => v.insert(ScoreCount { n: 0, score: 0.0 }),
                        };
                        tval.n += 1;
                        if b.len() == 1 {
                            tval.score += 1.0;
                        } else {
                            tval.score += settings.multi_label_weight / b.len() as f64;
                        }
                        if tval.score > self.max_atlas_score {
                            self.max_atlas_score = tval.score;
                        }
                    }
                }
                self.n_tissue += 1;
            } else if a.to_string().ends_with("Cells") {
                for vlue in b {
                    let n_per_label = 6 / b.len() as i32;
                    for _ in 0..n_per_label {
                        self.cell_entropy_labels.push(vlue.to_string());
                    }
                    if vlue != &Tissue::Common {
                        let tval = match cell.entry(vlue) {
                            Entry::Occupied(o) => o.into_mut(),
                            Entry::Vacant(v) => v.insert(ScoreCount { n: 0, score: 0.0 }),
                        };
                        tval.n += 1;
                        if b.len() == 1 {
                            tval.score += 1.0;
                        } else {
                            tval.score += settings.multi_label_weight / b.len() as f64;
                        }
                        if tval.score > self.max_cell_score {
                            self.max_cell_score = tval.score;
                        }
                    }
                }
                self.n_cell += 1;
            }
            if b.contains(&Tissue::Common) {
                self.n_common += 1;
            }
            self.n_measure += 1;
        }
        while self.cell_entropy_labels.len() < 12 {
            self.cell_entropy_labels.push(String::from("missing"));
        }
        while self.tissue_entropy_labels.len() < 24 {
            self.tissue_entropy_labels.push(String::from("missing"));
        }
        for (k, v) in atlas {
            self.tissue.insert(*k, v);
        }
        for (k, v) in cell {
            self.cell.insert(*k, v);
        }
    }
    pub fn create_labels(&mut self, settings: &ScoreSettingForCreate) {
        self.cummulative_cell_score = 0.0;
        self.cummulative_atlas_score = 0.0;
        let mut tissue_labels = HashSet::new();
        for (k, v) in &self.tissue {
            if v.score > self.max_atlas_score * settings.delta_to_include && k != &Tissue::Common {
                tissue_labels.insert(k);
                self.cummulative_atlas_score += v.score;
            }
        }
        if tissue_labels.len() > settings.max_labels as usize {
            self.tissue_label = HashSet::from([Tissue::Common]);
        } else {
            for k in tissue_labels {
                self.tissue_label.insert(*k);
            }
        }
        let mut cell_labels = HashSet::new();
        for (k, v) in &self.cell {
            if v.score > self.max_cell_score * settings.delta_to_include && k != &Tissue::Common {
                cell_labels.insert(k);
                self.cummulative_cell_score += v.score;
            }
        }
        if cell_labels.len() > settings.max_labels as usize {
            self.cell_label = HashSet::from([Tissue::Common]);
        } else {
            for k in cell_labels {
                self.cell_label.insert(*k);
            }
        }
        self.tissue_label_string = format_label(&self.tissue_label).unwrap();
        self.cell_label_string = format_label(&self.cell_label).unwrap();
        if self.cell_entropy_label.is_empty() || self.cell_entropy_label == "missing" {
            self.cell_entropy = 100.0;
        } else {
            self.cell_entropy = 0.0;
        }
        if self.tissue_entropy_label.is_empty() || self.tissue_entropy_label == "missing" {
            self.tissue_entropy = 100.0;
        } else {
            self.tissue_entropy = 0.0;
        }
        if self.global_label_string == "plasma" || self.global_label_string == "common" {
            self.entropy_label = self.global_label_string.clone();
        } else if self.tissue_entropy <= self.cell_entropy {
            self.entropy_label = self.tissue_entropy_label.clone();
        } else {
            self.entropy_label = self.cell_entropy_label.clone();
        }
    }
    pub fn create_global_label(&mut self, settings: &ScoreSettingForCreate) {
        if self.is_plasma() {
            self.global_label = HashSet::from([Tissue::Plasma]);
        } else if self.n_common > settings.common_threashold {
            self.global_label = HashSet::from([Tissue::Common]);
        } else if self.cummulative_atlas_score
            > self.cummulative_cell_score * settings.cell_atlas_weight
        {
            self.global_label = self.tissue_label.clone();
        } else {
            self.global_label = self.cell_label.clone();
        }
        self.global_label_string = format_label(&self.global_label).unwrap();
    }

    fn is_plasma(&self) -> bool {
        let mut n_protein_liver = 0;
        let mut n_rna_liver = 0;
        for (ds, lbl) in &self.label {
            if (ds == &DataSet::HaAtlas || ds == &DataSet::MspAtlas) && lbl.contains(&Tissue::Liver)
            {
                n_protein_liver += 1;
            }
            if (ds == &DataSet::EmblAtlas || ds == &DataSet::MsrAtlas)
                && lbl.contains(&Tissue::Liver)
            {
                n_rna_liver += 1;
            }
        }
        if n_protein_liver == 0 && n_rna_liver >= 2 {
            return true;
        }
        false
    }
}

impl fmt::Display for FullLabelData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "N: {} ({}/{}); common: {}; max: {:.2}/{:.2}",
            self.n_measure,
            self.n_tissue,
            self.n_cell,
            self.n_common,
            self.max_atlas_score,
            self.max_cell_score
        )
    }
}

impl Default for FullLabelData {
    fn default() -> Self {
        Self {
            label: HashMap::new(),
            score_setting_id: 0,
            ac: String::new(),
            description: String::new(),
            n_measure: -1,
            n_tissue: -1,
            n_cell: -1,
            n_common: -1,
            max_atlas_score: -1.0,
            max_cell_score: -1.0,
            cummulative_atlas_score: -1.0,
            cummulative_cell_score: -1.0,
            tissue: HashMap::new(),
            cell: HashMap::new(),
            tissue_label: HashSet::new(),
            cell_label: HashSet::new(),
            global_label: HashSet::new(),
            cell_label_string: String::new(),
            tissue_label_string: String::new(),
            global_label_string: String::new(),
            cell_entropy: -1.0,
            cell_entropy_label: String::new(),
            cell_entropy_labels: vec![],
            tissue_entropy: -1.0,
            tissue_entropy_label: String::new(),
            tissue_entropy_labels: vec![],
            entropy_label: String::new(),
        }
    }
}

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct LabelDataForCreate {
    pub ac: String,
    pub score_setting_id: i32,
    pub cell_entropy: f64,
    pub cell_entropy_label: String,
    pub cell_label_string: String,
    pub cummulative_atlas_score: f64,
    pub cummulative_cell_score: f64,
    pub description: String,
    pub entropy_label: String,
    pub global_label_string: String,
    pub max_atlas_score: f64,
    pub max_cell_score: f64,
    pub n_cell: i32,
    pub n_common: i32,
    pub n_measure: i32,
    pub n_tissue: i32,
    pub tissue_entropy: f64,
    pub tissue_entropy_label: String,
    pub tissue_label_string: String,
}

impl From<FullLabelData> for LabelDataForCreate {
    fn from(item: FullLabelData) -> Self {
        Self {
            ac: item.ac,
            score_setting_id: item.score_setting_id,
            cell_entropy: f64::from(item.cell_entropy),
            cell_entropy_label: item.cell_entropy_label,
            cell_label_string: item.cell_label_string,
            cummulative_atlas_score: item.cummulative_atlas_score,
            cummulative_cell_score: item.cummulative_cell_score,
            entropy_label: item.entropy_label,
            max_atlas_score: item.max_cell_score,
            max_cell_score: item.max_cell_score,
            n_cell: item.n_cell,
            n_tissue: item.n_tissue,
            n_common: item.n_common,
            n_measure: item.n_measure,
            tissue_entropy: f64::from(item.tissue_entropy),
            tissue_label_string: item.tissue_label_string,
            global_label_string: item.global_label_string,
            description: item.description,
            tissue_entropy_label: item.tissue_entropy_label,
        }
    }
}

impl LabelDataBmc {
    pub async fn bulk_import(mm: &ModelManager, entries: Vec<LabelDataForCreate>) -> Result<()> {
        let mut tx = mm.db().begin().await?;
        for entry in entries {
            let _result = sqlx::query(&format!(
            "INSERT INTO {} (ac,cell_entropy,cell_entropy_label,cell_label_string,cummulative_atlas_score,cummulative_cell_score,description,entropy_label,global_label_string,max_atlas_score,max_cell_score,n_cell,n_common,n_measure,n_tissue,score_setting_id,tissue_entropy,tissue_entropy_label,tissue_label_string)
    VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19)",
            Self::TABLE
        ))
        .bind(entry.ac)
        .bind(entry.cell_entropy)
        .bind(entry.cell_entropy_label)
        .bind(entry.cell_label_string)
        .bind(entry.cummulative_atlas_score)
        .bind(entry.cummulative_cell_score)
        .bind(entry.description)
        .bind(entry.entropy_label)
        .bind(entry.global_label_string)
        .bind(entry.max_atlas_score)
        .bind(entry.max_cell_score)
        .bind(entry.n_cell)
        .bind(entry.n_common)
        .bind(entry.n_measure)
        .bind(entry.n_tissue)
        .bind(entry.score_setting_id)
        .bind(entry.tissue_entropy)
        .bind(entry.tissue_entropy_label)
        .bind(entry.tissue_label_string)
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
  score_setting_id integer not null,
  ac character varying not null,
  cell_entropy float not null,
  cell_entropy_label character varying not null,
  cell_label_string character varying not null,
  cummulative_atlas_score float not null,
  cummulative_cell_score float not null,
  description character varying not null,
  entropy_label character varying not null,
  global_label_string character varying not null,
  max_atlas_score float not null,
  max_cell_score float not null,
  n_cell integer not null,
  n_common integer not null,
  n_measure integer not null,
  n_tissue integer not null,
  tissue_entropy float not null,
  tissue_entropy_label character varying not null,
  tissue_label_string character varying not null
);
create index if not exists "IDX_{table}_ac" ON {table} {BTREE} (ac);
create index if not exists "IDX_{table}_score_setting_id" ON {table} {BTREE} (score_setting_id);
        "##,
            match drop_table {
                true => format!("drop table if exists {table};"),
                false => String::new(),
            }
        )
    }
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default, Tabled)]
pub struct LabelData {
    pub id: i32,
    pub score_setting_id: i32,
    pub ac: String,
    pub cell_entropy: f64,
    pub cell_entropy_label: String,
    pub cell_label_string: String,
    pub cummulative_atlas_score: f64,
    pub cummulative_cell_score: f64,
    pub description: String,
    pub entropy_label: String,
    pub global_label_string: String,
    pub max_atlas_score: f64,
    pub max_cell_score: f64,
    pub n_cell: i32,
    pub n_common: i32,
    pub n_measure: i32,
    pub n_tissue: i32,
    pub tissue_entropy: f64,
    pub tissue_entropy_label: String,
    pub tissue_label_string: String,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct LabelDataForUpdate {
    pub ac: Option<String>,
    pub cell_entropy: Option<f64>,
    pub cell_entropy_label: Option<String>,
    pub cell_label_string: Option<String>,
    pub cummulative_atlas_score: Option<f64>,
    pub cummulative_cell_score: Option<f64>,
    pub description: Option<String>,
    pub entropy_label: Option<String>,
    pub global_label_string: Option<String>,
    pub max_atlas_score: Option<f64>,
    pub max_cell_score: Option<f64>,
    pub n_cell: Option<i32>,
    pub n_common: Option<i32>,
    pub n_measure: Option<i32>,
    pub n_tissue: Option<i32>,
    pub tissue_entropy: Option<f64>,
    pub tissue_entropy_label: Option<String>,
    pub tissue_label_string: Option<String>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct LabelDataFilter {
    id: Option<OpValsInt64>,
    ac: Option<OpValsString>,
    cell_entropy: Option<OpValsFloat64>,
    cell_entropy_label: Option<OpValsString>,
    cell_label_string: Option<OpValsString>,
    cummulative_atlas_score: Option<OpValsFloat64>,
    cummulative_cell_score: Option<OpValsFloat64>,
    description: Option<OpValsString>,
    entropy_label: Option<OpValsString>,
    global_label_string: Option<OpValsString>,
    max_atlas_score: Option<OpValsFloat64>,
    max_cell_score: Option<OpValsFloat64>,
    n_cell: Option<OpValsInt64>,
    n_common: Option<OpValsInt64>,
    n_measure: Option<OpValsInt64>,
    n_tissue: Option<OpValsInt64>,
    tissue_entropy: Option<OpValsFloat64>,
    tissue_entropy_label: Option<OpValsString>,
    tissue_label_string: Option<OpValsString>,
}

pub struct LabelDataBmc;

impl DbBmc for LabelDataBmc {
    const TABLE: &'static str = "labeldata";
}

impl LabelDataBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, clone_c: LabelDataForCreate) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, clone_c).await
    }
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<LabelData> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<LabelDataFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<LabelData>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        clone_u: LabelDataForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, clone_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}
