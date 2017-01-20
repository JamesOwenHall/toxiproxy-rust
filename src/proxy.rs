#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
pub struct Proxy {
    pub name: String,
    pub listen: String,
    pub upstream: String,
    pub enabled: bool,
}
