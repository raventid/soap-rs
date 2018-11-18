extern crate soap;
extern crate roxmltree;
extern crate codegen;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::env;
use codegen::Scope;

use soap::Wsdl;

fn main() {
    // Parse WSDL from file
    let wsdl = match Wsdl::load_from_file("examples/hello_world/hello.wsdl") {
        Ok(v) => v,
        Err(e) => panic!("Error: {}", e),
    };

    // Extract XSD messages information
    let messages = wsdl.messages;

    let mut scope = Scope::new();

    messages.iter().for_each(|message| {
        let structure = scope.new_struct(&message.name);

        match &message.documentation {
            Some(doc) => structure.doc(doc.text.as_str()),
            None => structure,
        };

        message.parts.iter().for_each(|part| {
            let type_info = match part.part_type {
                Some(ref val) => val.local_name.clone(),
                None => "String".to_string(), // if no type info - then String
            };

            structure.field(&part.name, type_info);
        });
    });

    let target_file = env::current_dir().unwrap().join("examples/hello_world/").join("codegen.rs");
    print_codegen(&scope, Some(target_file)).expect("Error while printing code.");
}

fn print_codegen(scope: &Scope, file: Option<PathBuf>) -> Result<(), std::io::Error> {
    match file {
        None => println!("Scope: {:#?}", scope),
        Some(f) => {
            File::create(f)?.write(scope.to_string().as_bytes())?;
        }
    }

    Ok(())
}
