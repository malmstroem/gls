use iwf::ctx::Ctx;
use iwf::sql::base::{self, DbBmc, BTREE};
use iwf::sql::ModelManager;
use iwf::sql::Result;
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsFloat64, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tabled::Tabled;

#[derive(Fields, Serialize, Deserialize, Clone, Debug)]
pub struct GseaEnrForCreate {
    pub kind: String,
    pub gene_set: String,
    pub term: String,
    pub overlap: String,
    pub pvalue: f64,
    pub adjusted_pvalue: f64,
    pub odds_ratio: f64,
    pub combined_score: f64,
    pub genes: String,
    pub cluster_id: String,
    pub n_query_genes: i32,
}

impl GseaEnrBmc {
    pub async fn bulk_import(mm: &ModelManager, entries: Vec<GseaEnrForCreate>) -> Result<()> {
        let mut tx = mm.db().begin().await?;
        for entry in entries {
            let _result = sqlx::query(&format!(
            "INSERT INTO {} (kind,adjusted_pvalue,cluster_id,combined_score,gene_set,genes,n_query_genes,odds_ratio,overlap,pvalue,term)
    VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)",
            Self::TABLE
        ))
        .bind(entry.kind)
        .bind(entry.adjusted_pvalue)
        .bind(entry.cluster_id)
        .bind(entry.combined_score)
        .bind(entry.gene_set)
        .bind(entry.genes)
        .bind(entry.n_query_genes)
        .bind(entry.odds_ratio)
        .bind(entry.overlap)
        .bind(entry.pvalue)
        .bind(entry.term)
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
  kind character varying not null,
  gene_set character varying not null,
  term character varying not null,
  overlap character varying not null,
  pvalue float not null,
  adjusted_pvalue float not null,
  odds_ratio float not null,
  combined_score float not null,
  genes character varying not null,
  cluster_id character varying not null,
  n_query_genes integer not null
);
create index if not exists "IDX_{table}_kind" ON {table} {BTREE} (kind);
create index if not exists "IDX_{table}_cluster_id" ON {table} {BTREE} (cluster_id);
create index if not exists "IDX_{table}_term" ON {table} {BTREE} (term);
create index if not exists "IDX_{table}_pvalue" ON {table} {BTREE} (pvalue);
        "##,
            match drop_table {
                true => format!("drop table if exists {table};"),
                false => String::new(),
            }
        )
    }
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default, Tabled)]
pub struct GseaEnr {
    pub id: i32,
    pub gene_set: String,
    pub term: String,
    pub overlap: String,
    pub pvalue: f64,
    pub adjusted_pvalue: f64,
    pub odds_ratio: f64,
    pub combined_score: f64,
    pub genes: String,
    pub cluster_id: String,
    pub n_query_genes: i32,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct GseaEnrForUpdate {
    pub key: Option<String>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct GseaEnrFilter {
    id: Option<OpValsInt64>,
    cluster_id: Option<OpValsString>,
    term: Option<OpValsString>,
    pvalue: Option<OpValsFloat64>,
}

pub struct GseaEnrBmc;

impl DbBmc for GseaEnrBmc {
    const TABLE: &'static str = "cluster_gsea";
}

impl GseaEnrBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, clone_c: GseaEnrForCreate) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, clone_c).await
    }
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<GseaEnr> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<GseaEnrFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<GseaEnr>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        clone_u: GseaEnrForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, clone_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}
