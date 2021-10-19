use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum UpdateValue<T> {
    Undefined,
    Value(T),
}

impl<T> UpdateValue<T> {
    fn get_default() -> Self {
        UpdateValue::Undefined
    }
}

#[derive(Debug, Deserialize)]
pub struct NewItem {
    pub text: String,
    pub number: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateItem {
    #[serde(default = "UpdateValue::get_default")]
    pub text: UpdateValue<String>,
    #[serde(default = "UpdateValue::get_default")]
    pub number: UpdateValue<i64>,
}
