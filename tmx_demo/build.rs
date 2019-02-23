use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("static.rs");
    let src_path = Path::new(&project_dir).join("resources");
    let mut f = File::create(&dest_path).unwrap();

    f.write(b"use std::collections::HashMap;\n").unwrap();
    f.write(b"\n").unwrap();
    f.write(b"pub fn static_content() -> HashMap<&'static str, &'static[u8]>{\n").unwrap();
    f.write(b"  let mut resources = HashMap::new();\n").unwrap();

    for entry in std::fs::read_dir(src_path).unwrap() {
        let entry = entry.unwrap();
        let include_file = format!("  resources.insert(\"{}\", include_bytes!(\"{}/resources/{}\").as_ref());\n",entry.file_name().to_str().unwrap(), project_dir, entry.file_name().to_str().unwrap());
        f.write(include_file.as_bytes()).unwrap();
    }

    f.write(b"  resources\n").unwrap();
    f.write(b"}\n").unwrap();
}