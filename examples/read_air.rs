extern crate soap;

use std::fs::File;
use std::path::PathBuf;
use std::io::Write;
use std::env;

use soap::Wsdl;

fn main() {
    let tmp_dir = env::current_dir().unwrap().join("examples/");

    let wsdl = match Wsdl::load_from_file("examples/Air.wsdl") {
        Ok(v) => v,
        Err(e) => panic!("Error: {}", e),
    };

    // Text representation.
    print_wsdl(&wsdl, Some(tmp_dir.join("wsdl_air.txt"))).expect("Error while printing WSDL.");
}

fn print_wsdl(wsdl: &Wsdl, file: Option<PathBuf>) -> Result<(), std::io::Error> {
    match file {
        None => println!("WSDL: {:#?}", wsdl),
        Some(f) => {
            let wsdl_str = format!("{:#?}", wsdl);
            File::create(f)?.write_all(wsdl_str.as_bytes())?;
        }
    }

    Ok(())
}
