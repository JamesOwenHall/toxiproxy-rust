use std::collections::HashMap;
use Error;

#[derive(Clone,Debug,PartialEq)]
pub struct Toxic {
    pub name: String,
    pub stream: Stream,
    pub toxicity: f64,
    pub typ: ToxicType,
}

impl Toxic {
    pub fn to_json_toxic(&self) -> JsonToxic {
        JsonToxic {
            name: self.name.clone(),
            typ: self.typ.name(),
            stream: self.stream.clone(),
            toxicity: self.toxicity,
            attributes: self.typ.attributes(),
        }
    }

    pub fn from_json_toxic(json: &JsonToxic) -> Result<Self, Error> {
        let typ = ToxicType::from(&json.typ, &json.attributes)?;
        Ok(Toxic {
            name: json.name.clone(),
            stream: json.stream.clone(),
            toxicity: json.toxicity,
            typ: typ,
        })
    }
}

#[derive(Clone,Debug,PartialEq)]
pub enum ToxicType {
    Latency{latency: i64, jitter: i64},
    Bandwidth{rate: i64},
    SlowClose{delay: i64},
    Timeout{timeout: i64},
    Slicer{average_size: i64, size_variation: i64, delay: i64},
    LimitData{bytes: i64},
}

impl ToxicType {
    fn name(&self) -> String {
        use self::ToxicType::*;
        match self {
            &Latency{..} => "latency",
            &Bandwidth{..} => "bandwidth",
            &SlowClose{..} => "slow_close",
            &Timeout{..} => "timeout",
            &Slicer{..} => "slicer",
            &LimitData{..} => "limit_data",
        }.to_string()
    }

    fn attributes(&self) -> HashMap<String, i64> {
        let mut map = HashMap::new();

        use self::ToxicType::*;
        match self {
            &Latency{latency: l, jitter: j} => {
                map.insert("latency".to_string(), l);
                map.insert("jitter".to_string(), j);
            },
            &Bandwidth{rate: r} => {
                map.insert("rate".to_string(), r);
            },
            &SlowClose{delay: d} => {
                map.insert("delay".to_string(), d);
            },
            &Timeout{timeout: t} => {
                map.insert("timeout".to_string(), t);
            },
            &Slicer{average_size: a, size_variation: v, delay: d} => {
                map.insert("average_size".to_string(), a);
                map.insert("size_variation".to_string(), v);
                map.insert("delay".to_string(), d);
            },
            &LimitData{bytes: b} => {
                map.insert("bytes".to_string(), b);
            },
        }

        map
    }

    fn from(name: &str, attributes: &HashMap<String, i64>) -> Result<Self, Error> {
        use self::ToxicType::*;
        match name {
            "latency" => Ok(Latency {
                latency: attributes.get("latency").cloned().unwrap_or_default(),
                jitter: attributes.get("jitter").cloned().unwrap_or_default(),
            }),
            "bandwidth" => Ok(Bandwidth {
                rate: attributes.get("rate").cloned().unwrap_or_default(),
            }),
            "slow_close" => Ok(SlowClose {
                delay: attributes.get("delay").cloned().unwrap_or_default(),
            }),
            "timeout" => Ok(Timeout {
                timeout: attributes.get("timeout").cloned().unwrap_or_default(),
            }),
            "slicer" => Ok(Slicer {
                average_size: attributes.get("average_size").cloned().unwrap_or_default(),
                size_variation: attributes.get("size_variation").cloned().unwrap_or_default(),
                delay: attributes.get("delay").cloned().unwrap_or_default(),
            }),
            "limit_data" => Ok(LimitData {
                bytes: attributes.get("bytes").cloned().unwrap_or_default(),
            }),
            t => Err(Error::ServerError(format!("unknown toxic type {}", t))),
        }
    }
}

#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
pub struct JsonToxic {
    name: String,
    #[serde(rename="type")]
    typ: String,
    stream: Stream,
    toxicity: f64,
    attributes: HashMap<String, i64>,
}

#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
pub enum Stream {
    #[serde(rename="downstream")]
    Downstream,
    #[serde(rename="upstream")]
    Upstream,
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
