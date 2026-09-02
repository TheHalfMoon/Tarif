use std::error::Error;
use std::fmt;

/// Stable, fail-closed error taxonomy for the Specification 002 boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionError {
    InvalidJson(String),
    DuplicateJsonKey(String),
    ExpectedObject(&'static str),
    MissingField(&'static str),
    UnknownTopLevelField(String),
    InvalidJsonRpcVersion,
    InvalidRequestId,
    UnsupportedProtocolRevision(String),
    UnsupportedMethod(String),
    InvalidToolName(String),
    ArgumentsNotObject,
    UnsupportedExecutionState(String),
    MetaNotObject,
    UnknownMetaField(String),
    InvalidMetaField {
        field: String,
        expected: &'static str,
    },
    ProtocolVersionMismatch {
        expected: String,
        observed: String,
    },
    UnsupportedActionSchema(String),
    InvalidActionIr(String),
    Canonicalization(String),
}

impl ActionError {
    /// Stable machine-oriented category. Human detail remains in the variant.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::InvalidJson(_) => "invalid_json",
            Self::DuplicateJsonKey(_) => "duplicate_json_key",
            Self::ExpectedObject(_) => "expected_object",
            Self::MissingField(_) => "missing_field",
            Self::UnknownTopLevelField(_) => "unknown_top_level_field",
            Self::InvalidJsonRpcVersion => "invalid_jsonrpc_version",
            Self::InvalidRequestId => "invalid_request_id",
            Self::UnsupportedProtocolRevision(_) => "unsupported_protocol_revision",
            Self::UnsupportedMethod(_) => "unsupported_method",
            Self::InvalidToolName(_) => "invalid_tool_name",
            Self::ArgumentsNotObject => "arguments_not_object",
            Self::UnsupportedExecutionState(_) => "unsupported_execution_state",
            Self::MetaNotObject => "meta_not_object",
            Self::UnknownMetaField(_) => "unknown_meta_field",
            Self::InvalidMetaField { .. } => "invalid_meta_field",
            Self::ProtocolVersionMismatch { .. } => "protocol_version_mismatch",
            Self::UnsupportedActionSchema(_) => "unsupported_action_schema",
            Self::InvalidActionIr(_) => "invalid_action_ir",
            Self::Canonicalization(_) => "canonicalization_failed",
        }
    }
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidJson(detail) => write!(f, "invalid JSON: {detail}"),
            Self::DuplicateJsonKey(key) => write!(f, "duplicate JSON object key: {key}"),
            Self::ExpectedObject(field) => write!(f, "{field} must be a JSON object"),
            Self::MissingField(field) => write!(f, "missing required field: {field}"),
            Self::UnknownTopLevelField(field) => write!(f, "unsupported top-level field: {field}"),
            Self::InvalidJsonRpcVersion => f.write_str("jsonrpc must be exactly \"2.0\""),
            Self::InvalidRequestId => f.write_str("MCP tools/call must contain a string or number request id"),
            Self::UnsupportedProtocolRevision(revision) => {
                write!(f, "unsupported MCP protocol revision: {revision}")
            }
            Self::UnsupportedMethod(method) => write!(f, "unsupported MCP method: {method}"),
            Self::InvalidToolName(name) => write!(f, "unsupported MCP tool name: {name}"),
            Self::ArgumentsNotObject => f.write_str("tools/call arguments, when present, must be an object"),
            Self::UnsupportedExecutionState(field) => {
                write!(f, "unsupported execution-affecting state: {field}")
            }
            Self::MetaNotObject => f.write_str("params._meta must be an object when present"),
            Self::UnknownMetaField(field) => write!(f, "unsupported MCP _meta field: {field}"),
            Self::InvalidMetaField { field, expected } => {
                write!(f, "MCP _meta field {field} must be {expected}")
            }
            Self::ProtocolVersionMismatch { expected, observed } => write!(
                f,
                "MCP _meta protocol version mismatch: expected {expected}, observed {observed}"
            ),
            Self::UnsupportedActionSchema(schema) => write!(f, "unsupported Action IR schema: {schema}"),
            Self::InvalidActionIr(detail) => write!(f, "invalid Action IR: {detail}"),
            Self::Canonicalization(detail) => write!(f, "JCS canonicalization failed: {detail}"),
        }
    }
}

impl Error for ActionError {}
