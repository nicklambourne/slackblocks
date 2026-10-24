use serde_json::json;
use slackblocks::*;

#[test]
fn native_prompt_fields_roundtrip_and_copy() -> Result<(), ValidationError> {
    let button = ButtonElement::builder()
        .text("Ask")
        .agent_prompt("Ask Slackbot")
        .agent_prompt_display("x".repeat(80))
        .build()?;
    assert_eq!(button.agent_prompt(), Some("Ask Slackbot"));
    assert_eq!(
        ButtonElement::try_from(serde_json::to_value(&button).unwrap())?,
        button
    );
    assert_eq!(
        serde_json::from_str::<ButtonElement>(&serde_json::to_string(&button).unwrap()).unwrap(),
        button
    );
    assert_eq!(
        button
            .clone()
            .into_builder()
            .agent_prompt("Changed")
            .build()?
            .agent_prompt(),
        Some("Changed")
    );
    assert!(
        button
            .clone()
            .into_builder()
            .clear_agent_prompt()
            .build()
            .is_err()
    );
    let cleared = button
        .into_builder()
        .clear_agent_prompt_display()
        .clear_agent_prompt()
        .build()?;
    assert_eq!(cleared.agent_prompt(), None);
    assert_eq!(cleared.agent_prompt_display(), None);
    Ok(())
}

#[test]
fn checked_ingress_rejects_non_string_fields() {
    for field in ["agent_prompt", "agent_prompt_display"] {
        let mut wire = json!({"type":"button","text":{"type":"plain_text","text":"Ask"}});
        wire[field] = json!(1);
        assert_eq!(
            ButtonElement::try_from(wire).unwrap_err().category(),
            ErrorCategory::TypeMismatch
        );
    }
}
