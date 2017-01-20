#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
pub struct Toxic {
    pub name: String,
    #[serde(rename="type")]
    pub typ: String,
    pub stream: String,
    pub toxicity: f64,
}
