use serde_json::{Map, Value};

pub fn insert_or_arrayify(map: &mut Map<String, Value>, key: String, value: Value) {
    match map.remove(key.as_str()) {
        Some(Value::Array(mut arr)) => {
            arr.push(value);
            map.insert(key, Value::Array(arr))
        }
        Some(prev) => map.insert(key, Value::Array(vec![prev, value])),
        None => map.insert(key, value),
    };
}
