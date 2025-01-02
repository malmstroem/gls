use crate::error::Result;
use iwf::md::model::mpage::StdPage;
use iwf::plot::stacked_bar::{BarGlyph, StackedBarLO};
use iwf::sql::ModelManager;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, sqlx::FromRow, Default, Serialize, Deserialize, Tabled)]
pub struct Fig2f {
    global_label_string: String,
    n: i64,
}

pub async fn get(mm: &ModelManager) -> Result<StdPage<Fig2f>> {
    let name = String::from("fig2f");
    let stmt =
    "select global_label_string,count(*) as n from labeldata where global_label_string = tissue_label_string group by global_label_string having count(*) > 10 order by count(*) desc;";
    let items: Vec<Fig2f> = sqlx::query_as::<_, Fig2f>(stmt).fetch_all(&mm.db).await?;
    let mut glyphs = vec![];
    for item in &items {
        glyphs.push(BarGlyph {
            trace: "data".into(),
            x: item.global_label_string.clone(),
            y: item.n as f64,
            ..Default::default()
        });
    }
    let fig = StackedBarLO {
        x_lab: "Label".into(),
        y_lab: "Count".into(),
        width: 2400,
        height: 900,
        ..Default::default()
    };
    let plot = fig.get_plotly(glyphs, None, &name)?;
    let stdpage = StdPage::<Fig2f> {
        plot_path: Some(format!("{name}.png")),
        figure_text: None,
        plot: Some(plot),
        table: Some(items),
        name,
        ..Default::default()
    };

    Ok(stdpage)
}
