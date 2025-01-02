use crate::error::Result;
use iwf::md::model::mpage::StdPage;
use iwf::plot::dotplot::{DotGlyph, DotLO};
use iwf::sql::ModelManager;
use log::info;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, sqlx::FromRow, Default, Serialize, Deserialize, Tabled)]
pub struct Fig1E {
    kind: String,
    cluster_id: String,
    nlog_pvalue: f64,
    log_cs: f64,
    pvalue: f64,
    term: String,
    genes: String,
}

pub async fn get(mm: &ModelManager) -> Result<StdPage<Fig1E>> {
    let name = String::from("fig1e");

    let stmt = "select kind,cluster_id,-log(pvalue) as nlog_pvalue,log(combined_score)*10 as log_cs,pvalue,term,genes from cluster_gsea where kind = 'wkde_haatlas';";
    let items: Vec<Fig1E> = sqlx::query_as::<_, Fig1E>(stmt).fetch_all(&mm.db).await?;
    info!("N items: {}", items.len());
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
        x_lab: "log_pvalue".into(),
        y_lab: "Tissue".into(),
        width: 900,
        height: 900,
        ..Default::default()
    };
    let plot = fig.get_plotly(glyphs, None, &name)?;
    let stdpage = StdPage::<Fig1E> {
        plot_path: Some(format!("{name}.png")),
        figure_text: None,
        plot: Some(plot),
        table: Some(items),
        name,
        ..Default::default()
    };

    Ok(stdpage)
}
