use crate::error::Result;
use iwf::md::model::mpage::StdPage;
use iwf::plot::stacked_bar::{BarGlyph, StackedBarLO};
use iwf::sql::ModelManager;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, sqlx::FromRow, Default, Serialize, Deserialize, Tabled)]
pub struct Fig3b {
    term: String,
    nlog_pvalue: f64,
}

pub async fn get(mm: &ModelManager) -> Result<StdPage<Fig3b>> {
    let name = String::from("fig3b");
    let stmt = "select term,-log(pvalue) as nlog_pvalue from cluster_gsea where kind = 'fig3b' order by nlog_pvalue desc limit 40";

    let items: Vec<Fig3b> = sqlx::query_as::<_, Fig3b>(stmt).fetch_all(&mm.db).await?;
    let mut glyphs = vec![];
    for item in &items {
        glyphs.push(BarGlyph {
            trace: "trace".into(),
            x: item.term.clone(),
            y: item.nlog_pvalue,
            ..Default::default()
        });
    }
    let fig = StackedBarLO {
        x_lab: "Term".into(),
        y_lab: "nlog_pvalue".into(),
        width: 1200,
        height: 900,
        ..Default::default()
    };
    let plot = fig.get_plotly(glyphs, None, &name)?;
    let stdpage = StdPage::<Fig3b> {
        plot_path: Some(format!("{name}.png")),
        figure_text: None,
        plot: Some(plot),
        table: Some(items),
        name,
        ..Default::default()
    };

    Ok(stdpage)
}
