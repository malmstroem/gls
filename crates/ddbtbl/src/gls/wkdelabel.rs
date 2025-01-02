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
pub struct WkdeLabelForCreate {
    pub ac_id: i32,
    pub kind: String,
    pub labels: Vec<i32>,
}

impl WkdeLabelBmc {
    pub async fn bulk_import(mm: &ModelManager, entries: Vec<WkdeLabelForCreate>) -> Result<()> {
        let mut tx = mm.db().begin().await?;
        for entry in entries {
            let _result = sqlx::query(&format!(
                "INSERT INTO {} (ac_id,kind,labels)
    VALUES ($1,$2,$3)",
                Self::TABLE
            ))
            .bind(entry.ac_id)
            .bind(entry.kind)
            .bind(entry.labels)
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
  ac_id integer not null,
  kind character varying not null,
  labels integer[] not null
);
create index if not exists "IDX_{table}_ac_id" ON {table} {BTREE} (ac_id);
create index if not exists "IDX_{table}_kind" ON {table} {BTREE} (kind);

        "##,
            match drop_table {
                true => format!("drop table if exists {table};"),
                false => String::new(),
            }
        )
    }
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default, Tabled)]
pub struct WkdeLabel {
    pub id: i32,
    pub ac_id: i32,
    pub kind: String,
    pub labels: String,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct WkdeLabelForUpdate {
    pub ac_id: Option<i32>,
    pub kind: Option<String>,
    pub labels: Option<String>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct WkdeLabelFilter {
    id: Option<OpValsInt64>,
    ac_id: Option<OpValsInt64>,
    kind: Option<OpValsString>,
    labels: Option<OpValsString>,
}

pub struct WkdeLabelBmc;

impl DbBmc for WkdeLabelBmc {
    const TABLE: &'static str = "wkdelabel";
}

impl WkdeLabelBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, clone_c: WkdeLabelForCreate) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, clone_c).await
    }
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<WkdeLabel> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<WkdeLabelFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<WkdeLabel>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        clone_u: WkdeLabelForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, clone_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}
