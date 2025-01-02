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
pub struct AnnForCreate {
    pub measurement: String,
    pub display_name: String,
    pub permid: String,
    pub global_grp: String,
    pub sample_grp: String,
    pub sample_type: String,
    pub qmatrix_type: String,
}

impl AnnBmc {
    pub fn parse(input_path: &Utf8PathBuf) -> Result<Vec<AnnForCreate>> {
        let mut ret = vec![];
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_path(input_path)?;
        for line in rdr.deserialize() {
            let record: AnnForCreate = line?;
            ret.push(record);
        }
        Ok(ret)
    }
    pub async fn bulk_import(mm: &ModelManager, entries: Vec<AnnForCreate>) -> Result<()> {
        let mut tx = mm.db().begin().await?;
        for entry in entries {
            let _result = sqlx::query(&format!(
                "INSERT INTO {} (measurement,display_name,permid,global_grp,sample_grp,sample_type,qmatrix_type)

        VALUES ($1, $2, $3, $4, $5, $6, $7)",
                Self::TABLE
            ))
            .bind(entry.measurement)
            .bind(entry.display_name)
            .bind(entry.permid)
            .bind(entry.global_grp)
            .bind(entry.sample_grp)
            .bind(entry.sample_type)
            .bind(entry.qmatrix_type)
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
  measurement character varying not null,
  display_name character varying not null,
  permid character varying not null,
  global_grp character varying not null,
  sample_grp character varying not null,
  sample_type character varying not null,
  qmatrix_type character varying not null
);
create index if not exists "IDX_{table}_measurement" ON {table} {BTREE} (measurement);
create index if not exists "IDX_{table}_global_grp" ON {table} {BTREE} (global_grp);
        "##,
            match drop_table {
                true => format!("drop table if exists {table};"),
                false => String::new(),
            }
        )
    }
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default, Tabled)]
pub struct Ann {
    pub id: i32,
    pub measurement: String,
    pub display_name: String,
    pub permid: String,
    pub global_grp: String,
    pub sample_grp: String,
    pub sample_type: String,
    pub qmatrix_type: String,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct AnnForUpdate {
    pub measurement: Option<String>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct AnnFilter {
    id: Option<OpValsInt64>,
    measurement: Option<OpValsString>,
    qmatrix_type: Option<OpValsString>,
}

pub struct AnnBmc;

impl DbBmc for AnnBmc {
    const TABLE: &'static str = "ann";
}

impl AnnBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, clone_c: AnnForCreate) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, clone_c).await
    }
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Ann> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<AnnFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Ann>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        clone_u: AnnForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, clone_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}
