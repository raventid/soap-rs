mod document;

struct Header {
 wsse_auth: WsseAuth,
 wsse_timestamp: WsseTimestamp,
 wsse_signature: WsseSignature,
 soap_header: SoapHeader
}
