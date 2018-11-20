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

    // Code generation for types
    let mut types_scope = Scope::new();

    // Types generation
    messages.iter().for_each(|message| {
        let structure = types_scope.new_struct(&message.name);

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

    // Code generation for parsers
    let mut parsers_scope = Scope::new();

    // Include xmltree and types
    parsers_scope.import("types", "*");

    parsers_scope.raw(r#"
        let response = match roxmltree::Document::parse(&content) {
            Ok(v) => v,
            Err(e) => {
                println!("Error: {}.", e);
                process::exit(1);
            }
        };
    "#);

    messages.iter().for_each(|message| {
        message.parts.iter().for_each(|part| {
            parsers_scope.raw(&format!(r#"
                                let {field} = response.get({field});
                              "#,
                                      field=part.name));
        });
    });

    let types_file = env::current_dir().unwrap().join("examples/hello_world/").join("types.rs");
    print_codegen(&types_scope, Some(types_file)).expect("Error while printing types");

    let parsers_file = env::current_dir().unwrap().join("examples/hello_world/").join("parsers.rs");
    print_codegen(&parsers_scope, Some(parsers_file)).expect("Error while printing parsers");
}

fn print_codegen(types_scope: &Scope, file: Option<PathBuf>) -> Result<(), std::io::Error> {
    match file {
        None => println!("Scope: {:#?}", types_scope),
        Some(f) => {
            File::create(f)?.write(types_scope.to_string().as_bytes())?;
        }
    }

    Ok(())
}
