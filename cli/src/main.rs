use std::{fs, io};
use std::io::{
    Read
};
use std::path::Path;
use std::sync::Arc;
use clap::Parser;
use qsc_frontend::compile::{PackageStore, SourceContents, SourceName};
use miette::{Context, IntoDiagnostic};
use qsc::packages::BuildableProgram;
use qsc::target::Profile;
use qsc::target::TargetCapabilityFlags;
use qsc_wasm::project_system::{into_qsc_args_x, IProjectConfig};
use qsc_wasm::project_system::ProgramConfig;
use qsc_project::{PackageGraphSources, Project};

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn read_source(path: impl AsRef<Path>) -> miette::Result<(SourceName, SourceContents)> {
    let path = path.as_ref();
    if path.as_os_str() == "-" {
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .into_diagnostic()
            .context("could not read standard input")?;

        Ok(("<stdin>".into(), input.into()))
    } else {
        let contents = fs::read_to_string(path)
            .into_diagnostic()
            .with_context(|| format!("could not read source file `{}`", path.display()))?;

        Ok((path.to_string_lossy().into(), contents.into()))
    }
}

#[allow(clippy::type_complexity)]
#[allow(clippy::needless_pass_by_value)]
pub fn project_to_qsc_args(
    package_graph_sources: PackageGraphSources,
    entry: Option<String>,
) -> Result<
    (
        qsc::SourceMap,
        qsc::TargetCapabilityFlags,
        qsc::LanguageFeatures,
        qsc::PackageStore,
        Vec<(qsc::hir::PackageId, Option<Arc<str>>)>,
    ),
    Vec<qsc::compile::Error>,
> {
    /**
        We're only going to use the AdaptiveRI profile for now.
    **/
    let capabilities = qsc::TargetCapabilityFlags::from(Profile::AdaptiveRI);

    let pkg_graph: PackageGraphSources = package_graph_sources.into();
    let pkg_graph: qsc_project::PackageGraphSources = pkg_graph.into();

    // this function call builds all dependencies as a part of preparing the package store
    // for building the user code.
    let buildable_program = BuildableProgram::new(capabilities, pkg_graph);

    if !buildable_program.dependency_errors.is_empty() {
        return Err(buildable_program.dependency_errors);
    }

    let BuildableProgram {
        store,
        user_code,
        user_code_dependencies,
        ..
    } = buildable_program;

    let source_map = qsc::SourceMap::new(user_code.sources, entry.map(std::convert::Into::into));
    let language_features = qsc::LanguageFeatures::from_iter(user_code.language_features);

    Ok((
        source_map,
        capabilities,
        language_features,
        store,
        user_code_dependencies,
    ))
}


fn main() {
    let args = Cli::parse();
    println!("Reading path from: {:?}", args.path);

    let path = args.path;

    let paths = vec![
        path,
    ];

    let sources = paths.iter()
            .map(read_source)
            .collect::<miette::Result<Vec<_>>>().unwrap();

    let mut store = PackageStore::new(qsc::compile::core());

    let single_source_ptr = sources.get(0).unwrap();
    let single_source = single_source_ptr.clone();
    let single_source_content = single_source.1;

    let project_config = Project::from_single_file(
        Arc::from("single_file"),
        single_source_content
    );

    println!("Loaded file to memory.");

    let qsc_args = project_to_qsc_args(project_config.package_graph_sources, None);
    println!("qsc_args {:?}", qsc_args);
}
