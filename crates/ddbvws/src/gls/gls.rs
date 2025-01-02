use iwf::ctx::Ctx;
use iwf::sql::base::{self, DbBmc};
use iwf::sql::ModelManager;
use iwf::sql::Result;
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tabled::Tabled;

impl GlsBmc {
    #[must_use]
    pub fn get_drop_sql() -> String {
        format!("DROP VIEW IF EXISTS {};", Self::TABLE)
    }
    #[must_use]
    pub fn get_create_sql() -> String {
        let table = Self::TABLE;
        format!(
            r##"
CREATE VIEW {table} AS
      select row_number() OVER () AS id, ac,greatest(cummulative_atlas_score,cummulative_cell_score) as gls, case when cummulative_cell_score >= cummulative_atlas_score then cell_label_string else tissue_label_string end as sel,global_label_string,cell_label_string,tissue_label_string, case when cummulative_cell_score >= cummulative_atlas_score then 'cell' else 'tissue' end as kind from labeldata;

"##
        )
    }
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default, Tabled)]
pub struct Gls {
    pub id: i64,
    pub ac: String,
    pub gls: i32,
    pub sel: String,
    pub global_label_string: String,
    pub cell_label_string: String,
    pub tissue_label_string: String,
    pub kind: String,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct GlsFilter {
    id: Option<OpValsInt64>,
    ac: Option<OpValsString>,
    kind: Option<OpValsString>,
    global_label_string: Option<OpValsString>,
}

pub struct GlsBmc;

impl DbBmc for GlsBmc {
    const TABLE: &'static str = "gls";
}

impl GlsBmc {
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Gls> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<GlsFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Gls>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }
}
