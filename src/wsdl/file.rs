use std::fs::File;
use std::io::{Read, Result};

pub fn load(location: &str) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();

    File::open(location)?.read_to_end(&mut bytes)?;

    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn load_file_test() {
        let file = get_wsdl_file("etoimik.wsdl").unwrap();
        let result = load(&file);

        assert!(result.is_ok());
        let file_contents = result.unwrap();
        assert!(file_contents.len() > 0);
    }

    #[test]
    fn load_file_fail_test() {
        let file = get_wsdl_file("etoimik2.wsdl").unwrap();
        let result = load(&file);

        assert!(result.is_err());
    }

    fn get_wsdl_file(name: &str) -> Option<String> {
        let mut start = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        start.push("examples");
        start.push(name);

        start.to_str().map(String::from)
    }
}
