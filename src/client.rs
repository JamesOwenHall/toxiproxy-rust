use std::collections::HashMap;
use std::io::Read;
use {hyper, serde_json};
use {Error, Proxy, Toxic};
use hyper::status::StatusCode;

#[derive(Debug)]
pub struct Client {
    url: String,
    client: hyper::Client,
}

impl Client {
    pub fn new<S: ToString>(url: S) -> Client {
        let mut url = url.to_string();
        if !url.starts_with("http://") {
            let mut new_url = String::from("http://");
            new_url.push_str(&url);
            url = new_url;
        }

        Client {
            url: url,
            client: hyper::Client::new(),
        }
    }

    pub fn proxies(&self) -> Result<HashMap<String, Proxy>, Error> {
        let mut resp = self.client.get(&self.full_url("/proxies")).send()?;
        match resp.status {
            StatusCode::Ok => {},
            code => return Err(Self::code_error(code)),
        }

        let mut body = String::new();
        resp.read_to_string(&mut body)?;

        let proxies: HashMap<String, Proxy> = serde_json::from_str(&body)?;
        Ok(proxies)
    }

    pub fn create_proxy(&self, proxy: &Proxy) -> Result<(), Error> {
        let encoded = serde_json::to_string(proxy).unwrap();
        let resp = self.client
            .post(&self.full_url("/proxies"))
            .body(&encoded)
            .send()?;

        match resp.status {
            StatusCode::Created => Ok(()),
            code => Err(Self::code_error(code)),
        }
    }

    pub fn delete_proxy(&self, proxy: &str) -> Result<(), Error> {
        let mut url = self.full_url("/proxies/");
        url.push_str(proxy);

        let resp = self.client
            .delete(&url)
            .send()?;

        match resp.status {
            StatusCode::NoContent => Ok(()),
            code => Err(Self::code_error(code)),
        }
    }

    pub fn update_proxy(&self, proxy: &Proxy) -> Result<(), Error> {
        let mut url = self.full_url("/proxies/");
        url.push_str(&proxy.name);

        let encoded = serde_json::to_string(proxy).unwrap();
        let resp = self.client
            .post(&url)
            .body(&encoded)
            .send()?;

        match resp.status {
            StatusCode::Ok => Ok(()),
            code => Err(Self::code_error(code)),
        }
    }

    pub fn toxics(&self, proxy: &str) -> Result<Vec<Toxic>, Error> {
        let path = format!("/proxies/{}/toxics", proxy);
        let mut resp = self.client.get(&self.full_url(&path)).send()?;
        match resp.status {
            StatusCode::Ok => {},
            code => return Err(Self::code_error(code)),
        }

        let mut body = String::new();
        resp.read_to_string(&mut body)?;

        let toxics: Vec<Toxic> = serde_json::from_str(&body)?;
        Ok(toxics)
    }

    fn full_url(&self, path: &str) -> String {
        let mut url = self.url.clone();
        url.push_str(path);
        url
    }

    fn code_error(code: StatusCode) -> Error {
        Error::ServerError(format!("{}", code))
    }
}
