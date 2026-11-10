use std::{
    fs,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn evidence_command_refuses_a_forged_display_quote_with_a_retained_hash() {
    let root = std::env::temp_dir().join(format!(
        "agon-evidence-{}-{}",
        std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    ));
    fs::create_dir(&root).unwrap();
    let text = root.join("text.txt");
    let span = root.join("span.json");
    fs::write(&text, "Hello world.").unwrap();
    let mut value = serde_json::json!({"segment_id":"test", "char_start_canonical":0,"char_end_canonical":5,"char_start_raw":0,"char_end_raw":5,"verbatim_quote":"Hello","quote_hash":blake3::hash(b"Hello").to_hex().to_string(),"normalization_version":"0.1.0"});
    fs::write(&span, serde_json::to_vec(&value).unwrap()).unwrap();
    let call = || {
        Command::new(env!("CARGO_BIN_EXE_agon"))
            .arg("verify-evidence")
            .arg("--text")
            .arg(&text)
            .arg("--span")
            .arg(&span)
            .output()
            .unwrap()
    };
    let good = call();
    assert!(good.status.success());
    let data: serde_json::Value = serde_json::from_slice(&good.stdout).unwrap();
    assert_eq!(data["exact"], true);
    value["verbatim_quote"] = serde_json::json!("A different policy position");
    fs::write(&span, serde_json::to_vec(&value).unwrap()).unwrap();
    let bad = call();
    assert_eq!(bad.status.code(), Some(2));
    let data: serde_json::Value = serde_json::from_slice(&bad.stdout).unwrap();
    assert_eq!(data["exact"], false);
    fs::remove_dir_all(root).unwrap();
}
