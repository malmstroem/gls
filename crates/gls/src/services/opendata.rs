use camino::Utf8PathBuf;
use iwf::sql::{DbCredentials, ModelManager};

use ddbtbl::gls::ac::AcBmc;
use ddbtbl::opentarget::target::TargetBmc;

pub async fn create(mm: &ModelManager, dbc: &DbCredentials) -> iwf::Result<()> {
    iwf::sql::create_tables(
        dbc,
        vec![AcBmc::get_create_sql(true), TargetBmc::get_create_sql(true)],
    )
    .await?;

    AcBmc::bulk_import(mm, AcBmc::parse(&Utf8PathBuf::from("filtered.tsv"))?).await?;
    TargetBmc::parse(mm, &Utf8PathBuf::from("target.bincode")).await?;
    Ok(())
}
