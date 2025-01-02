use crate::error::Result;
use iwf::md::model::mpage::StdPage;
use iwf::plot::scatter::{ScatterGlyph, ScatterLO};
use iwf::sql::ModelManager;
use log::info;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, sqlx::FromRow, Default, Serialize, Deserialize, Tabled)]
pub struct Fig1D {
    name: String,
    y1: f64,
    y2: f64,
}

pub async fn get(mm: &ModelManager) -> Result<StdPage<Fig1D>> {
    let stmt = "select labels as name,y1,y2 from umap inner join ac_labels on idx::int4 = ac_labels.ac_id and umap.kind = ac_labels.kind where umap.kind = 'haatlas';";
    let name = String::from("fig1d");
    let items: Vec<Fig1D> = sqlx::query_as::<_, Fig1D>(stmt).fetch_all(&mm.db).await?;

    info!("N items: {}", items.len());
    let mut glyphs = vec![];
    for item in &items {
        let trace = if item.name.contains(',') {
            String::from("multi")
        } else {
            item.name.clone()
        };
        glyphs.push(ScatterGlyph {
            trace,
            x: item.y1,
            y: item.y2,
            ..Default::default()
        });
    }
    let fig = ScatterLO {
        x_lab: "Gene".into(),
        y_lab: "Delta".into(),
        width: 900,
        height: 900,
        ..Default::default()
    };
    let plot = fig.get_plotly(glyphs, None, &name)?;
    let stdpage = StdPage::<Fig1D> {
        plot_path: Some(format!("{name}.png")),
        figure_text: None,
        plot: Some(plot),
        table: Some(items),
        name,
        ..Default::default()
    };

    Ok(stdpage)
}
