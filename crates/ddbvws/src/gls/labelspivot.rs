use iwf::ctx::Ctx;
use iwf::sql::base::{self, DbBmc};
use iwf::sql::ModelManager;
use iwf::sql::Result;
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tabled::Tabled;

impl LabelspivotBmc {
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
      select row_number() OVER () AS id, 
    ac_id,
    MAX(CASE WHEN kind = 'haatlas' THEN labels ELSE '' END) AS haatlas_clusters,
    MAX(CASE WHEN kind = 'emblatlas' THEN labels ELSE '' END) AS emblatlas_clusters,
    MAX(CASE WHEN kind = 'msratlas' THEN labels ELSE '' END) AS msratlas_clusters,
    MAX(CASE WHEN kind = 'mspatlas' THEN labels ELSE '' END) AS mspatlas_clusters,
    MAX(CASE WHEN kind = 'hacells' THEN labels ELSE '' END) AS hacells_clusters,
    MAX(CASE WHEN kind = 'emblcells' THEN labels ELSE '' END) AS emblcells_clusters
FROM ac_labels
GROUP BY ac_id
ORDER BY ac_id;
"##
        )
    }
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default, Tabled)]
pub struct Labelspivot {
    pub id: i64,
    pub haatlas_clusters: String,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct LabelspivotFilter {
    id: Option<OpValsInt64>,
    haatlas_clusters: Option<OpValsString>,
}

pub struct LabelspivotBmc;

impl DbBmc for LabelspivotBmc {
    const TABLE: &'static str = "labels_pivot";
}

impl LabelspivotBmc {
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Labelspivot> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<LabelspivotFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Labelspivot>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }
}
