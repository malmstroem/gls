use crate::error::Result;
use iwf::md::model::mpage::StdPage;
use iwf::plot::dotplot::{DotGlyph, DotLO};
use iwf::sql::ModelManager;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, sqlx::FromRow, Default, Serialize, Deserialize, Tabled)]
pub struct Fig2e {
    term: String,
    overlap: String,
    genes: String,
    cluster_id: String,
    nlog_pvalue: f64,
    log_cs: f64,
}

pub async fn get(mm: &ModelManager, filter: &str) -> Result<StdPage<Fig2e>> {
    let name = String::from("fig2e");
    let stmt = format!("select term,overlap,genes,kind as cluster_id,-log(pvalue) as nlog_pvalue,log(combined_score)*10 as log_cs from cluster_gsea where kind like '{filter}%'");
    let items: Vec<Fig2e> = sqlx::query_as::<_, Fig2e>(&stmt).fetch_all(&mm.db).await?;
    let mut glyphs = vec![];
    for item in &items {
        glyphs.push(DotGlyph {
            trace: item.cluster_id.clone(),
            x: item.nlog_pvalue,
            y: item.cluster_id.clone(),
            size: item.log_cs as usize,
            ..Default::default()
        });
    }
    let fig = DotLO {
        x_lab: "Gene".into(),
        y_lab: "Delta".into(),
        width: 900,
        height: 900,
        ..Default::default()
    };
    let plot = fig.get_plotly(glyphs, None, &name)?;
    let stdpage = StdPage::<Fig2e> {
        plot_path: Some(format!("{name}.png")),
        figure_text: None,
        plot: Some(plot),
        table: Some(items),
        name,
        ..Default::default()
    };

    Ok(stdpage)
}
