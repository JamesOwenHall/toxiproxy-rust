use super::*;

#[test]
fn no_proxies() {
    let client = test_client();
    let proxies = client.proxies().unwrap();
    assert_eq!(proxies.len(), 0);
}

#[test]
fn create_proxy() {
    let client = test_client();
    let proxy = test_proxy();
    client.create_proxy(&proxy).unwrap();

    let proxies = client.proxies().unwrap();
    assert_eq!(1, proxies.len());
    assert_eq!(Some(&proxy), proxies.get(&proxy.name));
}

#[test]
fn update_proxy() {
    let client = test_client();
    let mut proxy = test_proxy();
    client.create_proxy(&proxy).unwrap();

    proxy.enabled = false;
    proxy.upstream = "example.org:80".to_string();
    client.update_proxy(&proxy).unwrap();

    let proxies = client.proxies().unwrap();
    assert_eq!(1, proxies.len());
    assert_eq!(Some(&proxy), proxies.get(&proxy.name));
}

#[test]
fn delete_proxy() {
    let client = test_client();
    let proxy = test_proxy();
    client.create_proxy(&proxy).unwrap();
    client.delete_proxy(&proxy.name).unwrap();
    
    let proxies = client.proxies().unwrap();
    assert_eq!(0, proxies.len());
}

#[test]
fn unknown_proxy() {
    let client = test_client();
    let proxy = test_proxy();

    assert!(client.update_proxy(&proxy).is_err());
    assert!(client.delete_proxy(&proxy.name).is_err());
}

#[test]
fn duplicate_proxy() {
    let client = test_client();
    let proxy = test_proxy();
    client.create_proxy(&proxy).unwrap();
    assert!(client.create_proxy(&proxy).is_err());
}

#[test]
fn no_toxics() {
    let client = test_client();
    let proxy = test_proxy();

    client.create_proxy(&proxy).unwrap();
    let toxics = client.toxics(&proxy.name).unwrap();
    assert_eq!(0, toxics.len());
}

#[test]
fn create_toxic() {
    let client = test_client();
    let proxy = test_proxy();
    client.create_proxy(&proxy).unwrap();

    let toxic = test_toxic();
    client.create_toxic(&proxy.name, &toxic).unwrap();
    
    let toxics = client.toxics(&proxy.name).unwrap();
    assert_eq!(1, toxics.len());
    assert_eq!(toxic, toxics[0]);
}

#[test]
fn update_toxic() {
    let client = test_client();
    let proxy = test_proxy();
    client.create_proxy(&proxy).unwrap();

    let mut toxic = test_toxic();
    client.create_toxic(&proxy.name, &toxic).unwrap();

    toxic.toxicity = 0.5;
    client.update_toxic(&proxy.name, &toxic).unwrap();

    let toxics = client.toxics(&proxy.name).unwrap();
    assert_eq!(1, toxics.len());
    assert_eq!(toxic, toxics[0]);
}

#[test]
fn delete_toxic() {
    let client = test_client();
    let proxy = test_proxy();
    client.create_proxy(&proxy).unwrap();

    let toxic = test_toxic();
    client.create_toxic(&proxy.name, &toxic).unwrap();
    client.delete_toxic(&proxy.name, &toxic.name).unwrap();
    
    let toxics = client.toxics(&proxy.name).unwrap();
    assert_eq!(0, toxics.len());
}

#[test]
fn unknown_toxic() {
    let client = test_client();
    let proxy = test_proxy();
    client.create_proxy(&proxy).unwrap();

    let toxic = test_toxic();
    assert!(client.update_toxic(&proxy.name, &toxic).is_err());
    assert!(client.delete_toxic(&proxy.name, &toxic.name).is_err());
}

#[test]
fn duplicate_toxic() {
    let client = test_client();
    let proxy = test_proxy();
    client.create_proxy(&proxy).unwrap();

    let toxic = test_toxic();
    client.create_toxic(&proxy.name, &toxic).unwrap();
    assert!(client.create_toxic(&proxy.name, &toxic).is_err());
}

fn test_client() -> Client {
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

fn test_toxic() -> Toxic {
    Toxic {
        name: "testtoxic".to_string(),
        stream: Stream::Downstream,
        toxicity: 1.0,
        typ: ToxicType::Latency {
            latency: 100,
            jitter: 0,
        },
    }
}
