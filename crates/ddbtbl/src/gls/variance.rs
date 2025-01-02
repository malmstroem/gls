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
pub struct VarianceForCreate {
    pub ac_id: i32,
    pub kind: String,
    pub mean: f64,
    pub variance: f64,
}

impl VarianceBmc {
    pub async fn bulk_import(mm: &ModelManager, entries: Vec<VarianceForCreate>) -> Result<()> {
        let mut tx = mm.db().begin().await?;
        for entry in entries {
            let _result = sqlx::query(&format!(
                "INSERT INTO {} (ac_id,kind,mean,variance)
    VALUES ($1,$2,$3,$4)",
                Self::TABLE
            ))
            .bind(entry.ac_id)
            .bind(entry.kind)
            .bind(entry.mean)
            .bind(entry.variance)
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
  mean float not null,
  variance float not null
);

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
pub struct Variance {
    pub id: i32,
    pub ac_id: i32,
    pub kind: String,
    pub mean: f64,
    pub variance: f64,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct VarianceForUpdate {
    pub ac_id: Option<i32>,
    pub kind: Option<String>,
    pub mean: Option<f64>,
    pub variance: Option<f64>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct VarianceFilter {
    id: Option<OpValsInt64>,
    ac_id: Option<OpValsInt64>,
    kind: Option<OpValsString>,
    mean: Option<OpValsFloat64>,
    variance: Option<OpValsFloat64>,
}

pub struct VarianceBmc;

impl DbBmc for VarianceBmc {
    const TABLE: &'static str = "variance";
}

impl VarianceBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, clone_c: VarianceForCreate) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, clone_c).await
    }
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Variance> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<VarianceFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Variance>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        clone_u: VarianceForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, clone_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}
