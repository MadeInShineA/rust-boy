use std::env;
use std::fmt::Write;
use std::fs;
use std::path::Path;

fn create_sm83_tests() {
    let sm83_test_folder_path = Path::new("./tests/sm83");

    let out_dir = env::var("OUT_DIR").unwrap();
    let destination = Path::new(&out_dir).join("sm83_tests.rs");

    let mut result = String::new();

    for sm83_test_folder_entry_result in fs::read_dir(sm83_test_folder_path).unwrap() {
        let sm83_test_folder_entry = sm83_test_folder_entry_result.unwrap();
        let file_path = sm83_test_folder_entry.path().canonicalize().unwrap();

        if file_path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }

        // "01.json" -> "sm83_01", "cb 40.json" -> "sm83_cb_40"
        let stem = file_path.file_stem().unwrap().to_str().unwrap();
        let test_name: String = format!(
            "sm83_{}",
            stem.chars()
                .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
                .collect::<String>()
        );

        let file_path_str = file_path.into_string().unwrap();

        writeln!(result, "create_sm83_test!({test_name},{file_path_str:?});").unwrap();
    }

    fs::write(destination, result).unwrap();

    println!("cargo:rerun-if-changed=tests/sm83");
}
fn main() {
    create_sm83_tests();
}
