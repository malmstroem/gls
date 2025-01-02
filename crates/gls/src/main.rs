//! gls

mod services;

use camino::Utf8PathBuf;
use clap::Parser;
#[cfg(feature = "rebuild")]
use ddbtsk::gls::ExeClusteringTask;
use iwf::ctx::Ctx;
use iwf::md::{MBook, StdPage};
use iwf::osobject::{download_named_resources, OsObject};
use iwf::sql::ModelManager;
use iwf::{wfopts, DefaultSettings, IwfExe, IwfRun, IwfSettings, IwfSetup, IwfTuning};
use iwfmacros::IwfWf;
#[allow(unused_imports)]
use log::warn;
use ordered_float::OrderedFloat;
use rust_xlsxwriter::Workbook;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

#[cfg(feature = "rebuild")]
use ddbtsk::gls::{
    CalculateEnrichmentFig2eTask, CalculateEnrichmentListTask, CalculateLabelDataTask,
    CalculateVarianceTask, CalculateWkdeEnrichmentTask, ExeWkdeTask, Fig4fUmapTask, Fig4gUmapTask,
    MiPatientUmapTask, SepsisPatientUmapTask,
};

const APP_INFO: &str = concat!(env!("CARGO_BIN_NAME"), "_", env!("CARGO_PKG_VERSION"),);

#[tokio::main]
async fn main() -> iwf::Result<()> {
    let cli = wfopts::Cli::parse();
    iwf::setup_logger();
    let ret = iwf::wfo::<GlsWf, DefaultSettings>(&cli, APP_INFO).await?;
    if let Some((params, task, settings, execs)) = ret {
        match params.action {
            iwf::WfAction::Run => run(&params, task, settings, execs).await?,
        }
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Default, Hash)]
pub struct Qmi {
    filepath: Utf8PathBuf,
    name: String,
    ex: bool,
    impute: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Hash)]
pub struct WkdeSettings {
    fraction: OrderedFloat<f64>,
    bandwidth: OrderedFloat<f64>,
    min_value: OrderedFloat<f64>,
    no_class_criteria: OrderedFloat<f64>,
    max_n_labels: u8,
}

#[derive(IwfWf, Serialize, Deserialize, Debug, Default, Hash)]
#[iwfworkflow(method = "gls v0.1.0", name = "gls", description = "gls workflow")]
pub struct GlsWf {
    pub name: String,
    pub objects: BTreeMap<String, OsObject>,
    pub atlas_list: Vec<String>,
    pub protein_list: Vec<String>,
    pub protein_list_dev: Vec<String>,
    pub pl_4n: Vec<String>,
    pub nvar_grp: BTreeMap<String, String>,
    pub f4g: Vec<String>,
    pub f3a: Vec<String>,
    pub f3b: Vec<String>,
    pub qmi: Vec<Qmi>,
    pub gmt: Utf8PathBuf,
    pub wkde_settings: WkdeSettings,
    pub sample_map: BTreeMap<String, (String, String)>,
}

#[derive(Deserialize)]
struct IwfT {
    bandwidth: OrderedFloat<f64>,
    fraction: OrderedFloat<f64>,
    min_value: OrderedFloat<f64>,
    no_class_criteria: OrderedFloat<f64>,
    max_n_labels: u8,
}

impl IwfSetup for GlsWf {}
impl IwfRun for GlsWf {}
impl IwfTuning for GlsWf {
    fn tune(&mut self, json: &str) -> iwf::Result<()> {
        let tune: IwfT = serde_json::from_str(json)?;
        self.wkde_settings.bandwidth = tune.bandwidth;
        self.wkde_settings.fraction = tune.fraction;
        self.wkde_settings.min_value = tune.min_value;
        self.wkde_settings.no_class_criteria = tune.no_class_criteria;
        self.wkde_settings.max_n_labels = tune.max_n_labels;
        Ok(())
    }
}

async fn run(
    params: &iwf::WfParameters,
    task: GlsWf,
    settings: DefaultSettings,
    execs: HashMap<String, impl IwfExe + Clone>,
) -> iwf::Result<()> {
    let ctx = &Ctx::new(1)?;
    let mut dbc = settings.dbcredentials();
    let db = params
        .wd_path
        .file_name()
        .unwrap_or("no_file_name_in_wd_path")
        .replace('.', "_");

    dbc.update_database(&db);
    #[cfg(feature = "rebuild")]
    iwf::sql::create_db(&dbc).await?;
    #[cfg(feature = "rebuild")]
    iwf::setup_dgs(&dbc).await?;

    let mm = &ModelManager::new(&dbc.get_db_url()).await?;
    let objects: HashMap<_, _> = task.objects.clone().into_iter().collect();
    let _datasets = download_named_resources(&objects, &settings.oscredentials()).await?;
    #[cfg(feature = "rebuild")]
    {
        services::opendata::create(mm, &dbc).await?;
        services::rawdata::create(ctx, mm, &dbc, &task.qmi).await?;
        services::deriveddata::create(&dbc).await?;
        match protocol(ctx, mm, &task, execs, params).await {
            Ok(()) => (),
            Err(e) => iwf::dgs(&e)?,
        }
    }

    #[cfg(feature = "report")]
    match create_report(ctx, mm, params.wd_path.clone(), &task).await {
        Ok(()) => (),
        Err(e) => iwf::dgs(&e)?,
    }

    let dgs = iwf::get_dgsfc()?;
    iwf::DgsBmc::bulk_import(mm, dgs).await?;

    Ok(())
}

#[cfg(feature = "rebuild")]
async fn protocol(
    ctx: &Ctx,
    mm: &ModelManager,
    task: &GlsWf,
    execs: HashMap<String, impl IwfExe + Clone>,
    params: &iwf::WfParameters,
) -> iwf::Result<()> {
    let gsea = iwf::get_exec(&execs, "gseapy");
    let wkde = iwf::get_exec(&execs, "wkde");
    let gmt = &task.gmt;

    for name in &task.atlas_list {
        let _ = ExeClusteringTask {
            ctx,
            mm,
            gsea: gsea.clone(),
            params,
            name,
        }
        .execute()
        .await?;
    }
    for kind in &task.atlas_list {
        let _ = ExeWkdeTask {
            mm,
            params,
            wkde: wkde.clone(),
            kind,
            fraction: task.wkde_settings.fraction.into(),
            bandwidth: task.wkde_settings.bandwidth.into(),
            no_class_criteria: task.wkde_settings.no_class_criteria.into(),
            min_value: task.wkde_settings.min_value.into(),
        }
        .execute()
        .await?;
    }

    let _ = CalculateWkdeEnrichmentTask {
        mm,
        gmt,
        gsea: gsea.clone(),
        params,
    }
    .execute()
    .await?;
    let _ = CalculateLabelDataTask { ctx, mm }.execute().await?;
    let _ = SepsisPatientUmapTask {
        ctx,
        mm,
        gsea: gsea.clone(),
        params,
        protein_list: &task.pl_4n,
    }
    .execute()
    .await?;
    let _ = MiPatientUmapTask {
        ctx,
        mm,
        gsea: gsea.clone(),
        params,
    }
    .execute()
    .await?;
    let _ = Fig4gUmapTask {
        ctx,
        mm,
        gsea: gsea.clone(),
        params,
        protein_list: &task.f4g,
        name: "fig4g",
    }
    .execute()
    .await?;
    let _ = Fig4fUmapTask {
        ctx,
        mm,
        gsea: gsea.clone(),
        params,
    }
    .execute()
    .await?;
    let _ = CalculateEnrichmentListTask {
        mm,
        gsea: gsea.clone(),
        params,
        gmt,
        kind: "fig3b",
        protein_list: &task.f3b,
    }
    .execute()
    .await?;
    let _ = CalculateVarianceTask {
        mm,
        mpr: &task.sample_map.clone().into_iter().collect(),
    }
    .execute()
    .await?;
    let _ = CalculateEnrichmentFig2eTask {
        mm,
        gsea,
        params,
        gmt,
    }
    .execute()
    .await?;
    Ok(())
}

#[cfg(feature = "report")]
async fn create_report(
    _ctx: &Ctx,
    mm: &ModelManager,
    basepath: Utf8PathBuf,
    task: &GlsWf,
) -> iwf::Result<()> {
    println!("Creating the report");
    let mut book = MBook {
        basepath,
        ..Default::default()
    };
    let mut wb = Workbook::new();
    book.add_page(StdPage::get_page(
        &ddbstp::gls::fig1d::get(mm)
            .await
            .map_err(|e| iwf::Error::Defined(format!("Cannot create fig1d: {e}")))?,
        &mut wb,
    )?);
    book.add_page(StdPage::get_page(
        &ddbstp::gls::fig1e::get(mm)
            .await
            .map_err(|e| iwf::Error::Defined(format!("Cannot create fig1e: {e}")))?,
        &mut wb,
    )?);
    book.add_page(StdPage::get_page(
        &ddbstp::gls::fig2c::get(mm)
            .await
            .map_err(|e| iwf::Error::Defined(format!("Cannot create fig2c: {e}")))?,
        &mut wb,
    )?);
    book.add_page(StdPage::get_page(
        &ddbstp::gls::fig2d::get(mm)
            .await
            .map_err(|e| iwf::Error::Defined(format!("Cannot create fig2d: {e}")))?,
        &mut wb,
    )?);
    book.add_page(StdPage::get_page(
        &ddbstp::gls::fig2e::get(mm, "brain_gls")
            .await
            .map_err(|e| iwf::Error::Defined(format!("Cannot create fig2e: {e}")))?,
        &mut wb,
    )?);
    book.add_page(StdPage::get_page(
        &ddbstp::gls::fig2f::get(mm)
            .await
            .map_err(|e| iwf::Error::Defined(format!("Cannot create fig2f: {e}")))?,
        &mut wb,
    )?);
    book.add_page(StdPage::get_page(
        &ddbstp::gls::fig2g::get(mm)
            .await
            .map_err(|e| iwf::Error::Defined(format!("Cannot create fig2g: {e}")))?,
        &mut wb,
    )?);
    book.add_page(StdPage::get_page(
        &ddbstp::gls::fig3a::get(mm, &task.f3a)
            .await
            .map_err(|e| iwf::Error::Defined(format!("Cannot create fig3a: {e}")))?,
        &mut wb,
    )?);
    book.add_page(StdPage::get_page(
        &ddbstp::gls::fig3b::get(mm)
            .await
            .map_err(|e| iwf::Error::Defined(format!("Cannot create fig3b: {e}")))?,
        &mut wb,
    )?);
    book.add_page(StdPage::get_page(
        &ddbstp::gls::fig3c::get(mm, &task.nvar_grp)
            .await
            .map_err(|e| iwf::Error::Defined(format!("Cannot create fig3c: {e}")))?,
        &mut wb,
    )?);
    book.add_page(StdPage::get_page(
        &ddbstp::gls::fig3d::get(mm)
            .await
            .map_err(|e| iwf::Error::Defined(format!("Cannot create fig3d: {e}")))?,
        &mut wb,
    )?);
    book.add_page(StdPage::get_page(
        &ddbstp::gls::fig4d::get(mm)
            .await
            .map_err(|e| iwf::Error::Defined(format!("Cannot create fig4d: {e}")))?,
        &mut wb,
    )?);
    book.add_page(StdPage::get_page(
        &ddbstp::gls::fig4e::get(mm)
            .await
            .map_err(|e| iwf::Error::Defined(format!("Cannot create fig4e: {e}")))?,
        &mut wb,
    )?);
    book.add_page(StdPage::get_page(
        &ddbstp::gls::fig4f::get(mm)
            .await
            .map_err(|e| iwf::Error::Defined(format!("Cannot create fig4f: {e}")))?,
        &mut wb,
    )?);
    book.add_page(StdPage::get_page(
        &ddbstp::gls::fig4g::get(mm, "fig4")
            .await
            .map_err(|e| iwf::Error::Defined(format!("Cannot create fig4g: {e}")))?,
        &mut wb,
    )?);
    book.add_page(StdPage::get_page(
        &ddbstp::gls::fig4i::get(mm)
            .await
            .map_err(|e| iwf::Error::Defined(format!("Cannot create fig4i: {e}")))?,
        &mut wb,
    )?);
    book.add_page(StdPage::get_page(
        &ddbstp::gls::fig4ja::get(mm)
            .await
            .map_err(|e| iwf::Error::Defined(format!("Cannot create fig4ja: {e}")))?,
        &mut wb,
    )?);
    book.add_page(StdPage::get_page(
        &ddbstp::gls::fig4jb::get(mm)
            .await
            .map_err(|e| iwf::Error::Defined(format!("Cannot create fig4jb: {e}")))?,
        &mut wb,
    )?);
    book.add_page(StdPage::get_page(
        &ddbstp::gls::fig4k::get(mm)
            .await
            .map_err(|e| iwf::Error::Defined(format!("Cannot create fig4k: {e}")))?,
        &mut wb,
    )?);
    book.add_page(StdPage::get_page(
        &ddbstp::gls::fig4l::get(mm)
            .await
            .map_err(|e| iwf::Error::Defined(format!("Cannot create fig4l: {e}")))?,
        &mut wb,
    )?);
    book.add_page(StdPage::get_page(
        &ddbstp::gls::fig4m::get(mm)
            .await
            .map_err(|e| iwf::Error::Defined(format!("Cannot create fig4m: {e}")))?,
        &mut wb,
    )?);
    book.add_page(StdPage::get_page(
        &ddbstp::gls::fig4n::get(mm)
            .await
            .map_err(|e| iwf::Error::Defined(format!("Cannot create fig4n: {e}")))?,
        &mut wb,
    )?);
    book.add_page(StdPage::get_page(
        &ddbstp::gls::fig4o::get(mm)
            .await
            .map_err(|e| iwf::Error::Defined(format!("Cannot create fig4o: {e}")))?,
        &mut wb,
    )?);

    book.render()?;
    wb.save(format!("gls_{}.xlsx", env!("CARGO_PKG_VERSION")))?;
    Ok(())
}
