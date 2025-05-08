use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// DAP 메시지 기본 구조
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "request")]
    Request(Request),
    
    #[serde(rename = "response")]
    Response(Response),
    
    #[serde(rename = "event")]
    Event(Event),
}

/// 요청 메시지 구조
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub seq: i64,
    pub command: String,
    #[serde(default)]
    pub arguments: serde_json::Value,
}

/// 응답 메시지 구조
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub request_seq: i64,
    pub success: bool,
    pub command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
}

/// 이벤트 메시지 구조
#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub seq: i64,
    pub event: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
}

/// 초기화 요청 인자
#[derive(Debug, Serialize, Deserialize)]
pub struct InitializeRequestArguments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_start_at1: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_start_at1: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_variable_type: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_variable_paging: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_run_in_terminal_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_memory_references: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_progress_reporting: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_invalidated_event: Option<bool>,
}

/// 초기화 응답 본문
#[derive(Debug, Serialize, Deserialize)]
pub struct InitializeResponseBody {
    pub supports_configuration_done_request: bool,
    pub supports_function_breakpoints: bool,
    pub supports_conditional_breakpoints: bool,
    pub supports_hit_conditional_breakpoints: bool,
    pub supports_evaluate_for_hovers: bool,
    pub supports_step_back: bool,
    pub supports_set_variable: bool,
    pub supports_restart_frame: bool,
    pub supports_goto_targets_request: bool,
    pub supports_step_in_targets_request: bool,
    pub supports_completions_request: bool,
    pub supports_modules_request: bool,
    pub supports_restart_request: bool,
    pub supports_exception_options: bool,
    pub supports_value_formatting_options: bool,
    pub supports_terminate_request: bool,
    pub supports_terminate_threads_request: bool,
    pub supports_data_breakpoints: bool,
    pub supports_read_memory_request: bool,
    pub supports_write_memory_request: bool,
    pub supports_disassemble_request: bool,
    pub supports_cancel_request: bool,
    pub supports_breakpoint_locations_request: bool,
    pub supports_clipboard_context: bool,
    pub supports_stepping_granularity: bool,
    pub supports_instruction_breakpoints: bool,
    pub supports_exception_filter_options: bool,
}

/// 실행(launch) 요청 인자
#[derive(Debug, Serialize, Deserialize)]
pub struct LaunchRequestArguments {
    pub program: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub stop_on_entry: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
}

/// 중단점 정보
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceBreakpoint {
    pub line: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_message: Option<String>,
}

/// 중단점 설정 요청 인자
#[derive(Debug, Serialize, Deserialize)]
pub struct SetBreakpointsArguments {
    pub source: Source,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub breakpoints: Option<Vec<SourceBreakpoint>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub lines: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub source_modified: Option<bool>,
}

/// 소스 파일 정보
#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_reference: Option<i64>,
}

/// 중단점 설정 응답
#[derive(Debug, Serialize, Deserialize)]
pub struct SetBreakpointsResponseBody {
    pub breakpoints: Vec<Breakpoint>,
}

/// 중단점 정보
#[derive(Debug, Serialize, Deserialize)]
pub struct Breakpoint {
    pub id: Option<i64>,
    pub verified: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i64>,
}

/// 스레드 요청에 대한 응답
#[derive(Debug, Serialize, Deserialize)]
pub struct ThreadsResponseBody {
    pub threads: Vec<Thread>,
}

/// 스레드 정보
#[derive(Debug, Serialize, Deserialize)]
pub struct Thread {
    pub id: i64,
    pub name: String,
}

/// 스택 트레이스 요청 인자
#[derive(Debug, Serialize, Deserialize)]
pub struct StackTraceArguments {
    pub thread_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_frame: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub levels: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<StackFrameFormat>,
}

/// 스택 프레임 포맷 옵션
#[derive(Debug, Serialize, Deserialize)]
pub struct StackFrameFormat {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_types: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_names: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_values: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_all: Option<bool>,
}

/// 스택 트레이스 응답
#[derive(Debug, Serialize, Deserialize)]
pub struct StackTraceResponseBody {
    pub stack_frames: Vec<StackFrame>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_frames: Option<i64>,
}

/// 스택 프레임 정보
#[derive(Debug, Serialize, Deserialize)]
pub struct StackFrame {
    pub id: i64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    pub line: i64,
    pub column: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentation_hint: Option<String>,
}

/// 중단(Stop) 이벤트 본문
#[derive(Debug, Serialize, Deserialize)]
pub struct StoppedEventBody {
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_focus_hint: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_threads_stopped: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_breakpoint_ids: Option<Vec<i64>>,
}

// 변수 스코프 조회 응답
#[derive(Debug, Serialize, Deserialize)]
pub struct ScopesResponseBody {
    pub scopes: Vec<Scope>,
}

// 변수 스코프 정보
#[derive(Debug, Serialize, Deserialize)]
pub struct Scope {
    pub name: String,
    pub variables_reference: i64,
    pub expensive: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_variables: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexed_variables: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i64>,
}

// 변수 목록 조회 응답
#[derive(Debug, Serialize, Deserialize)]
pub struct VariablesResponseBody {
    pub variables: Vec<Variable>,
}

// 변수 정보
#[derive(Debug, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentation_hint: Option<VariablePresentationHint>,
    pub variables_reference: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_variables: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexed_variables: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reference: Option<String>,
}

// 변수 표시 힌트
#[derive(Debug, Serialize, Deserialize)]
pub struct VariablePresentationHint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}