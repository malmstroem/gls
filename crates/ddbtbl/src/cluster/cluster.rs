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
pub struct ClusterForCreate {
    pub label: String,
}

impl ClusterBmc {
    #[must_use]
    pub fn get_create_sql(drop_table: bool) -> String {
        let table = Self::TABLE;
        format!(
            r##"{}
create table if not exists {table} (
  id serial primary key,
  label character varying not null
);
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
pub struct Cluster {
    pub id: i32,
    pub label: String,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct ClusterForUpdate {
    pub label: Option<String>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ClusterFilter {
    id: Option<OpValsInt64>,
    label: Option<OpValsString>,
}

pub struct ClusterBmc;

impl DbBmc for ClusterBmc {
    const TABLE: &'static str = "cluster";
}

impl ClusterBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, clone_c: ClusterForCreate) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, clone_c).await
    }
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Cluster> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<ClusterFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Cluster>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        clone_u: ClusterForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, clone_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}
