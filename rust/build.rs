// use std::env;
// use std::fs;
// use std::path::Path;

fn main() {
    // let out_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    // let model_path = Path::new("src/models");
    // let entries = fs::read_dir(model_path).unwrap();

    // let mut content = String::new();

    // for entry in entries {
    //     let entry = entry.unwrap();
    //     let path = entry.path();
    //     let file_stem = path.file_stem().unwrap();

    //     if let Some(file_name_str) = file_stem.to_str() {
    //         if file_name_str == "mod" {
    //             continue;
    //         }
    //         content.push_str(&format!("pub mod {};\n", file_name_str));
    //     } else {
    //         panic!("File name is not valid Unicode: {:?}", file_stem);
    //     }
    // }

    // let dest_path = Path::new(&out_dir).join("./src/models/mod.rs");

    // fs::write(dest_path, content).unwrap();
}
