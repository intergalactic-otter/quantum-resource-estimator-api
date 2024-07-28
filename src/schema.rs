use juniper::{EmptySubscription, FieldResult, RootNode};
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
struct EstimationResult {
    status: String,
    job_params: JobParams,
    physical_counts: PhysicalCounts,
    physical_counts_formatted: PhysicalCountsFormatted,
    logical_qubit: LogicalQubit,
    error_budget: ErrorBudget,
    logical_counts: LogicalCounts,
    report_data: ReportData,
}

#[derive(GraphQLObject)]
struct JobParams {
    qec_scheme: QecScheme,
    error_budget: f64,
    qubit_params: QubitParams,
    constraints: Constraints,
    estimate_type: String,
}

#[derive(GraphQLObject)]
struct QecScheme {
    name: String,
    error_correction_threshold: f64,
    crossing_prefactor: f64,
    logical_cycle_time: String,
    physical_qubits_per_logical_qubit: String,
    max_code_distance: i32,
}

#[derive(GraphQLObject)]
struct QubitParams {
    instruction_set: String,
    name: String,
    one_qubit_measurement_time: String,
    one_qubit_gate_time: String,
    two_qubit_gate_time: String,
    t_gate_time: String,
    one_qubit_measurement_error_rate: f64,
    one_qubit_gate_error_rate: f64,
    two_qubit_gate_error_rate: f64,
    t_gate_error_rate: f64,
    idle_error_rate: f64,
}

#[derive(GraphQLObject)]
struct Constraints {
    max_distillation_rounds: i32,
}

#[derive(GraphQLObject)]
struct PhysicalCounts {
    physical_qubits: i32,
    runtime: i32,
    rqops: i32,
    breakdown: PhysicalCountsBreakdown,
}

#[derive(GraphQLObject)]
struct PhysicalCountsBreakdown {
    algorithmic_logical_qubits: i32,
    algorithmic_logical_depth: i32,
    logical_depth: i32,
    num_tstates: i32,
    clock_frequency: f64,
    num_tfactories: i32,
    num_tfactory_runs: i32,
    physical_qubits_for_tfactories: i32,
    physical_qubits_for_algorithm: i32,
    required_logical_qubit_error_rate: f64,
    required_logical_tstate_error_rate: Option<f64>,
    num_ts_per_rotation: Option<i32>,
    clifford_error_rate: f64,
}

#[derive(GraphQLObject)]
struct PhysicalCountsFormatted {
    runtime: String,
    rqops: String,
    physical_qubits: String,
    algorithmic_logical_qubits: String,
    algorithmic_logical_depth: String,
    logical_depth: String,
    num_tstates: String,
    num_tfactories: String,
    num_tfactory_runs: String,
    physical_qubits_for_algorithm: String,
    physical_qubits_for_tfactories: String,
    physical_qubits_for_tfactories_percentage: String,
    required_logical_qubit_error_rate: String,
    required_logical_tstate_error_rate: String,
    physical_qubits_per_logical_qubit: String,
    logical_cycle_time: String,
    clock_frequency: String,
    logical_error_rate: String,
    // ... (other fields)
}

#[derive(GraphQLObject)]
struct LogicalQubit {
    code_distance: i32,
    physical_qubits: i32,
    logical_cycle_time: i32,
    logical_error_rate: f64,
}

#[derive(GraphQLObject)]
struct ErrorBudget {
    logical: f64,
    tstates: f64,
    rotations: f64,
}

#[derive(GraphQLObject)]
struct LogicalCounts {
    num_qubits: i32,
    t_count: i32,
    rotation_count: i32,
    rotation_depth: i32,
    ccz_count: i32,
    ccix_count: i32,
    measurement_count: i32,
}

#[derive(GraphQLObject)]
struct ReportData {
    groups: Vec<ReportGroup>,
    assumptions: Vec<String>,
}

#[derive(GraphQLObject)]
struct ReportGroup {
    title: String,
    always_visible: bool,
    entries: Vec<ReportEntry>,
}

#[derive(GraphQLObject)]
struct ReportEntry {
    path: String,
    label: String,
    description: String,
    explanation: String,
}

pub struct Query;

#[juniper::graphql_object]
impl Query {
    fn estimation_result() -> FieldResult<EstimationResult> {
        Ok(EstimationResult {
            status: "Success".to_string(),
            job_params: JobParams {
                qec_scheme: QecScheme {
                    name: "Example".to_string(),
                    error_correction_threshold: 0.1,
                    crossing_prefactor: 1.0,
                    logical_cycle_time: "1ns".to_string(),
                    physical_qubits_per_logical_qubit: "100".to_string(),
                    max_code_distance: 5,
                },
                error_budget: 0.01,
                qubit_params: QubitParams {
                    instruction_set: "Set".to_string(),
                    name: "Qubit".to_string(),
                    one_qubit_measurement_time: "10ns".to_string(),
                    one_qubit_gate_time: "10ns".to_string(),
                    two_qubit_gate_time: "20ns".to_string(),
                    t_gate_time: "30ns".to_string(),
                    one_qubit_measurement_error_rate: 0.001,
                    one_qubit_gate_error_rate: 0.002,
                    two_qubit_gate_error_rate: 0.003,
                    t_gate_error_rate: 0.004,
                    idle_error_rate: 0.005,
                },
                constraints: Constraints {
                    max_distillation_rounds: 3,
                },
                estimate_type: "type".to_string(),
            },
            physical_counts: PhysicalCounts {
                physical_qubits: 1000,
                runtime: 1000,
                rqops: 1000,
                breakdown: PhysicalCountsBreakdown {
                    algorithmic_logical_qubits: 50,
                    algorithmic_logical_depth: 500,
                    logical_depth: 500,
                    num_tstates: 100,
                    clock_frequency: 1.0,
                    num_tfactories: 10,
                    num_tfactory_runs: 100,
                    physical_qubits_for_tfactories: 1000,
                    physical_qubits_for_algorithm: 5000,
                    required_logical_qubit_error_rate: 0.0001,
                    required_logical_tstate_error_rate: None,
                    num_ts_per_rotation: None,
                    clifford_error_rate: 0.001,
                },
            },
            physical_counts_formatted: PhysicalCountsFormatted {
                runtime: "1000s".to_string(),
                rqops: "1000".to_string(),
                physical_qubits: "1000".to_string(),
                algorithmic_logical_qubits: "50".to_string(),
                algorithmic_logical_depth: "500".to_string(),
                logical_depth: "500".to_string(),
                num_tstates: "100".to_string(),
                num_tfactories: "10".to_string(),
                num_tfactory_runs: "100".to_string(),
                physical_qubits_for_algorithm: "5000".to_string(),
                physical_qubits_for_tfactories: "1000".to_string(),
                physical_qubits_for_tfactories_percentage: "20%".to_string(),
                required_logical_qubit_error_rate: "0.0001".to_string(),
                required_logical_tstate_error_rate: "N/A".to_string(),
                physical_qubits_per_logical_qubit: "100".to_string(),
                logical_cycle_time: "1ns".to_string(),
                clock_frequency: "1GHz".to_string(),
                logical_error_rate: "0.0001".to_string(),
            },
            logical_qubit: LogicalQubit {
                code_distance: 5,
                physical_qubits: 100,
                logical_cycle_time: 100,
                logical_error_rate: 0.0001,
            },
            error_budget: ErrorBudget {
                logical: 0.01,
                tstates: 0.02,
                rotations: 0.03,
            },
            logical_counts: LogicalCounts {
                num_qubits: 10,
                t_count: 100,
                rotation_count: 100,
                rotation_depth: 100,
                ccz_count: 10,
                ccix_count: 5,
                measurement_count: 1000,
            },
            report_data: ReportData {
                groups: vec![ReportGroup {
                    title: "Group1".to_string(),
                    always_visible: true,
                    entries: vec![ReportEntry {
                        path: "/path1".to_string(),
                        label: "Label1".to_string(),
                        description: "Description1".to_string(),
                        explanation: "Explanation1".to_string(),
                    }],
                }],
                assumptions: vec!["Assumption1".to_string()],
            },
        })
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Input data for estimation")]
struct EstimationInput {
    label: String,
    detail: String,
    params: ParamsInput,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Parameters for the estimation")]
struct ParamsInput {
    qubit_params: QubitParamsInput,
    qec_scheme: QecSchemeInput,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Qubit parameters for the estimation")]
struct QubitParamsInput {
    name: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "QEC scheme details")]
struct QecSchemeInput {
    name: String,
}

pub struct Mutation;

#[juniper::graphql_object]
impl Mutation {
    fn create_estimation(estimation: EstimationInput) -> FieldResult<EstimationResult> {
        Ok(EstimationResult {
            status: "Success".to_string(),
            job_params: JobParams {
                qec_scheme: QecScheme {
                    name: "Example".to_string(),
                    error_correction_threshold: 0.1,
                    crossing_prefactor: 1.0,
                    logical_cycle_time: "1ns".to_string(),
                    physical_qubits_per_logical_qubit: "100".to_string(),
                    max_code_distance: 5,
                },
                error_budget: 0.01,
                qubit_params: QubitParams {
                    instruction_set: "Set".to_string(),
                    name: "Qubit".to_string(),
                    one_qubit_measurement_time: "10ns".to_string(),
                    one_qubit_gate_time: "10ns".to_string(),
                    two_qubit_gate_time: "20ns".to_string(),
                    t_gate_time: "30ns".to_string(),
                    one_qubit_measurement_error_rate: 0.001,
                    one_qubit_gate_error_rate: 0.002,
                    two_qubit_gate_error_rate: 0.003,
                    t_gate_error_rate: 0.004,
                    idle_error_rate: 0.005,
                },
                constraints: Constraints {
                    max_distillation_rounds: 3,
                },
                estimate_type: "type".to_string(),
            },
            physical_counts: PhysicalCounts {
                physical_qubits: 1000,
                runtime: 1000,
                rqops: 1000,
                breakdown: PhysicalCountsBreakdown {
                    algorithmic_logical_qubits: 50,
                    algorithmic_logical_depth: 500,
                    logical_depth: 500,
                    num_tstates: 100,
                    clock_frequency: 1.0,
                    num_tfactories: 10,
                    num_tfactory_runs: 100,
                    physical_qubits_for_tfactories: 1000,
                    physical_qubits_for_algorithm: 5000,
                    required_logical_qubit_error_rate: 0.0001,
                    required_logical_tstate_error_rate: None,
                    num_ts_per_rotation: None,
                    clifford_error_rate: 0.001,
                },
            },
            physical_counts_formatted: PhysicalCountsFormatted {
                runtime: "1000s".to_string(),
                rqops: "1000".to_string(),
                physical_qubits: "1000".to_string(),
                algorithmic_logical_qubits: "50".to_string(),
                algorithmic_logical_depth: "500".to_string(),
                logical_depth: "500".to_string(),
                num_tstates: "100".to_string(),
                num_tfactories: "10".to_string(),
                num_tfactory_runs: "100".to_string(),
                physical_qubits_for_algorithm: "5000".to_string(),
                physical_qubits_for_tfactories: "1000".to_string(),
                physical_qubits_for_tfactories_percentage: "20%".to_string(),
                required_logical_qubit_error_rate: "0.0001".to_string(),
                required_logical_tstate_error_rate: "N/A".to_string(),
                physical_qubits_per_logical_qubit: "100".to_string(),
                logical_cycle_time: "1ns".to_string(),
                clock_frequency: "1GHz".to_string(),
                logical_error_rate: "0.0001".to_string(),
            },
            logical_qubit: LogicalQubit {
                code_distance: 5,
                physical_qubits: 100,
                logical_cycle_time: 100,
                logical_error_rate: 0.0001,
            },
            error_budget: ErrorBudget {
                logical: 0.01,
                tstates: 0.02,
                rotations: 0.03,
            },
            logical_counts: LogicalCounts {
                num_qubits: 10,
                t_count: 100,
                rotation_count: 100,
                rotation_depth: 100,
                ccz_count: 10,
                ccix_count: 5,
                measurement_count: 1000,
            },
            report_data: ReportData {
                groups: vec![ReportGroup {
                    title: "Group1".to_string(),
                    always_visible: true,
                    entries: vec![ReportEntry {
                        path: "/path1".to_string(),
                        label: "Label1".to_string(),
                        description: "Description1".to_string(),
                        explanation: "Explanation1".to_string(),
                    }],
                }],
                assumptions: vec!["Assumption1".to_string()],
            },
        })
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {}, EmptySubscription::new())
}