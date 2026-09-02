use tarif_core::parse_action_ir;

fn action_with_arguments(arguments: &str) -> String {
    format!(
        r#"{{"schema":"tarif.action/v1","protocol":{{"name":"mcp","revision":"2026-07-28"}},"operation":"tools/call","target":{{"kind":"mcp_tool","name":"search"}},"arguments":{arguments},"mcp_context":{{}}}}"#
    )
}

#[test]
fn absent_arguments_reject_explicit_value_even_null() {
    let raw = action_with_arguments(r#"{"state":"absent","value":null}"#);
    let error = parse_action_ir(&raw).unwrap_err();
    assert_eq!(error.code(), "invalid_action_ir");
}

#[test]
fn absent_arguments_reject_unknown_fields() {
    let raw = action_with_arguments(r#"{"state":"absent","extra":true}"#);
    let error = parse_action_ir(&raw).unwrap_err();
    assert_eq!(error.code(), "invalid_action_ir");
}

#[test]
fn present_arguments_require_a_value() {
    let raw = action_with_arguments(r#"{"state":"present"}"#);
    let error = parse_action_ir(&raw).unwrap_err();
    assert_eq!(error.code(), "invalid_action_ir");
}

#[test]
fn present_arguments_reject_unknown_fields() {
    let raw = action_with_arguments(r#"{"state":"present","value":{},"extra":true}"#);
    let error = parse_action_ir(&raw).unwrap_err();
    assert_eq!(error.code(), "invalid_action_ir");
}

#[test]
fn present_arguments_reject_non_object_value() {
    let raw = action_with_arguments(r#"{"state":"present","value":null}"#);
    let error = parse_action_ir(&raw).unwrap_err();
    assert_eq!(error.code(), "arguments_not_object");
}
