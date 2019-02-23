use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Maybe<T> {
    StrValue(String),
    Value(T),
}
