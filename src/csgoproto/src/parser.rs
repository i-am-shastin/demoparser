use std::collections::HashMap;

pub fn parse(input: &str) -> HashMap<String, String> {
    let mut translation_map = HashMap::new();

    for line in input.split('\n') {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('/') {
            continue;
        }

        let parts = trimmed.splitn(2, ['\t', ' ']).collect::<Vec<_>>();
        let [key, value] = parts.as_slice() else { continue };
        if !value.is_empty() {
            translation_map.insert(trim_string(key), trim_string(value));
        }
    }

    translation_map
}

fn trim_string(value: &str) -> String {
    value.trim().trim_matches(['\t', '"']).to_string()
}