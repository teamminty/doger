#[test]
fn anyhow_macro() {
    assert_eq!(doger::anyhow!("oof").to_string(), "oof");
}