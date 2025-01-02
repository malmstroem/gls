use iwf::ctx::Ctx;
use iwf::sql::base::{self, DbBmc, BTREE};
use iwf::sql::ModelManager;
use iwf::sql::Result;
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsFloat64, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tabled::Tabled;

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct GlsnForCreate {
    pub qmatrix_id: i32,
    pub ac_id: i32,
    pub score: f64,
    pub tissue: String,
}

impl GlsnBmc {
    pub async fn bulk_import(mm: &ModelManager, entries: Vec<GlsnForCreate>) -> Result<()> {
        let mut tx = mm.db().begin().await?;
        for entry in entries {
            let _result = sqlx::query(&format!(
                "INSERT INTO {} (qmatrix_id,ac_id,score,tissue)
    VALUES ($1,$2,$3,$4)",
                Self::TABLE
            ))
            .bind(entry.qmatrix_id)
            .bind(entry.ac_id)
            .bind(entry.score)
            .bind(entry.tissue)
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
  score float not null,
  tissue character varying not null
);
create index if not exists "IDX_{table}_qmatrix_id" ON {table} {BTREE} (qmatrix_id);
create index if not exists "IDX_{table}_ac_id" ON {table} {BTREE} (ac_id);
create index if not exists "IDX_{table}_tissue" ON {table} {BTREE} (tissue);
        "##,
            match drop_table {
                true => format!("drop table if exists {table};"),
                false => String::new(),
            }
        )
    }
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default, Tabled)]
pub struct Glsn {
    pub id: i32,
    pub qmatrix_id: i32,
    pub ac_id: i32,
    pub score: f64,
    pub tissue: String,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct GlsnForUpdate {
    pub qmatrix_id: Option<i32>,
    pub ac_id: Option<i32>,
    pub score: Option<f64>,
    pub tissue: Option<String>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct GlsnFilter {
    id: Option<OpValsInt64>,
    qmatrix_id: Option<OpValsInt64>,
    ac_id: Option<OpValsInt64>,
    score: Option<OpValsFloat64>,
    tissue: Option<OpValsString>,
}

pub struct GlsnBmc;

impl DbBmc for GlsnBmc {
    const TABLE: &'static str = "glsn";
}

impl GlsnBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, clone_c: GlsnForCreate) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, clone_c).await
    }
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Glsn> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<GlsnFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Glsn>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        clone_u: GlsnForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, clone_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}
