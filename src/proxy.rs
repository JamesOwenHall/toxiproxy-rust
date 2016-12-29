#[derive(Clone,Debug,PartialEq,RustcEncodable,RustcDecodable)]
pub struct Proxy {
    pub name: String,
    pub listen: String,
    pub upstream: String,
    pub enabled: bool,
}
