use std::collections::HashMap;
use std::io::Read;
use {Error, hyper, Proxy};
use rustc_serialize::json;

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
        let mut body = String::new();
        resp.read_to_string(&mut body)?;

        let proxies: HashMap<String, Proxy> = json::decode(&body)?;
        Ok(proxies)
    }

    pub fn create_proxy(&self, proxy: &Proxy) -> Result<(), Error> {
        let encoded = json::encode(proxy).unwrap();
        self.client
            .post(&self.full_url("/proxies"))
            .body(&encoded)
            .send()?;
        Ok(())
    }

    pub fn delete_proxy(&self, name: &str) -> Result<(), Error> {
        let mut url = self.full_url("/proxies/");
        url.push_str(name);

        self.client
            .delete(&url)
            .send()?;
        Ok(())
    }

    pub fn update_proxy(&self, proxy: &Proxy) -> Result<(), Error> {
        let mut url = self.full_url("/proxies/");
        url.push_str(&proxy.name);

        let encoded = json::encode(proxy).unwrap();
        self.client
            .post(&url)
            .body(&encoded)
            .send()?;

        Ok(())
    }

    fn full_url(&self, path: &str) -> String {
        let mut url = self.url.clone();
        url.push_str(path);
        url
    }
}
