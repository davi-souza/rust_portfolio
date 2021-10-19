use crate::models::{pair::Pair, provider::Provider};

pub fn parse_pair_array(param: Option<String>) -> Option<Vec<Pair>> {
    match param {
        Some(p) => Some::<Vec<Pair>>(
            p.split(',')
                .filter_map(|s| {
                    let maybe_pair: Result<Pair, _> = serde_json::from_str(&format!(r#""{}""#, s));
                    match maybe_pair {
                        Ok(pair) => Some(pair),
                        Err(_) => None,
                    }
                })
                .collect(),
        ),
        None => None,
    }
}

pub fn parse_provider_array(param: Option<String>) -> Option<Vec<Provider>> {
    match param {
        Some(p) => Some::<Vec<Provider>>(
            p.split(',')
                .filter_map(|s| {
                    let maybe_provider: Result<Provider, _> =
                        serde_json::from_str(&format!(r#""{}""#, s));
                    match maybe_provider {
                        Ok(provider) => Some(provider),
                        Err(_) => None,
                    }
                })
                .collect(),
        ),
        None => None,
    }
}
