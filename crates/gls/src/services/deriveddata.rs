use iwf::sql::DbCredentials;

use ddbtbl::cluster::cluster::ClusterBmc;
use ddbtbl::cluster::cluster_gsea::GseaEnrBmc;
use ddbtbl::cluster::umap::UmapBmc;
use ddbtbl::gls::glsn::GlsnBmc;
use ddbtbl::gls::labeldata::LabelDataBmc;
use ddbtbl::gls::scoresetting::ScoreSettingBmc;
use ddbtbl::gls::variance::VarianceBmc;
use ddbtbl::gls::wkdelabel::WkdeLabelBmc;
use ddbtbl::gls::wkdetag::WkdeTagBmc;
use ddbtbl::gls::zscore::ZscoreBmc;
use ddbvws::gls::aclabels::AclabelsBmc;
use ddbvws::gls::gls::GlsBmc;
use ddbvws::gls::labelspivot::LabelspivotBmc;

pub async fn create(dbc: &DbCredentials) -> iwf::Result<()> {
    iwf::sql::create_tables(
        dbc,
        vec![
            GlsBmc::get_drop_sql(),
            AclabelsBmc::get_drop_sql(),
            LabelspivotBmc::get_drop_sql(),
            ZscoreBmc::get_create_sql(true),
            UmapBmc::get_create_sql(true),
            ClusterBmc::get_create_sql(true),
            GlsnBmc::get_create_sql(true),
            GseaEnrBmc::get_create_sql(true),
            WkdeLabelBmc::get_create_sql(true),
            WkdeTagBmc::get_create_sql(true),
            VarianceBmc::get_create_sql(true),
            LabelDataBmc::get_create_sql(true),
            ScoreSettingBmc::get_create_sql(true),
            GlsBmc::get_create_sql(),
            AclabelsBmc::get_create_sql(),
            LabelspivotBmc::get_create_sql(),
        ],
    )
    .await?;
    Ok(())
}
