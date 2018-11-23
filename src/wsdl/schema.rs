use super::errors::*;

use super::http;
use super::file;

use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::namespace::Namespace;
use xml::reader::{EventReader, Events, XmlEvent};

use encoding::all::UTF_8;
use encoding::DecoderTrap;
use encoding::types::decode;

const NS_WSDL: &'static str = "http://schemas.xmlsoap.org/wsdl/";

pub trait Documented {
    fn get_documentation(&self) -> &Option<WsdlDocumentation>;
}

macro_rules! impl_documented {
    ($type:ty) => {
        impl Documented for $type {
            fn get_documentation(&self) -> &Option<WsdlDocumentation> {
                &self.documentation
            }
        }
    }
}

pub trait NamedItem {
    fn get_name(&self) -> &str;
}

macro_rules! impl_named_item {
    ($type:ty) => {
        impl NamedItem for $type {
            fn get_name(&self) -> &str {
                self.name.as_str()
            }
        }
    }
}

#[derive(Debug)]
pub struct Wsdl {
    pub documentation: Option<WsdlDocumentation>,
    pub target_namespace: Option<String>,
    pub types: Vec<WsdlTypes>,
    pub port_types: Vec<WsdlPortType>,
    pub services: Vec<WsdlService>,
    pub bindings: Vec<WsdlBinding>,
    pub messages: Vec<WsdlMessage>,
}

impl_documented!(Wsdl);

#[derive(Debug)]
pub struct WsdlService {
    pub documentation: Option<WsdlDocumentation>,
    pub name: String,
    pub ports: Vec<WsdlPort>,
}

impl_documented!(WsdlService);
impl_named_item!(WsdlService);

#[derive(Debug)]
pub struct WsdlBinding {
    pub documentation: Option<WsdlDocumentation>,
    pub name: String,
    pub port_type: OwnedName,
    pub operations: Vec<WsdlOperationBinding>,
}

impl_documented!(WsdlBinding);
impl_named_item!(WsdlBinding);

#[derive(Debug)]
pub struct WsdlMessage {
    pub documentation: Option<WsdlDocumentation>,
    pub name: String,
    pub parts: Vec<WsdlMessagePart>,
}

impl_documented!(WsdlMessage);
impl_named_item!(WsdlMessage);

#[derive(Debug)]
pub struct WsdlPort {
    pub documentation: Option<WsdlDocumentation>,
    pub name: String,
    pub binding: OwnedName,
}

impl_documented!(WsdlPort);
impl_named_item!(WsdlPort);

#[derive(Debug)]
pub struct WsdlOperationBinding {
    pub documentation: Option<WsdlDocumentation>,
    pub name: String,
    pub input: Option<WsdlInputBinding>,
    pub output: Option<WsdlOutputBinding>,
    pub fault: Option<WsdlFaultBinding>,
}

impl_documented!(WsdlOperationBinding);
impl_named_item!(WsdlOperationBinding);

#[derive(Debug)]
pub struct WsdlMessagePart {
    pub name: String,
    pub element: Option<OwnedName>,
    pub part_type: Option<OwnedName>,
}

#[derive(Debug)]
pub struct WsdlInputBinding {
    pub documentation: Option<WsdlDocumentation>,
    pub text: String,
}

impl_documented!(WsdlInputBinding);

#[derive(Debug)]
pub struct WsdlOutputBinding {
    pub documentation: Option<WsdlDocumentation>,
}

impl_documented!(WsdlOutputBinding);

#[derive(Debug)]
pub struct WsdlFaultBinding {
    pub documentation: Option<WsdlDocumentation>,
    pub name: String,
}

impl_documented!(WsdlFaultBinding);
impl_named_item!(WsdlFaultBinding);

#[derive(Debug)]
pub struct WsdlTypes {
    pub documentation: Option<WsdlDocumentation>,
}

impl_documented!(WsdlTypes);

#[derive(Debug)]
pub struct WsdlPortType {
    pub documentation: Option<WsdlDocumentation>,
    pub name: String,
}

impl_documented!(WsdlPortType);
impl_named_item!(WsdlPortType);

#[derive(Debug)]
pub struct WsdlDocumentation {
    pub text: String
}

impl Wsdl {
    pub fn load_from_url(url: &str) -> Result<Wsdl> {
        let contents = http::get(url)?;
        let decoded_contents = decode_contents(&contents)?;
        Wsdl::parse(&decoded_contents[..])
    }

    pub fn load_from_file(location: &str) -> Result<Wsdl> {
        let contents = file::load(location)?;
        let decoded_contents = decode_contents(&contents)?;
        Wsdl::parse(&decoded_contents[..])
    }

    pub fn parse(decoded_contents: &[u8]) -> Result<Wsdl> {
        let ns_wsdl = Some(NS_WSDL.to_string());
        let parser = EventReader::new(decoded_contents);
        let mut iter = parser.into_iter();

        while let Some(v) = iter.next() {
            match v? {
                XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } if name.namespace == ns_wsdl && name.local_name == "definitions" => {
                    return Wsdl::read(attributes, &mut iter);
                }
                _ => continue,
            }
        }

        Err(ErrorKind::MissingElement("definitions".to_string()).into())
    }

    fn read(attributes: &[OwnedAttribute], mut iter: &mut Events<&[u8]>) -> Result<Wsdl> {
        let ns_wsdl = Some(NS_WSDL.to_string());
        let target_namespace = find_attribute("targetNamespace", attributes);

        let mut depth = 0;
        let mut documentation = None;
        let mut types = Vec::new();
        let mut port_types = Vec::new();
        let mut services = Vec::new();
        let mut bindings = Vec::new();
        let mut messages = Vec::new();

        while let Some(v) = iter.next() {
            match (v?, depth) {
                (XmlEvent::StartElement { ref name, .. }, 0) if name.namespace == ns_wsdl &&
                                                                name.local_name ==
                                                                "documentation" => {
                    documentation = Some(WsdlDocumentation::read(&mut iter)?)
                }
                (XmlEvent::StartElement { ref name, .. }, 0) if name.namespace == ns_wsdl &&
                                                                name.local_name == "import" => unimplemented!(),
                (XmlEvent::StartElement { ref name, .. }, 0) if name.namespace == ns_wsdl &&
                                                                name.local_name == "types" => {
                    types.push(WsdlTypes::read(&mut iter)?)
                }
                (XmlEvent::StartElement {
                     ref name,
                     ref attributes,
                     ..
                 },
                 0) if name.namespace == ns_wsdl && name.local_name == "message" => {
                    messages.push(WsdlMessage::read(attributes, &mut iter)?)
                }
                (XmlEvent::StartElement {
                     ref name,
                     ref attributes,
                     ..
                 },
                 0) if name.namespace == ns_wsdl && name.local_name == "portType" => {
                    port_types.push(WsdlPortType::read(attributes, &mut iter)?)
                }
                (XmlEvent::StartElement {
                     ref name,
                     ref attributes,
                     ref namespace,
                 },
                 0) if name.namespace == ns_wsdl && name.local_name == "binding" => {
                    bindings.push(WsdlBinding::read(attributes, namespace, &mut iter)?)
                }
                (XmlEvent::StartElement {
                     ref name,
                     ref attributes,
                     ..
                 },
                 0) if name.namespace == ns_wsdl && name.local_name == "service" => {
                    services.push(WsdlService::read(attributes, &mut iter)?)
                }
                (XmlEvent::StartElement { .. }, _) => depth += 1,
                (XmlEvent::EndElement { ref name }, 0) if name.namespace == ns_wsdl &&
                                                          name.local_name == "definitions" => break,
                (XmlEvent::EndElement { .. }, _) => depth -= 1,
                _ => continue,
            }
        }

        Ok(Wsdl {
               documentation,
               target_namespace,
               types,
               port_types,
               services,
               bindings,
               messages,
           })
    }
}

impl WsdlService {
    fn read(attributes: &[OwnedAttribute], iter: &mut Events<&[u8]>) -> Result<WsdlService> {
        let ns_wsdl = Some(NS_WSDL.to_string());
        let service_name = find_attribute("name", attributes);

        let mut depth = 0;
        let mut ports = Vec::new();

        for event in iter {
            match (event?, depth) {
                (XmlEvent::StartElement {
                     ref name,
                     ref attributes,
                     ref namespace,
                 },
                 0) if name.namespace == ns_wsdl && name.local_name == "port" => {
                    ports.push(WsdlPort::read(attributes, namespace)?)
                }
                (XmlEvent::StartElement { .. }, _) => depth += 1,
                (XmlEvent::EndElement { ref name }, 0) if name.namespace == ns_wsdl &&
                                                          name.local_name == "service" => break,
                (XmlEvent::EndElement { .. }, _) => depth -= 1,
                _ => continue,
            }
        }

        Ok(WsdlService {
               documentation: None,
               name: service_name
                   .ok_or_else(|| {
                                   ErrorKind::MandatoryAttribute("name".to_string(),
                                                                 "wsdl:service".to_string())
                               })?,
               ports,
           })
    }
}

impl WsdlBinding {
    fn read(attributes: &[OwnedAttribute],
            namespace: &Namespace,
            iter: &mut Events<&[u8]>)
            -> Result<WsdlBinding> {
        let ns_wsdl = Some(NS_WSDL.to_string());
        let mut binding_name = None;
        let mut port_type = None;

        for attr in attributes {
            if attr.name.namespace.is_none() {
                if attr.name.local_name == "name" {
                    binding_name = Some(attr.value.clone());
                } else if attr.name.local_name == "type" {
                    port_type = Some(attr.value.clone());
                }
            }
        }

        let mut port_type: OwnedName = port_type
            .ok_or_else(|| {
                            ErrorKind::MandatoryAttribute("type".to_string(),
                                                          "wsdl:binding".to_string())
                        })?
            .parse()
            .unwrap();

        if let Some(ref pfx) = port_type.prefix {
            port_type.namespace = namespace.get(pfx).map(|x| x.to_string());
        }

        let mut operations = Vec::new();

        for event in iter {
            match event? {
                XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } if name.namespace == ns_wsdl && name.local_name == "operation" => {
                    operations.push(WsdlOperationBinding::read(attributes)?);
                }
                XmlEvent::EndElement { ref name, .. } if name.namespace == ns_wsdl &&
                                                         name.local_name == "binding" => {;
                    return Ok(WsdlBinding {
                            documentation: None,
                            name: binding_name.ok_or_else(|| ErrorKind::MandatoryAttribute("name".to_string(), "wsdl:binding".to_string()))?,
                            port_type,
                            operations
                        });
                }
                _ => continue,
            }
        }

        Err(ErrorKind::InvalidElement("wsdl:binding".to_string()).into())
    }
}

impl WsdlMessage {
    fn read(attributes: &[OwnedAttribute], iter: &mut Events<&[u8]>) -> Result<WsdlMessage> {
        let ns_wsdl = Some(NS_WSDL.to_string());
        let message_name = find_attribute("name", attributes);

        let mut parts = Vec::new();

        for event in iter {
            match event? {
                XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } if name.namespace == ns_wsdl && name.local_name == "part" => {
                    parts.push(WsdlMessagePart::read(attributes)?);
                }
                XmlEvent::EndElement { ref name, .. } if name.namespace == ns_wsdl &&
                                                         name.local_name == "message" => {
                    return Ok(WsdlMessage {
                            documentation: None,
                            name: message_name.ok_or_else(|| ErrorKind::MandatoryAttribute("name".to_string(), "wsdl:message".to_string()))?,
                            parts
                        });
                }
                _ => continue,
            }
        }

        Err(ErrorKind::InvalidElement("wsdl:message".to_string()).into())
    }
}

impl WsdlPort {
    fn read(attributes: &[OwnedAttribute], namespace: &Namespace) -> Result<WsdlPort> {
        let mut name = None;
        let mut binding = None;

        for attr in attributes {
            if attr.name.namespace.is_none() {
                if attr.name.local_name == "name" {
                    name = Some(attr.value.clone());
                } else if attr.name.local_name == "binding" {
                    binding = Some(attr.value.clone());
                }
            }
        }

        let mut binding: OwnedName = binding
            .ok_or_else(|| {
                            ErrorKind::MandatoryAttribute("binding".to_string(),
                                                          "wsdl:port".to_string())
                        })?
            .parse()
            .unwrap();

        if let Some(ref pfx) = binding.prefix {
            binding.namespace = namespace.get(pfx).map(|x| x.to_string());
        }

        Ok(WsdlPort {
               documentation: None,
               name: name.ok_or_else(|| {
                                         ErrorKind::MandatoryAttribute("name".to_string(),
                                                                       "wsdl:port".to_string())
                                     })?,
               binding,
           })
    }
}

impl WsdlOperationBinding {
    fn read(attributes: &[OwnedAttribute]) -> Result<WsdlOperationBinding> {
        let name = find_attribute("name", attributes);
        let internals = attributes
            .iter()
            .map(|attr| attr.value.clone())
            .collect::<Vec<_>>()
            .join("");

        Ok(WsdlOperationBinding {
               documentation: None,
               name: name.ok_or_else(|| {
                                         ErrorKind::MandatoryAttribute("name".to_string(),
                                                                       "wsdl:operation".to_string())
                                     })?,
               input: Some(WsdlInputBinding{documentation: None, text: internals.clone()}),
               output: None,
               fault: None,
           })
    }
}

impl WsdlMessagePart {
    fn read(attributes: &[OwnedAttribute]) -> Result<WsdlMessagePart> {
        let part_name = find_attribute("name", attributes);

        Ok(WsdlMessagePart {
               name: part_name
                   .ok_or_else(|| {
                                   ErrorKind::MandatoryAttribute("name".to_string(),
                                                                 "wsdl:part".to_string())
                               })?,
               element: None,
               part_type: None,
           })
    }
}

impl WsdlTypes {
    fn read(iter: &mut Events<&[u8]>) -> Result<WsdlTypes> {
        let ns_wsdl = Some(NS_WSDL.to_string());

        let mut depth = 0;

        for event in iter {
            match (event?, depth) {
                (XmlEvent::StartElement { .. }, _) => depth += 1,
                (XmlEvent::EndElement { ref name, .. }, 0) if name.namespace == ns_wsdl &&
                                                              name.local_name == "types" => {
                    break;
                }
                (XmlEvent::EndElement { .. }, _) => depth -= 1,
                _ => continue,
            }
        }

        Ok(WsdlTypes { documentation: None })
    }
}

impl WsdlPortType {
    fn read(attributes: &[OwnedAttribute], iter: &mut Events<&[u8]>) -> Result<WsdlPortType> {
        let name = find_attribute("name", attributes)
            .ok_or_else(|| {
                            ErrorKind::MandatoryAttribute("name".to_string(),
                                                          "wsdl:portType".to_string())
                        })?;

        let ns_wsdl = Some(NS_WSDL.to_string());

        let mut depth = 0;

        for event in iter {
            match (event?, depth) {
                (XmlEvent::StartElement { .. }, _) => depth += 1,
                (XmlEvent::EndElement { ref name, .. }, 0) if name.namespace == ns_wsdl &&
                                                              name.local_name == "portType" => {
                    break;
                }
                (XmlEvent::EndElement { .. }, _) => depth -= 1,
                _ => continue,
            }
        }

        Ok(WsdlPortType {
               name,
               documentation: None,
           })
    }
}

impl WsdlDocumentation {
    fn read(iter: &mut Events<&[u8]>) -> Result<WsdlDocumentation> {
        let ns_wsdl = Some(NS_WSDL.to_string());

        let mut depth = 0;

        for event in iter {
            match (event?, depth) {
                (XmlEvent::StartElement { .. }, _) => depth += 1,
                (XmlEvent::EndElement { ref name, .. }, 0) if name.namespace == ns_wsdl &&
                                                              name.local_name ==
                                                              "documentation" => {
                    break;
                }
                (XmlEvent::EndElement { .. }, _) => depth -= 1,
                _ => continue,
            }
        }

        Ok(WsdlDocumentation { text: "placeholder".to_string() })
    }
}

fn decode_contents(bytes: &[u8]) -> Result<Vec<u8>> {
    let (decoded_contents, _) = decode(bytes, DecoderTrap::Replace, UTF_8);
    Ok(decoded_contents?.as_bytes().to_vec())
}

fn find_attribute(name: &str, attributes: &[OwnedAttribute]) -> Option<String> {
    attributes
        .iter()
        .find(|a| a.name.namespace.is_none() && a.name.local_name == name)
        .map(|a| a.value.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_check_depth() {
        let result = Wsdl::parse(r#"<?xml version="1.0" encoding="utf-8"?>
<wsdl:definitions xmlns:wsdl="http://schemas.xmlsoap.org/wsdl/">
    <wsdl:service name="root1">
        <wsdl:service name="wrapped1" />
        <wsdl:service name="wrapped2" />
    </wsdl:service>
    <wsdl:service name="root2">
    </wsdl:service>
</wsdl:definitions>
"#
                                         .as_bytes());

        assert!(result.is_ok());

        let wsdl = result.unwrap();

        assert_eq!(2, wsdl.services.len());
        assert_eq!("root1", wsdl.services[0].name);
        assert_eq!("root2", wsdl.services[1].name);
    }
}
