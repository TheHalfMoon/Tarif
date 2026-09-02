use tarif_core::{
    ACTION_SCHEMA_V1, ActionError, MCP_REVISION_2026_07_28, canonical_bytes,
    normalize_and_canonicalize, normalize_mcp_tools_call, parse_action_ir,
};

const REQUIRED_META: &str = r#""_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":{}}"#;

fn bare_request(params: &str) -> String {
    format!(r#"{{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{params}}}"#)
}

fn request(params: &str) -> String {
    let mut params = params.to_owned();

    if let Some(meta_start) = params.find(r#""_meta":{"#) {
        let insert_at = meta_start + r#""_meta":{"#.len();
        let mut required = Vec::new();
        if !params.contains(r#""io.modelcontextprotocol/protocolVersion""#) {
            required.push(r#""io.modelcontextprotocol/protocolVersion":"2026-07-28""#);
        }
        if !params.contains(r#""io.modelcontextprotocol/clientCapabilities""#) {
            required.push(r#""io.modelcontextprotocol/clientCapabilities":{}"#);
        }
        if !required.is_empty() {
            let mut insertion = required.join(",");
            if params.as_bytes().get(insert_at) != Some(&b'}') {
                insertion.push(',');
            }
            params.insert_str(insert_at, &insertion);
        }
    } else {
        let insert_at = params.rfind('}').expect("test params must be an object");
        params.insert_str(insert_at, &format!(",{REQUIRED_META}"));
    }

    bare_request(&params)
}

fn canonical(raw: &str) -> Vec<u8> {
    normalize_and_canonicalize(raw, MCP_REVISION_2026_07_28)
        .expect("request should normalize")
        .1
}

#[test]
fn object_order_and_whitespace_canonicalize_identically() {
    let first = request(r#"{"name":"search","arguments":{"b":2,"a":1}}"#);
    let second = r#"{ "params": { "_meta": { "io.modelcontextprotocol/clientCapabilities": {}, "io.modelcontextprotocol/protocolVersion": "2026-07-28" }, "arguments": { "a": 1, "b": 2 }, "name": "search" }, "method": "tools/call", "id": 99, "jsonrpc": "2.0" }"#;
    assert_eq!(canonical(&first), canonical(second));
}

#[test]
fn repeated_canonicalization_is_deterministic() {
    let raw = request(r#"{"name":"search","arguments":{"q":"otters"}}"#);
    let action = normalize_mcp_tools_call(&raw, MCP_REVISION_2026_07_28).unwrap();
    assert_eq!(
        canonical_bytes(&action).unwrap(),
        canonical_bytes(&action).unwrap()
    );
}

#[test]
fn nested_duplicate_keys_fail_before_map_collapse() {
    let raw = request(r#"{"name":"search","arguments":{"nested":{"x":1,"x":2}}}"#);
    let error = normalize_mcp_tools_call(&raw, MCP_REVISION_2026_07_28).unwrap_err();
    assert_eq!(error.code(), "duplicate_json_key");
    assert!(matches!(error, ActionError::DuplicateJsonKey(key) if key == "x"));
}

#[test]
fn malformed_json_fails_closed() {
    let error = normalize_mcp_tools_call("{", MCP_REVISION_2026_07_28).unwrap_err();
    assert_eq!(error.code(), "invalid_json");
}

#[test]
fn jcs_number_semantics_are_applied_before_action_construction() {
    let raw = request(r#"{"name":"search","arguments":{"n":333333333.33333329}}"#);
    let bytes = canonical(&raw);
    let text = String::from_utf8(bytes).unwrap();
    assert!(text.contains("333333333.3333333"));
}

#[test]
fn high_precision_integer_is_normalized_before_action_construction() {
    let raw = request(r#"{"name":"search","arguments":{"n":9007199254740993}}"#);
    let bytes = canonical(&raw);
    let text = String::from_utf8(bytes).unwrap();
    assert!(text.contains("9007199254740992"));
}

#[test]
fn jcs_incompatible_number_fails_closed() {
    let raw = request(r#"{"name":"search","arguments":{"n":1e400}}"#);
    let error = normalize_mcp_tools_call(&raw, MCP_REVISION_2026_07_28).unwrap_err();
    assert_eq!(error.code(), "invalid_json");
}

#[test]
fn unicode_is_not_normalized() {
    let precomposed = request(r#"{"name":"search","arguments":{"q":"é"}}"#);
    let decomposed = request(r#"{"name":"search","arguments":{"q":"é"}}"#);
    assert_ne!(canonical(&precomposed), canonical(&decomposed));
}

#[test]
fn tool_names_are_case_sensitive() {
    let lower = request(r#"{"name":"search"}"#);
    let upper = request(r#"{"name":"Search"}"#);
    assert_ne!(canonical(&lower), canonical(&upper));
}

#[test]
fn non_profile_tool_names_fail_closed() {
    let raw = request(r#"{"name":"search tool"}"#);
    let error = normalize_mcp_tools_call(&raw, MCP_REVISION_2026_07_28).unwrap_err();
    assert_eq!(error.code(), "invalid_tool_name");
}

#[test]
fn omitted_arguments_are_distinct_from_present_empty_object() {
    let absent = request(r#"{"name":"search"}"#);
    let present = request(r#"{"name":"search","arguments":{}}"#);
    assert_ne!(canonical(&absent), canonical(&present));
}

#[test]
fn arguments_must_be_an_object() {
    let raw = request(r#"{"name":"search","arguments":[1,2]}"#);
    let error = normalize_mcp_tools_call(&raw, MCP_REVISION_2026_07_28).unwrap_err();
    assert_eq!(error.code(), "arguments_not_object");
}

#[test]
fn mrtr_state_fails_closed() {
    for field in ["inputResponses", "requestState"] {
        let raw = request(&format!(r#"{{"name":"search","{field}":{{}}}}"#));
        let error = normalize_mcp_tools_call(&raw, MCP_REVISION_2026_07_28).unwrap_err();
        assert_eq!(error.code(), "unsupported_execution_state");
    }
}

#[test]
fn task_augmented_state_fails_closed() {
    let raw = request(r#"{"name":"search","task":{"id":"task-1"}}"#);
    let error = normalize_mcp_tools_call(&raw, MCP_REVISION_2026_07_28).unwrap_err();
    assert_eq!(error.code(), "unsupported_execution_state");
}

#[test]
fn missing_request_meta_fails_closed() {
    let raw = bare_request(r#"{"name":"search"}"#);
    let error = normalize_mcp_tools_call(&raw, MCP_REVISION_2026_07_28).unwrap_err();
    assert_eq!(error.code(), "missing_field");
    assert!(matches!(error, ActionError::MissingField("params._meta")));
}

#[test]
fn missing_request_protocol_version_fails_closed() {
    let raw = bare_request(
        r#"{"name":"search","_meta":{"io.modelcontextprotocol/clientCapabilities":{}}}"#,
    );
    let error = normalize_mcp_tools_call(&raw, MCP_REVISION_2026_07_28).unwrap_err();
    assert_eq!(error.code(), "missing_field");
    assert!(matches!(
        error,
        ActionError::MissingField("params._meta.io.modelcontextprotocol/protocolVersion")
    ));
}

#[test]
fn missing_request_client_capabilities_fails_closed() {
    let raw = bare_request(
        r#"{"name":"search","_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28"}}"#,
    );
    let error = normalize_mcp_tools_call(&raw, MCP_REVISION_2026_07_28).unwrap_err();
    assert_eq!(error.code(), "missing_field");
    assert!(matches!(
        error,
        ActionError::MissingField("params._meta.io.modelcontextprotocol/clientCapabilities")
    ));
}

#[test]
fn unknown_meta_fails_closed() {
    let raw = request(r#"{"name":"search","_meta":{"vendor/example":true}}"#);
    let error = normalize_mcp_tools_call(&raw, MCP_REVISION_2026_07_28).unwrap_err();
    assert_eq!(error.code(), "unknown_meta_field");
}

#[test]
fn envelope_protocol_version_must_match() {
    let raw = request(
        r#"{"name":"search","_meta":{"io.modelcontextprotocol/protocolVersion":"2025-11-25"}}"#,
    );
    let error = normalize_mcp_tools_call(&raw, MCP_REVISION_2026_07_28).unwrap_err();
    assert_eq!(error.code(), "protocol_version_mismatch");
}

#[test]
fn matching_envelope_protocol_version_is_bound_by_protocol_revision_only() {
    let raw = request(
        r#"{"name":"search","_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28"}}"#,
    );
    let action = normalize_mcp_tools_call(&raw, MCP_REVISION_2026_07_28).unwrap();
    assert!(
        !action
            .mcp_context()
            .contains_key("io.modelcontextprotocol/protocolVersion")
    );
    assert!(
        action
            .mcp_context()
            .contains_key("io.modelcontextprotocol/clientCapabilities")
    );
    assert_eq!(action.protocol().revision(), MCP_REVISION_2026_07_28);
}

#[test]
fn unsupported_revision_fails_closed() {
    let raw = request(r#"{"name":"search"}"#);
    let error = normalize_mcp_tools_call(&raw, "2025-11-25").unwrap_err();
    assert_eq!(error.code(), "unsupported_protocol_revision");
}

#[test]
fn client_info_is_bound_but_not_promoted_to_identity() {
    let one = request(
        r#"{"name":"search","_meta":{"io.modelcontextprotocol/clientInfo":{"name":"one","version":"1"}}}"#,
    );
    let two = request(
        r#"{"name":"search","_meta":{"io.modelcontextprotocol/clientInfo":{"name":"two","version":"1"}}}"#,
    );
    let action = normalize_mcp_tools_call(&one, MCP_REVISION_2026_07_28).unwrap();
    assert!(
        action
            .mcp_context()
            .contains_key("io.modelcontextprotocol/clientInfo")
    );
    assert_ne!(canonical(&one), canonical(&two));
}

#[test]
fn client_capabilities_are_bound_as_untrusted_context() {
    let one = request(
        r#"{"name":"search","_meta":{"io.modelcontextprotocol/clientCapabilities":{"sampling":{}}}}"#,
    );
    let two = request(
        r#"{"name":"search","_meta":{"io.modelcontextprotocol/clientCapabilities":{"roots":{}}}}"#,
    );
    let action = normalize_mcp_tools_call(&one, MCP_REVISION_2026_07_28).unwrap();
    assert!(
        action
            .mcp_context()
            .contains_key("io.modelcontextprotocol/clientCapabilities")
    );
    assert_ne!(canonical(&one), canonical(&two));
}

#[test]
fn log_level_is_bound_as_untrusted_context() {
    let one = request(r#"{"name":"search","_meta":{"io.modelcontextprotocol/logLevel":"info"}}"#);
    let two = request(r#"{"name":"search","_meta":{"io.modelcontextprotocol/logLevel":"debug"}}"#);
    let action = normalize_mcp_tools_call(&one, MCP_REVISION_2026_07_28).unwrap();
    assert!(
        action
            .mcp_context()
            .contains_key("io.modelcontextprotocol/logLevel")
    );
    assert_ne!(canonical(&one), canonical(&two));
}

#[test]
fn invalid_supported_meta_types_fail_closed() {
    for (field, value) in [
        ("io.modelcontextprotocol/clientInfo", "true"),
        ("io.modelcontextprotocol/clientCapabilities", "[]"),
        ("io.modelcontextprotocol/logLevel", "{}"),
    ] {
        let raw = request(&format!(
            r#"{{"name":"search","_meta":{{"{field}":{value}}}}}"#
        ));
        let error = normalize_mcp_tools_call(&raw, MCP_REVISION_2026_07_28).unwrap_err();
        assert_eq!(error.code(), "invalid_meta_field");
    }
}

#[test]
fn trace_context_does_not_become_action_authority() {
    for key in ["traceparent", "tracestate", "baggage"] {
        let one = request(&format!(r#"{{"name":"search","_meta":{{"{key}":"one"}}}}"#));
        let two = request(&format!(r#"{{"name":"search","_meta":{{"{key}":"two"}}}}"#));
        assert_eq!(canonical(&one), canonical(&two));
    }
}

#[test]
fn unknown_action_schema_is_rejected() {
    let raw = r#"{"schema":"tarif.action/v2","protocol":{"name":"mcp","revision":"2026-07-28"},"operation":"tools/call","target":{"kind":"mcp_tool","name":"search"},"arguments":{"state":"absent"},"mcp_context":{}}"#;
    let error = parse_action_ir(raw).unwrap_err();
    assert_eq!(error.code(), "unsupported_action_schema");
}

#[test]
fn serialized_action_ir_requires_client_capabilities_context() {
    let raw = r#"{"schema":"tarif.action/v1","protocol":{"name":"mcp","revision":"2026-07-28"},"operation":"tools/call","target":{"kind":"mcp_tool","name":"search"},"arguments":{"state":"absent"},"mcp_context":{}}"#;
    let error = parse_action_ir(raw).unwrap_err();
    assert_eq!(error.code(), "missing_field");
    assert!(matches!(
        error,
        ActionError::MissingField("mcp_context.io.modelcontextprotocol/clientCapabilities")
    ));
}

#[test]
fn generated_action_ir_round_trips_through_strict_parser() {
    let raw = request(r#"{"name":"search","arguments":{"q":"otters"}}"#);
    let action = normalize_mcp_tools_call(&raw, MCP_REVISION_2026_07_28).unwrap();
    assert_eq!(action.schema(), ACTION_SCHEMA_V1);
    let bytes = canonical_bytes(&action).unwrap();
    let reparsed = parse_action_ir(std::str::from_utf8(&bytes).unwrap()).unwrap();
    assert_eq!(action, reparsed);
}

#[test]
fn unpaired_unicode_surrogate_is_rejected() {
    let raw = request(r#"{"name":"search","arguments":{"q":"\uD800"}}"#);
    let error = normalize_mcp_tools_call(&raw, MCP_REVISION_2026_07_28).unwrap_err();
    assert_eq!(error.code(), "invalid_json");
}

#[test]
fn excessive_nesting_is_rejected_by_the_strict_parser_boundary() {
    let nesting = 200;
    let arrays = "[".repeat(nesting) + "0" + &"]".repeat(nesting);
    let raw = request(&format!(
        r#"{{"name":"search","arguments":{{"x":{arrays}}}}}"#
    ));
    let error = normalize_mcp_tools_call(&raw, MCP_REVISION_2026_07_28).unwrap_err();
    assert_eq!(error.code(), "invalid_json");
}

#[test]
fn unknown_top_level_fields_fail_closed() {
    let raw =
        r#"{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"search"},"extra":true}"#;
    let error = normalize_mcp_tools_call(raw, MCP_REVISION_2026_07_28).unwrap_err();
    assert_eq!(error.code(), "unknown_top_level_field");
}
