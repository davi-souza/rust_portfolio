use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Copy, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Pair {
    EurGbp,
    EurUsd,
    GbpUsd,
}
