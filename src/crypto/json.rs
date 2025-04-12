use serde_json::Value;
use std::collections::BTreeMap;

/// Recursively sorts JSON objects by key to create a canonical representation.
///
/// This is crucial for the signing process (/sign and /verify) to ensure that
/// the signature remains consistent even if the order of properties in the
/// input JSON object changes.
/// Arrays are processed element by element, but their order is preserved.
/// Other JSON types (String, Number, Boolean, Null) are returned as is.
pub fn canonicalize_json(value: &Value) -> Value {
    match value {
        Value::Object(obj) => {
            let mut sorted = BTreeMap::new();
            for (key, value) in obj {
                sorted.insert(key.clone(), canonicalize_json(value));
            }
            // Convert BTreeMap to serde_json::Map
            let mut map = serde_json::Map::new();
            for (key, value) in sorted {
                map.insert(key, value);
            }
            Value::Object(map)
        }
        Value::Array(arr) => {
            let mut sorted = Vec::new();
            for value in arr {
                sorted.push(canonicalize_json(value));
            }
            Value::Array(sorted)
        }
        _ => value.clone(),
    }
} 