extern crate soap;
extern crate roxmltree;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::path::Path;
use std::io::Write;
use std::env;
use std::process;

use soap::Wsdl;

fn main() {
    let tmp_dir = env::current_dir()
        .unwrap();
        // .join("examples/");

    let path = Path::new(&env::current_dir().unwrap().join("service_response.xml")).to_path_buf();


    println!("{:#?}", path);

    let mut content = String::new();
    File::open(path)
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let wsdl = match roxmltree::Document::parse(&content) {
        Ok(v) => v,
        Err(e) => {
            println!("Error: {}.", e);
            process::exit(1);
        }
    };

    // Text representation.
    print_wsdl(&wsdl, Some(tmp_dir.join("wsdl_service.txt"))).expect("Error while printing WSDL.");
}

fn print_wsdl(wsdl: &roxmltree::Document, file: Option<PathBuf>) -> Result<(), std::io::Error> {
    match file {
        None => println!("WSDL: {:#?}", wsdl),
        Some(f) => {
            let wsdl_str = format!("{:#?}", wsdl);
            File::create(f)?.write_all(wsdl_str.as_bytes())?;
        }
    }

    Ok(())
}
