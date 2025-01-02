use iwf::ctx::Ctx;
use iwf::sql::base::{self, DbBmc};
use iwf::sql::ModelManager;
use iwf::sql::Result;
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsFloat64, OpValsInt64};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt;
use tabled::Tabled;

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct ScoreSettingForCreate {
    pub cell_atlas_weight: f64,
    pub common_threashold: i32,
    pub delta_to_include: f64,
    pub max_labels: i32,
    pub multi_label_weight: f64,
}

impl Default for ScoreSettingForCreate {
    fn default() -> Self {
        Self {
            multi_label_weight: 0.9,
            delta_to_include: 0.90,
            max_labels: 3,
            common_threashold: 2,
            cell_atlas_weight: 1.0,
        }
    }
}

impl fmt::Display for ScoreSettingForCreate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "rust_mlw_{}_d_{}_maxl_{}_cth_{}_caw_{}",
            self.multi_label_weight,
            self.delta_to_include,
            self.max_labels,
            self.common_threashold,
            self.cell_atlas_weight
        )
    }
}

impl ScoreSettingBmc {
    #[must_use]
    pub fn get_create_sql(drop_table: bool) -> String {
        let table = Self::TABLE;
        format!(
            r##"{}
create table if not exists {table} (
  id serial primary key,
  cell_atlas_weight float not null,
  common_threashold integer not null,
  delta_to_include float not null,
  max_labels integer not null,
  multi_label_weight float not null
);





        "##,
            match drop_table {
                true => format!("drop table if exists {table};"),
                false => String::new(),
            }
        )
    }
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default, Tabled)]
pub struct ScoreSetting {
    pub id: i32,
    pub cell_atlas_weight: f64,
    pub common_threashold: i32,
    pub delta_to_include: f64,
    pub max_labels: i32,
    pub multi_label_weight: f64,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct ScoreSettingForUpdate {
    pub cell_atlas_weight: Option<f64>,
    pub common_threashold: Option<i32>,
    pub delta_to_include: Option<f64>,
    pub max_labels: Option<i32>,
    pub multi_label_weight: Option<f64>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ScoreSettingFilter {
    id: Option<OpValsInt64>,
    cell_atlas_weight: Option<OpValsFloat64>,
    common_threashold: Option<OpValsInt64>,
    delta_to_include: Option<OpValsFloat64>,
    max_labels: Option<OpValsInt64>,
    multi_label_weight: Option<OpValsFloat64>,
}

pub struct ScoreSettingBmc;

impl DbBmc for ScoreSettingBmc {
    const TABLE: &'static str = "scoresetting";
}

impl ScoreSettingBmc {
    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        clone_c: ScoreSettingForCreate,
    ) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, clone_c).await
    }
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<ScoreSetting> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<ScoreSettingFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<ScoreSetting>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        clone_u: ScoreSettingForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, clone_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}
