use crate::ActionError;
use crate::strict_json::parse_strict_json;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::BTreeMap;

pub const ACTION_SCHEMA_V1: &str = "tarif.action/v1";
pub const MCP_PROTOCOL_NAME: &str = "mcp";
pub const MCP_REVISION_2026_07_28: &str = "2026-07-28";
const MCP_TOOLS_CALL: &str = "tools/call";
const MCP_TOOL_KIND: &str = "mcp_tool";

const META_PROTOCOL_VERSION: &str = "io.modelcontextprotocol/protocolVersion";
const META_CLIENT_INFO: &str = "io.modelcontextprotocol/clientInfo";
const META_CLIENT_CAPABILITIES: &str = "io.modelcontextprotocol/clientCapabilities";
const META_LOG_LEVEL: &str = "io.modelcontextprotocol/logLevel";
const TRACE_KEYS: [&str; 3] = ["traceparent", "tracestate", "baggage"];

/// A validated Tarif Action IR value.
///
/// Fields are intentionally private and `Action` does not implement
/// `Deserialize`. This makes the strict Tarif ingestion paths the construction
/// boundary for canonicalizable actions, preventing callers from bypassing
/// JCS/I-JSON number normalization with arbitrary `serde_json::Value`s.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Action {
    schema: String,
    protocol: ProtocolDescriptor,
    operation: String,
    target: Target,
    arguments: ActionArguments,
    mcp_context: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ProtocolDescriptor {
    name: String,
    revision: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Target {
    kind: String,
    name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ArgumentState {
    Absent,
    Present,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ActionArguments {
    state: ArgumentState,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<Value>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ActionWire {
    schema: String,
    protocol: ProtocolDescriptorWire,
    operation: String,
    target: TargetWire,
    arguments: ActionArgumentsWire,
    mcp_context: BTreeMap<String, Value>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProtocolDescriptorWire {
    name: String,
    revision: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TargetWire {
    kind: String,
    name: String,
}

#[derive(Deserialize)]
#[serde(tag = "state", rename_all = "lowercase", deny_unknown_fields)]
enum ActionArgumentsWire {
    Absent,
    Present { value: Value },
}

impl From<ActionWire> for Action {
    fn from(wire: ActionWire) -> Self {
        Self {
            schema: wire.schema,
            protocol: ProtocolDescriptor {
                name: wire.protocol.name,
                revision: wire.protocol.revision,
            },
            operation: wire.operation,
            target: Target {
                kind: wire.target.kind,
                name: wire.target.name,
            },
            arguments: match wire.arguments {
                ActionArgumentsWire::Absent => ActionArguments {
                    state: ArgumentState::Absent,
                    value: None,
                },
                ActionArgumentsWire::Present { value } => ActionArguments {
                    state: ArgumentState::Present,
                    value: Some(value),
                },
            },
            mcp_context: wire.mcp_context,
        }
    }
}

impl Action {
    #[must_use]
    pub fn schema(&self) -> &str {
        &self.schema
    }

    #[must_use]
    pub fn protocol(&self) -> &ProtocolDescriptor {
        &self.protocol
    }

    #[must_use]
    pub fn operation(&self) -> &str {
        &self.operation
    }

    #[must_use]
    pub fn target(&self) -> &Target {
        &self.target
    }

    #[must_use]
    pub fn arguments(&self) -> &ActionArguments {
        &self.arguments
    }

    #[must_use]
    pub fn mcp_context(&self) -> &BTreeMap<String, Value> {
        &self.mcp_context
    }

    fn validate_contract(&self) -> Result<(), ActionError> {
        if self.schema != ACTION_SCHEMA_V1 {
            return Err(ActionError::UnsupportedActionSchema(self.schema.clone()));
        }
        if self.protocol.name != MCP_PROTOCOL_NAME {
            return Err(ActionError::InvalidActionIr(format!(
                "protocol.name must be {MCP_PROTOCOL_NAME}"
            )));
        }
        if self.protocol.revision != MCP_REVISION_2026_07_28 {
            return Err(ActionError::UnsupportedProtocolRevision(
                self.protocol.revision.clone(),
            ));
        }
        if self.operation != MCP_TOOLS_CALL {
            return Err(ActionError::UnsupportedMethod(self.operation.clone()));
        }
        if self.target.kind != MCP_TOOL_KIND {
            return Err(ActionError::InvalidActionIr(format!(
                "target.kind must be {MCP_TOOL_KIND}"
            )));
        }
        validate_tool_name(&self.target.name)?;

        match (&self.arguments.state, &self.arguments.value) {
            (ArgumentState::Absent, None) => {}
            (ArgumentState::Present, Some(Value::Object(_))) => {}
            (ArgumentState::Present, Some(_)) => return Err(ActionError::ArgumentsNotObject),
            (ArgumentState::Absent, Some(_)) | (ArgumentState::Present, None) => {
                return Err(ActionError::InvalidActionIr(
                    "arguments state/value combination is inconsistent".to_owned(),
                ));
            }
        }

        validate_canonical_context(&self.mcp_context)?;
        Ok(())
    }
}

impl ProtocolDescriptor {
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn revision(&self) -> &str {
        &self.revision
    }
}

impl Target {
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl ActionArguments {
    #[must_use]
    pub const fn state(&self) -> ArgumentState {
        self.state
    }

    #[must_use]
    pub fn value(&self) -> Option<&Value> {
        self.value.as_ref()
    }
}

/// Normalize a strict MCP 2026-07-28 baseline `tools/call` JSON-RPC request.
///
/// JSON-RPC correlation fields are validated but are not part of action
/// authority. Supported server-visible MCP `_meta` fields are bound as
/// untrusted execution context. Trace-only metadata is excluded.
pub fn normalize_mcp_tools_call(
    raw_request: &str,
    protocol_revision: &str,
) -> Result<Action, ActionError> {
    if protocol_revision != MCP_REVISION_2026_07_28 {
        return Err(ActionError::UnsupportedProtocolRevision(
            protocol_revision.to_owned(),
        ));
    }

    let value = parse_strict_json(raw_request)?;
    let request = value
        .as_object()
        .ok_or(ActionError::ExpectedObject("request"))?;

    for key in request.keys() {
        if !matches!(key.as_str(), "jsonrpc" | "id" | "method" | "params") {
            return Err(ActionError::UnknownTopLevelField(key.clone()));
        }
    }

    match request.get("jsonrpc").and_then(Value::as_str) {
        Some("2.0") => {}
        _ => return Err(ActionError::InvalidJsonRpcVersion),
    }

    match request.get("id") {
        Some(Value::String(_)) | Some(Value::Number(_)) => {}
        _ => return Err(ActionError::InvalidRequestId),
    }

    let method = request
        .get("method")
        .and_then(Value::as_str)
        .ok_or(ActionError::MissingField("method"))?;
    if method != MCP_TOOLS_CALL {
        return Err(ActionError::UnsupportedMethod(method.to_owned()));
    }

    let params = request
        .get("params")
        .ok_or(ActionError::MissingField("params"))?
        .as_object()
        .ok_or(ActionError::ExpectedObject("params"))?;

    validate_param_keys(params)?;

    let tool_name = params
        .get("name")
        .and_then(Value::as_str)
        .ok_or(ActionError::MissingField("params.name"))?;
    validate_tool_name(tool_name)?;

    let arguments = match params.get("arguments") {
        None => ActionArguments {
            state: ArgumentState::Absent,
            value: None,
        },
        Some(Value::Object(object)) => ActionArguments {
            state: ArgumentState::Present,
            value: Some(Value::Object(object.clone())),
        },
        Some(_) => return Err(ActionError::ArgumentsNotObject),
    };

    let mcp_context = match params.get("_meta") {
        None => BTreeMap::new(),
        Some(meta) => normalize_meta(meta, protocol_revision)?,
    };

    let action = Action {
        schema: ACTION_SCHEMA_V1.to_owned(),
        protocol: ProtocolDescriptor {
            name: MCP_PROTOCOL_NAME.to_owned(),
            revision: protocol_revision.to_owned(),
        },
        operation: MCP_TOOLS_CALL.to_owned(),
        target: Target {
            kind: MCP_TOOL_KIND.to_owned(),
            name: tool_name.to_owned(),
        },
        arguments,
        mcp_context,
    };
    action.validate_contract()?;
    Ok(action)
}

/// Parse a serialized Action IR through the same duplicate-safe JSON boundary.
pub fn parse_action_ir(raw_action: &str) -> Result<Action, ActionError> {
    let value = parse_strict_json(raw_action)?;
    let object = value
        .as_object()
        .ok_or_else(|| ActionError::InvalidActionIr("action must be an object".to_owned()))?;

    let schema = object
        .get("schema")
        .and_then(Value::as_str)
        .ok_or_else(|| ActionError::InvalidActionIr("missing string schema".to_owned()))?;
    if schema != ACTION_SCHEMA_V1 {
        return Err(ActionError::UnsupportedActionSchema(schema.to_owned()));
    }

    let arguments = object
        .get("arguments")
        .and_then(Value::as_object)
        .ok_or_else(|| ActionError::InvalidActionIr("arguments must be an object".to_owned()))?;
    validate_action_arguments_wire_shape(arguments)?;

    let wire: ActionWire = serde_json::from_value(value)
        .map_err(|error| ActionError::InvalidActionIr(error.to_string()))?;
    let action = Action::from(wire);
    action.validate_contract()?;
    Ok(action)
}

/// Produce RFC 8785 JCS bytes for a validated Action IR.
///
/// `Action` values are sealed behind Tarif's strict ingestion boundary; callers
/// cannot construct or deserialize an arbitrary action with unnormalized JSON
/// numbers and then bypass that boundary here.
pub fn canonical_bytes(action: &Action) -> Result<Vec<u8>, ActionError> {
    action.validate_contract()?;
    serde_json_canonicalizer::to_vec(action)
        .map_err(|error| ActionError::Canonicalization(error.to_string()))
}

/// Normalize and canonicalize in one fail-closed operation.
pub fn normalize_and_canonicalize(
    raw_request: &str,
    protocol_revision: &str,
) -> Result<(Action, Vec<u8>), ActionError> {
    let action = normalize_mcp_tools_call(raw_request, protocol_revision)?;
    let bytes = canonical_bytes(&action)?;
    Ok((action, bytes))
}

fn validate_action_arguments_wire_shape(arguments: &Map<String, Value>) -> Result<(), ActionError> {
    let state = arguments
        .get("state")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            ActionError::InvalidActionIr("arguments.state must be a string".to_owned())
        })?;

    match state {
        "absent" => {
            for key in arguments.keys() {
                if key != "state" {
                    return Err(ActionError::InvalidActionIr(format!(
                        "unsupported arguments field for absent state: {key}"
                    )));
                }
            }
        }
        "present" => {
            if !arguments.contains_key("value") {
                return Err(ActionError::InvalidActionIr(
                    "arguments.value is required when state is present".to_owned(),
                ));
            }
            for key in arguments.keys() {
                if !matches!(key.as_str(), "state" | "value") {
                    return Err(ActionError::InvalidActionIr(format!(
                        "unsupported arguments field for present state: {key}"
                    )));
                }
            }
        }
        _ => {
            return Err(ActionError::InvalidActionIr(format!(
                "unsupported arguments state: {state}"
            )));
        }
    }

    Ok(())
}

fn validate_param_keys(params: &Map<String, Value>) -> Result<(), ActionError> {
    for key in params.keys() {
        match key.as_str() {
            "name" | "arguments" | "_meta" => {}
            "inputResponses" | "requestState" => {
                return Err(ActionError::UnsupportedExecutionState(key.clone()));
            }
            _ => return Err(ActionError::UnsupportedExecutionState(key.clone())),
        }
    }
    Ok(())
}

fn normalize_meta(
    meta: &Value,
    protocol_revision: &str,
) -> Result<BTreeMap<String, Value>, ActionError> {
    let object = meta.as_object().ok_or(ActionError::MetaNotObject)?;
    let mut context = BTreeMap::new();

    for (key, value) in object {
        match key.as_str() {
            META_PROTOCOL_VERSION => {
                let observed = value
                    .as_str()
                    .ok_or_else(|| ActionError::InvalidMetaField {
                        field: key.clone(),
                        expected: "a string",
                    })?;
                if observed != protocol_revision {
                    return Err(ActionError::ProtocolVersionMismatch {
                        expected: protocol_revision.to_owned(),
                        observed: observed.to_owned(),
                    });
                }
            }
            META_CLIENT_INFO | META_CLIENT_CAPABILITIES => {
                if !value.is_object() {
                    return Err(ActionError::InvalidMetaField {
                        field: key.clone(),
                        expected: "an object",
                    });
                }
                context.insert(key.clone(), value.clone());
            }
            META_LOG_LEVEL => {
                if !value.is_string() {
                    return Err(ActionError::InvalidMetaField {
                        field: key.clone(),
                        expected: "a string",
                    });
                }
                context.insert(key.clone(), value.clone());
            }
            "inputResponses" | "requestState" => {
                return Err(ActionError::UnsupportedExecutionState(key.clone()));
            }
            trace if TRACE_KEYS.contains(&trace) => {
                if !value.is_string() {
                    return Err(ActionError::InvalidMetaField {
                        field: key.clone(),
                        expected: "a string",
                    });
                }
            }
            _ => return Err(ActionError::UnknownMetaField(key.clone())),
        }
    }

    Ok(context)
}

fn validate_canonical_context(context: &BTreeMap<String, Value>) -> Result<(), ActionError> {
    for (key, value) in context {
        match key.as_str() {
            META_CLIENT_INFO | META_CLIENT_CAPABILITIES if value.is_object() => {}
            META_LOG_LEVEL if value.is_string() => {}
            META_CLIENT_INFO | META_CLIENT_CAPABILITIES => {
                return Err(ActionError::InvalidMetaField {
                    field: key.clone(),
                    expected: "an object",
                });
            }
            META_LOG_LEVEL => {
                return Err(ActionError::InvalidMetaField {
                    field: key.clone(),
                    expected: "a string",
                });
            }
            _ => return Err(ActionError::UnknownMetaField(key.clone())),
        }
    }
    Ok(())
}

fn validate_tool_name(name: &str) -> Result<(), ActionError> {
    let bytes = name.as_bytes();
    let supported = (1..=128).contains(&bytes.len())
        && bytes
            .iter()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.'));
    if supported {
        Ok(())
    } else {
        Err(ActionError::InvalidToolName(name.to_owned()))
    }
}
