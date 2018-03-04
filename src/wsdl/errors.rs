use std::io::Error as IoError;
use std::borrow::Cow;

use hyper::Error as HyperError;
use xml::reader::Error as XmlError;

error_chain! {
    foreign_links {
        Io(IoError);
        Http(HyperError);
        Xml(XmlError);
    }

    errors {
        MandatoryAttribute(attribute: String, element: String) {
            description("mandatory attribute")
                display("Attribute `{}` is mandatory for `{}` element", attribute, element)
        }

        InvalidElement(element: String) {
            description("invalid element")
                display("Invalid `{}` element", element)
        }

        MissingElement(element: String) {
            description("missing element")
                display("Required `{}` element is missing from WSDL document", element)
        }
    }
}

impl<'a> From<Cow<'a, str>> for Error {
    fn from(error: Cow<'a, str>) -> Error {
        error.into()
    }
}
