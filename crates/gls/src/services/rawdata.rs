use camino::Utf8PathBuf;
use ddbtbl::gls::ann::AnnBmc;
use ddbtbl::gls::qm::QmBmc;
use ddbtbl::gls::qmatrix::QmatrixBmc;
use iwf::ctx::Ctx;
use iwf::sql::{DbCredentials, ModelManager};
use log::debug;

pub async fn create(
    ctx: &Ctx,
    mm: &ModelManager,
    dbc: &DbCredentials,
    qmi: &Vec<crate::Qmi>,
) -> iwf::Result<()> {
    iwf::sql::create_tables(
        dbc,
        vec![
            QmBmc::get_create_sql(true),
            QmatrixBmc::get_create_sql(true),
            AnnBmc::get_create_sql(true),
        ],
    )
    .await?;
    sqlx::query("insert into ann values (-1, 'common', 'common' ,'common', 'common', 'common', 'common', 'common') on conflict (id) do nothing;").execute(&mm.db).await?;
    sqlx::query("insert into ann values (-2, 'none', 'none' ,'none', 'none', 'none', 'none', 'none') on conflict (id) do nothing;").execute(&mm.db).await?;
    debug!("INSERTED common and None");
    AnnBmc::bulk_import(mm, AnnBmc::parse(&Utf8PathBuf::from("measure_v2.tsv"))?).await?;
    qm(ctx, mm, qmi).await?;
    ddbtbl::gls::qm::create_haatlas_qmatrix(ctx, mm).await?;
    Ok(())
}

async fn qm(ctx: &Ctx, mm: &ModelManager, input_filenames: &Vec<crate::Qmi>) -> iwf::Result<()> {
    for qmi in input_filenames {
        QmBmc::bulk_import(
            mm,
            QmBmc::parse(ctx, mm, &qmi.filepath, qmi.name.clone(), qmi.ex, qmi.impute).await?,
        )
        .await?;
    }
    Ok(())
}
