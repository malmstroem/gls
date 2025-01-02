use iwf::ctx::Ctx;
use iwf::sql::base::{self, DbBmc, BTREE};
use iwf::sql::ModelManager;
use iwf::sql::Result;
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tabled::Tabled;

#[derive(Fields, Serialize, Deserialize, Clone, Debug)]
pub struct UmapForCreate {
    pub kind: String,
    pub idx: String,
    pub cluster_id: i32,
    pub y1: f64,
    pub y2: f64,
}

impl UmapBmc {
    #[must_use]
    pub fn get_create_sql(drop_table: bool) -> String {
        let table = Self::TABLE;
        format!(
            r##"{}
create table if not exists {table} (
  id serial primary key,
  kind character varying not null,
  idx character varying not null,
  cluster_id integer not null,
  y1 float not null,
  y2 float not null
);
create index if not exists "IDX_{table}_cluster_id" ON {table} {BTREE} (cluster_id);
create index if not exists "IDX_{table}_idx" ON {table} {BTREE} (idx);
        "##,
            match drop_table {
                true => format!("drop table if exists {table};"),
                false => String::new(),
            }
        )
    }
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default, Tabled)]
pub struct Umap {
    pub id: i32,
    pub kind: String,
    pub idx: String,
    pub cluster_id: i32,
    pub y1: f64,
    pub y2: f64,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct UmapForUpdate {
    pub key: Option<String>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct UmapFilter {
    id: Option<OpValsInt64>,
    cluster_id: Option<OpValsInt64>,
    idx: Option<OpValsString>,
}

pub struct UmapBmc;

impl DbBmc for UmapBmc {
    const TABLE: &'static str = "umap";
}

impl UmapBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, clone_c: UmapForCreate) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, clone_c).await
    }
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Umap> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<UmapFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Umap>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        clone_u: UmapForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, clone_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}
