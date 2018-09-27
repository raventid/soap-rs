# soap-rs

STATUS: WORK IN PROGRESS.

Rust 2018 edition and shiny async/await inside.

SOAP client for Rust programming language.

LICENSE NOTE: Current WSDL implementation is slightly modified version of https://github.com/jaxx/wsdl-rs. WSDL parser will be changed, but it's not a high priority now.

This project is not intended to be used in production. It is more for a learning purpose, personally to me it is both Rust and SOAP, what I'm going to explore.

Client is planned to support SOAP 1.2 and SOAP 1.1. Right now I think it would be nice to just have 1.1 and 1.2 adapters for generic codebase.

For teting purposes I use some services from list here.
http://stackoverflow.com/questions/311654/public-free-web-services-for-testing-soap-client

To provide a better user experience we are looking at nice and functional SOAP libraries on other platforms:
- http://php.net/manual/en/soapclient.soapclient.php
- http://savonrb.com/version3/
- https://github.com/priore/SOAPEngine
- http://www.cs.fsu.edu/~engelen/soap.html
- http://axis.apache.org/axis/

## FAQ
#### Q: I see a SOAP client, but how to build a server?
A: Soap-rs is SOAP client library, we understand the need for SOAP client in modern world, but if you are building a Rust service (and you are) don't use SOAP for your partners. Even if they're C# folks. Your new partner might use Ruby, think about them. (Protip: use JSON).
#### Q: My partner send me some XSD files, but there's no WSDL, can I use this library?
A: Short: XSD is not a standard way to describe SOAP service. In practice though it happens sometimes. We'll try to support XSD base client in future, but expect this to be harder for you (you might have to write glue code).
#### Q: Well, I actually have to send just one request to my partner and I have XML example, can I generate structs just for this request?
A: Again, it's not a valid case for SOAP library, but yep, it will be possible with same limitation as XSD generation.

## Features
- [ ] Support both 2001 (v1.1) and 2003 (v1.2) XML schema.
- [ ] Support array, array of structs, dictionary and sets.
- [ ] Support for user-defined object with serialization of complex data types and array of complex data types, even embedded multilevel structures.
- [ ] Supports ASMX Services, WCF Services (SVC) and the WSDL definitions.
- [ ] Supports Basic, Digest and NTLM Authentication, WS-Security, Client side Certificate and custom security header.
- [ ] AES256 or 3DES Encrypt/Decrypt data without SSL security.
- [ ] An example of service and how to use it is included in source code.
- [ ] Different WSDL caching mods.
- [ ] Custom request headers.
- [ ] Request compression with gzip or other provider.
- [ ] Detailed documentation which covers every component.
- [ ] This is a Rust library, so it should be fast, like really fast.
