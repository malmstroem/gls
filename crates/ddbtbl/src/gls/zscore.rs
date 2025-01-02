use iwf::ctx::Ctx;
use iwf::sql::base::{self, DbBmc, BTREE};
use iwf::sql::ModelManager;
use iwf::sql::Result;
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsFloat64, OpValsInt64};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tabled::Tabled;

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct ZscoreForCreate {
    pub ac_id: i32,
    pub ann_id: i32,
    pub qmatrix_id: i32,
    pub zscore: f64,
}

impl ZscoreBmc {
    pub async fn bulk_import(mm: &ModelManager, entries: Vec<ZscoreForCreate>) -> Result<()> {
        let mut tx = mm.db().begin().await?;
        for entry in entries {
            let _result = sqlx::query(&format!(
                "INSERT INTO {} (ac_id,ann_id,qmatrix_id,zscore)
        VALUES ($1, $2, $3, $4)",
                Self::TABLE
            ))
            .bind(entry.ac_id)
            .bind(entry.ann_id)
            .bind(entry.qmatrix_id)
            .bind(entry.zscore)
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
  ann_id integer not null,
  qmatrix_id integer not null,
  zscore float not null
);
create index if not exists "IDX_{table}_ac_id" ON {table} {BTREE} (ac_id);
        "##,
            match drop_table {
                true => format!("drop table if exists {table};"),
                false => String::new(),
            }
        )
    }
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default, Tabled)]
pub struct Zscore {
    pub id: i32,
    pub ac_id: i32,
    pub ann_id: i32,
    pub qmatrix_id: i32,
    pub zscore: f64,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct ZscoreForUpdate {
    pub ac_id: Option<i32>,
    pub ann_id: Option<i32>,
    pub qmatrix_id: Option<i32>,
    pub zscore: Option<f64>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ZscoreFilter {
    id: Option<OpValsInt64>,
    ac_id: Option<OpValsInt64>,
    ann_id: Option<OpValsInt64>,
    qmatrix_id: Option<OpValsInt64>,
    zscore: Option<OpValsFloat64>,
}

pub struct ZscoreBmc;

impl DbBmc for ZscoreBmc {
    const TABLE: &'static str = "zscore";
}

impl ZscoreBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, clone_c: ZscoreForCreate) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, clone_c).await
    }
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Zscore> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<ZscoreFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Zscore>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        clone_u: ZscoreForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, clone_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}
