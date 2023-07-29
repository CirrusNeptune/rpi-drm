use std::fs::Metadata;
use std::path::PathBuf;

fn needs_build(vert_metadata: Metadata, frag_metadata: Metadata, rs_path: &PathBuf) -> bool {
    let path = rs_path.as_path();
    if !path.exists() {
        return true;
    }
    let rs_mod = path.metadata().unwrap().modified().unwrap();
    vert_metadata.modified().unwrap() > rs_mod || frag_metadata.modified().unwrap() > rs_mod
}

fn main() -> Result<(), String> {
    use std::fs;
    use vc4_mesa_compiler::compile_shader;

    let shaders_dir = PathBuf::from("src").join("shaders");
    println!("cargo:rerun-if-changed={}", shaders_dir.display());

    for path_ent in fs::read_dir(&shaders_dir).unwrap() {
        if let Ok(de) = path_ent {
            let vert_metadata = de.metadata().unwrap();
            if !vert_metadata.is_file() {
                continue;
            }
            let vert_path = de.path();
            if let Some(ext) = vert_path.extension() {
                if ext != "vert" {
                    continue;
                }
            } else {
                continue;
            }

            let frag_path = vert_path.with_extension("frag");
            let frag_path_path = frag_path.as_path();
            if !frag_path_path.exists() {
                return Err(format!(
                    "{} does not exist",
                    frag_path_path.to_str().unwrap()
                ));
            }
            let frag_metadata = frag_path_path.metadata().unwrap();
            if !frag_metadata.is_file() {
                return Err(format!(
                    "{} is not a file",
                    frag_path_path.to_str().unwrap()
                ));
            }

            let rs_path = vert_path.with_extension("rs");
            if needs_build(vert_metadata, frag_metadata, &rs_path) {
                match compile_shader(vert_path, frag_path, rs_path) {
                    Err(e) => return Err(e),
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
