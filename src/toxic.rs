#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
pub struct Toxic {
    pub name: String,
    #[serde(rename="type")]
    pub typ: ToxicType,
    pub stream: Stream,
    pub toxicity: f64,
    pub attributes: Attributes,
}

#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
pub enum Stream {
    #[serde(rename="downstream")]
    Downstream,
    #[serde(rename="upstream")]
    Upstream,
}

#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
pub enum ToxicType {
    #[serde(rename="latency")]
    Latency,
    #[serde(rename="bandwidth")]
    Bandwidth,
    #[serde(rename="slow_close")]
    SlowClose,
    #[serde(rename="timeout")]
    Timeout,
    #[serde(rename="slicer")]
    Slicer,
    #[serde(rename="limit_data")]
    LimitData,
}

#[derive(Clone,Debug,Default,PartialEq,Serialize,Deserialize)]
pub struct Attributes {
    #[serde(default)]
    pub latency: i64,
    #[serde(default)]
    pub jitter: i64,
    #[serde(default)]
    pub rate: i64,
    #[serde(default)]
    pub delay: i64,
    #[serde(default)]
    pub timeout: i64,
    #[serde(default)]
    pub average_size: i64,
    #[serde(default)]
    pub size_variation: i64,
    #[serde(default)]
    pub bytes: i64,
}
