use crate::error::Result;
use iwf::md::model::mpage::StdPage;
use iwf::plot::boxplot::{BoxGlyph, BoxLO};
use iwf::sql::ModelManager;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, sqlx::FromRow, Default, Serialize, Deserialize, Tabled)]
pub struct Fig3d {
    ac_id: i32,
    time_variance: f64,
    patient_variance: f64,
}

pub async fn get(mm: &ModelManager) -> Result<StdPage<Fig3d>> {
    let name = String::from("fig3d");
    let stmt = "select a.ac_id,a.variance as time_variance,b.variance as patient_variance from variance a inner join variance b on a.ac_id = b.ac_id where a.kind = 'time' and b.kind = 'patient'";
    let items: Vec<Fig3d> = sqlx::query_as::<_, Fig3d>(stmt).fetch_all(&mm.db).await?;
    let mut glyphs = vec![];
    for item in &items {
        glyphs.push(BoxGlyph {
            trace: "time".into(),
            value: item.time_variance,
            ..Default::default()
        });
        glyphs.push(BoxGlyph {
            trace: "patient".into(),
            value: item.patient_variance,
            ..Default::default()
        });
    }
    let fig = BoxLO {
        x_lab: "Type".into(),
        y_lab: "Variance".into(),
        width: 900,
        height: 900,
        ..Default::default()
    };
    let plot = fig.get_plotly(glyphs, None, &name)?;
    let stdpage = StdPage::<Fig3d> {
        plot_path: Some(format!("{name}.png")),
        figure_text: None,
        plot: Some(plot),
        table: Some(items),
        name,
        ..Default::default()
    };

    Ok(stdpage)
}
