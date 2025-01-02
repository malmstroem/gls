use crate::error::Result;
use iwf::md::model::mpage::StdPage;
use iwf::plot::boxplot::{BoxGlyph, BoxLO};
use iwf::sql::ModelManager;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, sqlx::FromRow, Default, Serialize, Deserialize, Tabled)]
pub struct Fig4o {
    entry_name: String,
    global_grp: String,
    norm_value: f64,
}

pub async fn get(mm: &ModelManager) -> Result<StdPage<Fig4o>> {
    let name = String::from("fig4o");
    let items: Vec<Fig4o> = sqlx::query_as::<_, Fig4o>("select entry_name,norm_value,global_grp from qm inner join qmatrix on qmatrix_id = qmatrix.id inner join ac on ac_id = ac.id inner join ann on ann_id = ann.id where qmatrix.name = 'plsepsis' and entry_name in ('CRHBP_HUMAN', 'CRP_HUMAN', 'HLAC_HUMAN');")
        .fetch_all(&mm.db)
        .await?;
    let mut glyphs = vec![];
    for item in &items {
        glyphs.push(BoxGlyph {
            trace: format!("{}:{}", item.entry_name, item.global_grp),
            value: item.norm_value,
            ..Default::default()
        });
    }
    let fig = BoxLO {
        x_lab: "Protein and Group".into(),
        y_lab: "Abundance".into(),
        width: 900,
        height: 900,
        ..Default::default()
    };
    let plot = fig.get_plotly(glyphs, None, &name)?;
    let stdpage = StdPage::<Fig4o> {
        plot_path: Some(format!("{name}.png")),
        figure_text: None,
        plot: Some(plot),
        table: Some(items),
        name,
        ..Default::default()
    };

    Ok(stdpage)
}