#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Read;

    use substreams_antelope_abigen::Abigen;

    #[test]
    fn test_contract_generation() {
        let abi_jsons_dir = "abi/";
        let mut files_tested = 0;
        for entry in fs::read_dir(abi_jsons_dir).expect("failed to read ABI JSON directory") {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == "json" {
                        let path = path.to_string_lossy();
                        let generated = Abigen::new("Contract", &path)
                            .expect(&format!("failed to read ABI JSON for {}", path))
                            .generate()
                            .expect(&format!("failed to generate contract for {}", path))
                            .get_code()
                            .to_string();

                        let golden_file_path =
                            format!("{}{}.rs.golden", abi_jsons_dir, entry.path().file_stem().unwrap().to_string_lossy());
                        let mut golden_contents = String::new();
                        fs::File::open(&golden_file_path)
                            .and_then(|mut file| file.read_to_string(&mut golden_contents))
                            .expect(&format!("failed to read golden file {golden_file_path}"));

                        pretty_assertions::assert_eq!(generated, golden_contents, "\n\n❌ Testing {path} against {golden_file_path}");
                        files_tested += 1;
                    }
                }
            }
        }
        assert!(files_tested > 0, "No files tested");
    }
}