use std::{fs, io};
use std::io::{
    Read
};
use std::path::Path;
use std::sync::Arc;
use qsc_frontend::compile::{SourceContents, SourceName};
use miette::{Context, IntoDiagnostic};
use qsc::{interpret, PackageType};
use qsc::packages::BuildableProgram;
use qsc::target::Profile;
use qsc_project::{PackageGraphSources, Project};
use resource_estimator::estimate_entry;
use miette::Result;


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

    /**
        This function call builds all dependencies as a part of preparing the package store for building the user code.
    **/
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

pub fn estimate(file_path: impl AsRef<Path>) -> Result<String> {
    let (source_name, source_contents) = read_source(file_path)?;

    let project_config = Project::from_single_file(
        Arc::from(source_name.as_ref()),
        source_contents
    );

    let (source_map, capabilities, language_features, store, deps) =
        project_to_qsc_args(project_config.package_graph_sources, None)
            .map_err(|e| miette::Error::msg(format!("QSC argument conversion error: {:?}", e)))?;

    let mut interpreter = interpret::Interpreter::new(
        source_map,
        PackageType::Exe,
        capabilities,
        language_features,
        store,
        &deps[..],
    ).map_err(|e| miette::Error::msg(format!("Interpreter creation error: {:?}", e)))?;

    let estimation_result = estimate_entry(&mut interpreter, r#"[{ "label": "qubit_maj_ns_e6 + surface_code", "detail": "Majorana qubit with 1e-6 error rate (surface code QEC)", "params": { "qubitParams": { "name": "qubit_maj_ns_e6" }, "qecScheme": { "name": "surface_code" } } }]"#)
        .map_err(|e| match &e[0] {
            resource_estimator::Error::Interpreter(interpret::Error::Eval(e)) => miette::Error::msg(e.to_string()),
            resource_estimator::Error::Interpreter(_) => miette::Error::msg("Unexpected interpreter error"),
            resource_estimator::Error::Estimation(e) => miette::Error::msg(e.to_string()),
        })?;

    Ok(estimation_result)
}

