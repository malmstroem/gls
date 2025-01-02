use crate::error::Result;
use iwf::md::model::mpage::StdPage;
use iwf::plot::stacked_bar::{BarGlyph, StackedBarLO};
use iwf::sql::ModelManager;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, sqlx::FromRow, Default, Serialize, Deserialize, Tabled)]
pub struct Fig4d {
    entry_name: String,
    scaled_value: f64,
    global_grp: String,
    sample_grp: String,
}

pub async fn get(mm: &ModelManager) -> Result<StdPage<Fig4d>> {
    let name = String::from("fig4d");
    let stmt = "select entry_name,scaled_value,global_grp,sample_grp from qm inner join qmatrix on qmatrix_id = qmatrix.id inner join ac on qm.ac_id = ac.id inner join ann on ann_id = ann.id where qmatrix.name = 'plpancr' and entry_name in ('CBPA1_HUMAN', 'CEL2A_HUMAN', 'CEL3A_HUMAN', 'CBPB1_HUMAN', 'CTRB2_HUMAN', 'GP2_HUMAN', 'REG1B_HUMAN', 'LIPP_HUMAN', 'AMYP_HUMAN') and measured = 'true';";
    let items: Vec<Fig4d> = sqlx::query_as::<_, Fig4d>(stmt).fetch_all(&mm.db).await?;
    let mut glyphs = vec![];
    for item in &items {
        glyphs.push(BarGlyph {
            x: format!("{}:{}", item.global_grp, item.sample_grp),
            trace: item.entry_name.clone(),
            y: item.scaled_value,
            ..Default::default()
        });
    }
    let fig = StackedBarLO {
        x_lab: "Gene".into(),
        y_lab: "Delta".into(),
        width: 900,
        height: 900,
        ..Default::default()
    };
    let plot = fig.get_plotly(glyphs, None, &name)?;
    let stdpage = StdPage::<Fig4d> {
        plot_path: Some(format!("{name}.png")),
        figure_text: None,
        plot: Some(plot),
        table: Some(items),
        name,
        ..Default::default()
    };

    Ok(stdpage)
}
