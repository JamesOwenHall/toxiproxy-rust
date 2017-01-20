use super::*;
use std::collections::HashMap;

#[test]
fn no_proxies() {
    let client = new_client();
    let proxies = client.proxies().unwrap();
    assert_eq!(proxies.len(), 0);
}

#[test]
fn proxy_lifecycle() {
    let client = new_client();
    let proxy = test_proxy();

    // Create.
    client.create_proxy(&proxy).unwrap();
    let mut exp = HashMap::new();
    exp.insert(proxy.name.to_string(), proxy.clone());
    assert_eq!(exp, client.proxies().unwrap());

    // Update.
    {
        let proxy = exp.get_mut(&proxy.name).unwrap();
        proxy.listen = "127.0.0.1:13307".to_string();
        proxy.upstream = "127.0.0.1:3307".to_string();
        client.update_proxy(proxy).unwrap();
    }
    assert_eq!(exp, client.proxies().unwrap());

    // Delete.
    client.delete_proxy(&proxy.name).unwrap();
    assert_eq!(0, client.proxies().unwrap().len());
}

#[test]
fn unknown_proxy() {
    let client = new_client();
    let proxy = test_proxy();

    assert!(client.update_proxy(&proxy).is_err());
    assert!(client.delete_proxy(&proxy.name).is_err());
}

#[test]
fn no_toxics() {
    let client = new_client();
    let proxy = test_proxy();

    client.create_proxy(&proxy).unwrap();
    let toxics = client.toxics(&proxy.name).unwrap();
    assert_eq!(0, toxics.len());
}

fn new_client() -> Client {
    let client = Client::new("localhost:8474");
    let proxies = client.proxies().unwrap();

    for name in proxies.keys() {
        client.delete_proxy(name).unwrap();
    }

    client
}

fn test_proxy() -> Proxy {
    Proxy {
        name: "testproxy".to_string(),
        listen: "127.0.0.1:13306".to_string(),
        upstream: "127.0.0.1:3306".to_string(),
        enabled: true,
    }
}
