use crate::error::Result;
use iwf::md::model::mpage::StdPage;
use iwf::plot::scatter::{ScatterGlyph, ScatterLO};
use iwf::sql::ModelManager;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, sqlx::FromRow, Default, Serialize, Deserialize, Tabled)]
pub struct Fig4g {
    id: i32,
    global_grp: String,
    y1: f64,
    y2: f64,
}

pub async fn get(mm: &ModelManager, kind: &str) -> Result<StdPage<Fig4g>> {
    let name = String::from("fig4g");
    let stmt = format!("select ann.id,global_grp,y1,y2 from umap inner join ann on CAST(REGEXP_REPLACE(umap.idx, '[^0-9]', '', 'g') AS INTEGER) = ann.id where kind = '{kind}';");
    let items: Vec<Fig4g> = sqlx::query_as::<_, Fig4g>(&stmt).fetch_all(&mm.db).await?;
    let mut glyphs = vec![];
    for item in &items {
        glyphs.push(ScatterGlyph {
            trace: item.global_grp.to_string(),
            x: item.y1,
            y: item.y2,
            size: 10,
            ..Default::default()
        });
    }
    let fig = ScatterLO {
        x_lab: "y1".into(),
        y_lab: "y2".into(),
        width: 900,
        height: 900,
        ..Default::default()
    };
    let plot = fig.get_plotly(glyphs, None, &name)?;
    let stdpage = StdPage::<Fig4g> {
        plot_path: Some(format!("{name}.png")),
        figure_text: None,
        plot: Some(plot),
        table: Some(items),
        name,
        ..Default::default()
    };

    Ok(stdpage)
}