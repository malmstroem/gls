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
pub struct QmatrixForCreate {
    pub typ: String,
    pub name: String,
    pub idx_column: String,
    pub qmatrixdf_name: String,
}

impl QmatrixBmc {
    pub fn parse(input_path: &Utf8PathBuf) -> Result<Vec<Qmatrix>> {
        let mut ret = vec![];
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_path(input_path)?;
        for line in rdr.deserialize() {
            let record: Qmatrix = line?;
            ret.push(record);
        }
        Ok(ret)
    }
    pub async fn bulk_import(mm: &ModelManager, entries: Vec<Qmatrix>) -> Result<()> {
        let mut tx = mm.db().begin().await?;
        for entry in entries {
            let _result = sqlx::query(&format!(
                "INSERT INTO {} (id,typ,name,idx_column,qmatrixdf_name)

        VALUES ($1, $2, $3, $4, $5)",
                Self::TABLE
            ))
            .bind(entry.id)
            .bind(entry.typ)
            .bind(entry.name)
            .bind(entry.idx_column)
            .bind(entry.qmatrixdf_name)
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
  typ character varying not null,
  name character varying not null,
  idx_column character varying not null,
  qmatrixdf_name character varying not null
);
create index if not exists "IDX_{table}_name" ON {table} {BTREE} (name);
        "##,
            match drop_table {
                true => format!("drop table if exists {table};"),
                false => String::new(),
            }
        )
    }
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default, Tabled)]
pub struct Qmatrix {
    pub id: i32,
    #[serde(rename = "type")]
    pub typ: String,
    pub name: String,
    pub idx_column: String,
    pub qmatrixdf_name: String,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct QmatrixForUpdate {
    pub name: Option<String>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct QmatrixFilter {
    id: Option<OpValsInt64>,
    name: Option<OpValsString>,
}

pub struct QmatrixBmc;

impl DbBmc for QmatrixBmc {
    const TABLE: &'static str = "qmatrix";
}

impl QmatrixBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, clone_c: QmatrixForCreate) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, clone_c).await
    }
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Qmatrix> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<QmatrixFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Qmatrix>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        clone_u: QmatrixForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, clone_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}
