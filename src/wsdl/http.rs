use std::io::Read;
use hyper::Client;
use super::errors::*;

// TODO: Some notes about http data transfering
// What if someone want to use another client?
// What about async/sync client does not matter?
pub fn get(url: &str) -> Result<Vec<u8>> {
    let client = Client::new();
    let mut bytes = Vec::new();

    client.get(url).send()?.read_to_end(&mut bytes)?;

    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_url_test() {
        let result = get("http://httpbin.org/get");

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.len() > 0)
    }

    #[test]
    fn get_url_fail_test() {
        let result = get("http://www.sde.dd/");

        assert!(result.is_err());
    }
}
