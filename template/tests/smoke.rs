use std::process::Command;

#[test]
fn scan_json_is_doctor_envelope() {
    let dir = std::env::temp_dir().join("{{bin_name}}-smoke");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("empty.txt"), "").unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_{{bin_name}}"))
        .args(["scan", dir.to_str().unwrap(), "--json"])
        .output()
        .unwrap();
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(v["schema"], "doctor/1");
    assert_eq!(v["findings"].as_array().unwrap().len(), 1);
}
