use tarif_core::{ActionError, parse_action_ir};

#[test]
fn serialized_action_ir_unsupported_protocol_has_stable_typed_error() {
    let raw = r#"{"schema":"tarif.action/v1","protocol":{"name":"a2a","revision":"2026-07-28"},"operation":"tools/call","target":{"kind":"mcp_tool","name":"search"},"arguments":{"state":"absent"},"mcp_context":{}}"#;

    let error = parse_action_ir(raw).unwrap_err();

    assert_eq!(error.code(), "unsupported_protocol");
    assert!(matches!(error, ActionError::UnsupportedProtocol(protocol) if protocol == "a2a"));
}
