# soap-rs

STATUS: WORK IN PROGRESS.

SOAP client for Rust programming language.

This project is not intended to be used in production. It is more for a learning purpose, personally to me it is both Rust and SOAP, what I'm going to explore.

Client is planned to support SOAP 1.2 and SOAP 1.1. Right now I think it would be nice to just have 1.1 and 1.2 adapters for generic codebase.

For teting purposes I use some services from list here.
http://stackoverflow.com/questions/311654/public-free-web-services-for-testing-soap-client


## Features
(NOT READY) Support both 2001 (v1.1) and 2003 (v1.2) XML schema.
(NOT READY) Support array, array of structs, dictionary and sets.
(NOT READY) Support for user-defined object with serialization of complex data types and array of complex data types, even embedded multilevel structures.
(NOT READY) Supports ASMX Services, WCF Services (SVC) and now also the WSDL definitions.
(NOT READY) Supports Basic, Digest and NTLM Authentication, WS-Security, Client side Certificate and custom security header.
(NOT READY) AES256 or 3DES Encrypt/Decrypt data without SSL security.
(NOT READY) An example of service and how to use it is included in source code.
