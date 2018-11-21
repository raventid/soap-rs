use codegen::Scope;
use wsdl::schema::WsdlMessage;

pub fn generate_messages(messages: &Vec<WsdlMessage>) -> String {
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

    types_scope.to_string()
}
