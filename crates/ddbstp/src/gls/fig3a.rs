use crate::error::Result;
use iwf::md::model::mpage::StdPage;
use iwf::plot::boxplot::{BoxGlyph, BoxLO};
use iwf::sql::ModelManager;
#[allow(unused_imports)]
use log::{debug, info};
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, sqlx::FromRow, Default, Serialize, Deserialize, Tabled)]
pub struct Fig3a {
    name: String,
    entry: String,
    global_grp: String,
    scaled_value: f64,
}

pub async fn get(mm: &ModelManager, protein_list: &Vec<String>) -> Result<StdPage<Fig3a>> {
    let name = String::from("fig3a");
    let selstr: Vec<String> = protein_list.iter().map(|e| format!("'{e}'")).collect();
    let stmt = format!("select qmatrix.name,entry,global_grp,scaled_value from qm inner join ac on ac_id = ac.id inner join ann on ann_id = ann.id inner join qmatrix on qmatrix_id= qmatrix.id where entry IN ({}) and global_grp = 'liver' and name in ('emblatlas', 'mspatlas','msratlas','haatlas')", selstr.join(","));
    debug!("Statment {}", stmt);
    let items: Vec<Fig3a> = sqlx::query_as::<_, Fig3a>(&stmt).fetch_all(&mm.db).await?;
    let mut glyphs = vec![];
    for item in &items {
        glyphs.push(BoxGlyph {
            trace: item.name.clone(),
            value: item.scaled_value,
            ..Default::default()
        });
    }
    let fig = BoxLO {
        x_lab: "Atlas".into(),
        y_lab: "Scaled value".into(),
        width: 900,
        height: 900,
        ..Default::default()
    };
    let plot = fig.get_plotly(glyphs, None, &name)?;
    let stdpage = StdPage::<Fig3a> {
        plot_path: Some(format!("{name}.png")),
        figure_text: None,
        plot: Some(plot),
        table: Some(items),
        name,
        ..Default::default()
    };

    Ok(stdpage)
}
