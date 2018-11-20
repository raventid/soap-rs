use types::*;


        let response = match roxmltree::Document::parse(&content) {
            Ok(v) => v,
            Err(e) => {
                println!("Error: {}.", e);
                process::exit(1);
            }
        };
    


                                let firstName = response.get(firstName);
                              


                                let greeting = response.get(greeting);
                              