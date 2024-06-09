use serde_json::{Map};
use serde_json::Value as JsonValue;

pub fn get_difference_json(original_json: &JsonValue, other_json: &JsonValue) -> JsonValue {
    match (original_json, other_json) {
        (JsonValue::Object(map1), JsonValue::Object(map2)) => {
            let mut delta_from_original = Map::new();

            for key in map1.keys().chain(map2.keys()) {
                let val1 = map1.get(key);
                let val2 = map2.get(key);

                if val1 != val2 {
                    delta_from_original.insert(key.clone(),
                                               val2.cloned().unwrap_or(JsonValue::Null));
                }
            }

            JsonValue::Object(delta_from_original)
        },
        (JsonValue::Array(arr1), JsonValue::Array(arr2)) => {
            let max_len = arr1.len().max(arr2.len());
            let mut diffs = vec![];

            for i in 0..max_len {
                let val1 = arr1.get(i).unwrap_or(&JsonValue::Null);
                let val2 = arr2.get(i).unwrap_or(&JsonValue::Null);

                if val1 != val2 {
                    diffs.push(JsonValue::Array(vec![val2.clone()]));
                }
            }

            JsonValue::Array(diffs)
        },
        _ => {
            if original_json != other_json {
                other_json.clone()
            } else {
                JsonValue::Null
            }
        }
    }
}