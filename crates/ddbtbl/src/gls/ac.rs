use camino::Utf8PathBuf;
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
pub struct AcForCreate {
    #[serde(rename = "Entry")]
    pub entry: String,
    #[serde(rename = "Entry Name")]
    pub entry_name: String,
    #[serde(rename = "From")]
    pub frm: String,
}

impl AcBmc {
    pub fn parse(input_path: &Utf8PathBuf) -> Result<Vec<AcForCreate>> {
        let mut ret = vec![];
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_path(input_path)?;
        for line in rdr.deserialize() {
            let record: AcForCreate = line?;
            ret.push(record);
        }
        Ok(ret)
    }
    pub async fn bulk_import(mm: &ModelManager, entries: Vec<AcForCreate>) -> Result<()> {
        let mut tx = mm.db().begin().await?;
        for entry in entries {
            let _result = sqlx::query(&format!(
                "INSERT INTO {} (entry,entry_name,frm)
        VALUES ($1, $2, $3)",
                Self::TABLE
            ))
            .bind(entry.entry)
            .bind(entry.entry_name)
            .bind(entry.frm)
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
  entry character varying not null,
  entry_name character varying not null,
  frm character varying not null,
  description character varying not null default ''
);
create index if not exists "IDX_{table}_frm" ON {table} {BTREE} (frm);
        "##,
            match drop_table {
                true => format!("drop table if exists {table};"),
                false => String::new(),
            }
        )
    }
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default, Tabled)]
pub struct Ac {
    pub id: i32,
    pub entry: String,
    pub entry_name: String,
    pub frm: String,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct AcForUpdate {
    pub entry: Option<String>,
    pub entry_name: Option<String>,
    pub frm: Option<String>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct AcFilter {
    id: Option<OpValsInt64>,
    entry: Option<OpValsString>,
    entry_name: Option<OpValsString>,
    frm: Option<OpValsString>,
}

pub struct AcBmc;

impl DbBmc for AcBmc {
    const TABLE: &'static str = "ac";
}

impl AcBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, clone_c: AcForCreate) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, clone_c).await
    }
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Ac> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<AcFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Ac>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(ctx: &Ctx, mm: &ModelManager, id: i32, clone_u: AcForUpdate) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, clone_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}
