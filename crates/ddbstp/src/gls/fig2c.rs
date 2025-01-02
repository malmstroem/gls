use crate::error::Result;
use iwf::md::model::mpage::StdPage;
use iwf::plot::scatter::{ScatterGlyph, ScatterLO};
use iwf::sql::ModelManager;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, sqlx::FromRow, Default, Serialize, Deserialize, Tabled)]
pub struct Fig2c {
    gls: f64,
    global_grp: String,
    global_label_string: String,
    v: f64,
    n: f64,
}

pub async fn get(mm: &ModelManager) -> Result<StdPage<Fig2c>> {
    let name = String::from("fig2c");
    let stmt = "select gls,global_grp,global_label_string,sum(scaled_value) as v,count(*)::float as n  from (select gls,global_label_string,global_grp,scaled_value from gls inner join qm on gls.ac::int = qm.ac_id inner join qmatrix on qmatrix_id = qmatrix.id inner join ann on ann_id = ann.id where qmatrix.name = 'haatlas' and global_label_string = 'brain') a group by gls,global_grp,global_label_string order by global_grp";
    let items: Vec<Fig2c> = sqlx::query_as::<_, Fig2c>(stmt).fetch_all(&mm.db).await?;
    let mut glyphs = vec![];
    for item in &items {
        glyphs.push(ScatterGlyph {
            trace: item.global_label_string.to_string(),
            x: item.gls,
            y: item.v,
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
    let stdpage = StdPage::<Fig2c> {
        plot_path: Some(format!("{name}.png")),
        figure_text: None,
        plot: Some(plot),
        table: Some(items),
        name,
        ..Default::default()
    };

    Ok(stdpage)
}
