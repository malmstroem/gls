use iwf::ctx::Ctx;
use iwf::sql::base::{self, DbBmc, BTREE};
use iwf::sql::ModelManager;
use iwf::sql::Result;
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tabled::Tabled;

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct WkdeTagForCreate {
    pub idx: i32,
    pub kind: String,
    pub label: String,
    pub n_labels: i32,
    pub n_pixels: i32,
}

impl WkdeTagBmc {
    pub async fn bulk_import(mm: &ModelManager, entries: Vec<WkdeTagForCreate>) -> Result<()> {
        let mut tx = mm.db().begin().await?;
        for entry in entries {
            let _result = sqlx::query(&format!(
                "INSERT INTO {} (idx,kind,label,n_labels,n_pixels)
    VALUES ($1,$2,$3,$4,$5)",
                Self::TABLE
            ))
            .bind(entry.idx)
            .bind(entry.kind)
            .bind(entry.label)
            .bind(entry.n_labels)
            .bind(entry.n_pixels)
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
  idx integer not null,
  kind character varying not null,
  label character varying not null,
  n_labels integer not null,
  n_pixels integer not null
);

create index if not exists "IDX_{table}_kind" ON {table} {BTREE} (kind);
create index if not exists "IDX_{table}_label" ON {table} {BTREE} (label);


        "##,
            match drop_table {
                true => format!("drop table if exists {table};"),
                false => String::new(),
            }
        )
    }
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default, Tabled)]
pub struct WkdeTag {
    pub id: i32,
    pub idx: i32,
    pub kind: String,
    pub label: String,
    pub n_labels: i32,
    pub n_pixels: i32,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct WkdeTagForUpdate {
    pub idx: Option<i32>,
    pub kind: Option<String>,
    pub label: Option<String>,
    pub n_labels: Option<i32>,
    pub n_pixels: Option<i32>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct WkdeTagFilter {
    id: Option<OpValsInt64>,
    idx: Option<OpValsInt64>,
    kind: Option<OpValsString>,
    label: Option<OpValsString>,
    n_labels: Option<OpValsInt64>,
    n_pixels: Option<OpValsInt64>,
}

pub struct WkdeTagBmc;

impl DbBmc for WkdeTagBmc {
    const TABLE: &'static str = "wkdetag";
}

impl WkdeTagBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, clone_c: WkdeTagForCreate) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, clone_c).await
    }
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<WkdeTag> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<WkdeTagFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<WkdeTag>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        clone_u: WkdeTagForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, clone_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}
