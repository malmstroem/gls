use crate::error::Result;
use iwf::md::model::mpage::StdPage;
use iwf::plot::scatter::{ScatterGlyph, ScatterLO};
use iwf::sql::ModelManager;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tabled::Tabled;

#[derive(Debug, sqlx::FromRow, Default, Serialize, Deserialize, Tabled)]
pub struct Fig3c {
    entry: String,
    grp: String,
    ac_id: i32,
    time_variance: f64,
    patient_variance: f64,
}

pub async fn get(mm: &ModelManager, grp_mpr: &BTreeMap<String, String>) -> Result<StdPage<Fig3c>> {
    let name = String::from("fig3c");
    let stat = "select ac.entry, 'grp' as grp, a.ac_id,a.variance as time_variance,b.variance as patient_variance from variance a inner join variance b on a.ac_id = b.ac_id inner join ac on a.ac_id = ac.id where a.kind = 'time' and b.kind = 'patient'";
    let mut items: Vec<Fig3c> = sqlx::query_as::<_, Fig3c>(stat).fetch_all(&mm.db).await?;
    let mut glyphs = vec![];
    for item in &mut items {
        item.grp = grp_mpr.get(&item.entry).unwrap_or(&String::new()).clone();
        glyphs.push(ScatterGlyph {
            trace: "item".to_string(),
            y: item.time_variance,
            x: item.patient_variance,
            ..Default::default()
        });
    }
    let fig = ScatterLO {
        y_lab: "TimeVariance".into(),
        x_lab: "PatientVariance".into(),
        width: 900,
        height: 900,
        ..Default::default()
    };
    let plot = fig.get_plotly(glyphs, None, &name)?;
    let stdpage = StdPage::<Fig3c> {
        plot_path: Some(format!("{name}.png")),
        figure_text: None,
        plot: Some(plot),
        table: Some(items),
        name,
        ..Default::default()
    };

    Ok(stdpage)
}
