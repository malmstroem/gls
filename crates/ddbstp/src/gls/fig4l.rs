use crate::error::Result;
use iwf::md::model::mpage::StdPage;
use iwf::plot::stacked_bar::{BarGlyph, StackedBarLO};
use iwf::sql::ModelManager;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, sqlx::FromRow, Default, Serialize, Deserialize, Tabled)]
pub struct Fig4l {
    atlases: String,
    entry: String,
    global_grp: String,
    mean_norm_value: f64,
}

pub async fn get(mm: &ModelManager) -> Result<StdPage<Fig4l>> {
    let name = String::from("fig4l");
    let stmt = "select string_agg(qmatrix.name, ',' order by qmatrix.name) as atlases,entry,ann.global_grp,avg(scaled_value) as mean_norm_value from qm inne join qmatrix on qmatrix_id = qmatrix.id inner join ac on ac_id = ac.id inner join ann on ann_id = ann.id  where entry in ('P59665', 'P80188', 'P26022', 'P08311', 'P27930', 'Q9HD89', 'P80511', 'O75594', 'P24158') and measured = true and qmatrix.name in ('haatlas', 'emblatlas', 'mspatlas', 'msratlas') group by entry,ann.global_grp order by global_grp,entry;";
    let items: Vec<Fig4l> = sqlx::query_as::<_, Fig4l>(stmt).fetch_all(&mm.db).await?;
    let mut glyphs = vec![];
    for item in &items {
        glyphs.push(BarGlyph {
            trace: item.global_grp.clone(),
            x: format!("{}:{}", item.global_grp, item.entry),
            y: item.mean_norm_value,
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
    let stdpage = StdPage::<Fig4l> {
        plot_path: Some(format!("{name}.png")),
        figure_text: None,
        plot: Some(plot),
        table: Some(items),
        name,
        ..Default::default()
    };

    Ok(stdpage)
}
