use crate::Result;
use camino::Utf8PathBuf;
use log::debug;
use serde_derive::{Deserialize, Serialize};
use std::io::{prelude::*, BufReader};

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Obj {
    pub items: Vec<Target>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum SynonymSources {
    #[serde(rename = "uniprot")]
    Uniprot,
    #[serde(rename = "NCBI_entrez")]
    NcbiEntrez,
    #[serde(rename = "HGNC")]
    Hgnc,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Synonym {
    pub label: String,
    pub source: SynonymSources,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Target {
    pub id: String,
    #[serde(rename = "approvedSymbol")]
    pub approved_symbol: String,
    pub biotype: String,
    #[serde(rename = "approvedName")]
    pub approved_name: String,
    #[serde(rename = "symbolSynonyms")]
    pub synonyms: Vec<Synonym>,
    #[serde(rename = "canonicalTranscript")]
    pub transcript: Option<Transcript>,
    pub go: Option<Vec<Go>>,
    #[serde(rename = "functionDescriptions")]
    pub function_descriptions: Option<Vec<String>>,
    #[serde(rename = "subcellularLocations")]
    pub subcellular_locations: Option<Vec<Location>>,
    pub constraint: Option<Vec<Constraint>>,
    #[serde(rename = "proteinIds")]
    pub protein_ids: Option<Vec<Protein>>,
    pub tractability: Option<Vec<Tractability>>,
    pub pathways: Option<Vec<Pathway>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pathway {
    #[serde(rename = "pathwayId")]
    pub pathway_id: String,
    #[serde(rename = "topLevelTerm")]
    pub top_level_term: String,
    pub pathway: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tractability {
    pub id: String,
    pub modality: String,
    pub value: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Protein {
    pub id: String,
    pub source: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Constraint {
    #[serde(rename = "constraintType")]
    pub constraint_type: String,
    pub score: Option<f32>,
    pub exp: Option<f32>,
    pub obs: Option<f32>,
    pub oe: Option<f32>,
    #[serde(rename = "oeLower")]
    pub oe_lower: Option<f32>,
    #[serde(rename = "oeUpper")]
    pub oe_upper: Option<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    pub location: String,
    pub source: String,
    #[serde(rename = "termSL")]
    pub term_sl: Option<String>,
    #[serde(rename = "labelSL")]
    pub label_sl: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Go {
    pub id: String,
    pub source: String,
    pub evidence: String,
    pub aspect: String,
    #[serde(rename = "geneProduct")]
    pub gene_product: String,
    #[serde(rename = "ecoId")]
    pub eco_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transcript {
    pub id: String,
    pub chromosome: String,
    pub start: u64,
    pub end: u64,
    pub strand: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Symbol2EnsId {
    pub approved_symbol: String,
    pub ens_id: String,
}

impl From<Target> for Symbol2EnsId {
    fn from(target: Target) -> Self {
        Self {
            approved_symbol: target.approved_symbol.clone(),
            ens_id: target.id,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Synonym2Symbol {
    pub synonym: String,
    pub approved_symbol: String,
}

impl Target {
    pub fn parse(filepath: &Utf8PathBuf) -> Result<Vec<Self>> {
        debug!("Parsing targets from {:?}", filepath);
        let fileh = std::fs::File::open(filepath)?;
        let reader = BufReader::new(fileh);
        let mut targets = Vec::new();
        for line in reader.lines() {
            targets.push(serde_json::from_str(&line?)?);
        }
        Ok(targets)
    }
}
