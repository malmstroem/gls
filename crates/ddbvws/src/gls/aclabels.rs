use iwf::ctx::Ctx;
use iwf::sql::base::{self, DbBmc};
use iwf::sql::ModelManager;
use iwf::sql::Result;
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tabled::Tabled;

impl AclabelsBmc {
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
      select row_number() OVER () AS id, ac_id,kind,string_agg(global_grp, ',' order by global_grp) as labels from wkdelabel inner join ann on ann.id = any(labels) group by ac_id,kind;
"##
        )
    }
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default, Tabled)]
pub struct Aclabels {
    pub id: i64,
    pub kind: String,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct AclabelsFilter {
    id: Option<OpValsInt64>,
    kind: Option<OpValsString>,
}

pub struct AclabelsBmc;

impl DbBmc for AclabelsBmc {
    const TABLE: &'static str = "ac_labels";
}

impl AclabelsBmc {
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Aclabels> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<AclabelsFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Aclabels>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }
}
