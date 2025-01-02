use camino::Utf8PathBuf;
use ddbot;
use iwf::ctx::Ctx;
use iwf::sql::base::{self, DbBmc, BTREE};
use iwf::sql::ModelManager;
use iwf::sql::{Error, Result};
use log::warn;
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tabled::Tabled;

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct TargetForCreate {
    pub ensg: String,
    pub symbol: String,
    pub name: String,
    pub biotype: String,
}

impl TargetBmc {
    pub async fn parse(mm: &ModelManager, target_bincode_path: &Utf8PathBuf) -> Result<()> {
        let mut tx = mm.db().begin().await?;
        let targets = ddbot::bincode2targets(target_bincode_path)
            .map_err(|_| Error::Specified("Cannot parse".to_string()))?;
        warn!("N targets parsed: {}", targets.len());
        for (_name, target) in targets {
            let c = TargetForCreate {
                ensg: target.id,
                symbol: target.approved_symbol,
                name: target.approved_name,
                biotype: target.biotype,
            };
            let _result = sqlx::query(&format!(
                "INSERT INTO {} (ensg,symbol,name,biotype)
        VALUES ($1, $2, $3, $4)",
                Self::TABLE
            ))
            .bind(c.ensg)
            .bind(c.symbol)
            .bind(c.name)
            .bind(c.biotype)
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
  ensg character varying not null,
  symbol character varying not null,
  name character varying not null,
  biotype character varying not null
);
create index if not exists "IDX_{table}_ensg" ON {table} {BTREE} (ensg);
        "##,
            match drop_table {
                true => format!("drop table if exists {table};"),
                false => String::new(),
            }
        )
    }
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default, Tabled)]
pub struct Target {
    pub id: i32,
    pub ensg: String,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct TargetForUpdate {
    pub ensg: Option<String>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct TargetFilter {
    id: Option<OpValsInt64>,
    ensg: Option<OpValsString>,
}

pub struct TargetBmc;

impl DbBmc for TargetBmc {
    const TABLE: &'static str = "target";
}

impl TargetBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, clone_c: TargetForCreate) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, clone_c).await
    }
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Target> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<TargetFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Target>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        clone_u: TargetForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, clone_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}
