use crate::error::Result;
use iwf::md::model::mpage::StdPage;
use iwf::plot::boxplot::{BoxGlyph, BoxLO};
use iwf::sql::ModelManager;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, sqlx::FromRow, Default, Serialize, Deserialize, Tabled)]
pub struct Fig2g {
    entry: String,
    global_label_string: String,
    global_grp: String,
    n2: i64,
    value: f64,
}

pub async fn get(mm: &ModelManager) -> Result<StdPage<Fig2g>> {
    let name = String::from("fig2g");
    let stmt = "select entry,global_label_string,global_grp,count(distinct ann.id) as n2,avg(scaled_value) as value from labeldata inner join qm on ac::int = qm.ac_id inner join ac on qm.ac_id = ac.id inner join qmatrix on qmatrix_id = qmatrix.id inner join ann on ann_id = ann.id where n_tissue = 1 and n_measure > 1 and global_label_string = tissue_label_string and qmatrix.name in ('haatlas', 'emblatlas', 'msratlas', 'mspatlas') and global_label_string = global_grp group by ac.entry,global_label_string,global_grp;";
    let items: Vec<Fig2g> = sqlx::query_as::<_, Fig2g>(stmt).fetch_all(&mm.db).await?;
    let mut glyphs = vec![];
    for item in &items {
        glyphs.push(BoxGlyph {
            trace: format!("{}:{}", item.n2, item.global_grp),
            value: item.value,
            ..Default::default()
        });
    }
    let fig = BoxLO {
        x_lab: "Tissue".into(),
        y_lab: "value".into(),
        width: 1800,
        height: 900,
        ..Default::default()
    };
    let plot = fig.get_plotly(glyphs, None, &name)?;
    let stdpage = StdPage::<Fig2g> {
        plot_path: Some(format!("{name}.png")),
        figure_text: None,
        plot: Some(plot),
        table: Some(items),
        name,
        ..Default::default()
    };

    Ok(stdpage)
}
