//! Merging the authored overlay onto the generated layer.
//!
//! Objects merge field by field. Arrays whose elements all carry an identity field
//! (`key`, `id` or `n`) merge element by element on that field; lists of panel ops
//! (elements with an `op`) are appended to, so a step keeps the ops the importer made
//! (a test run's output); other arrays are replaced by the overlay's array.

use serde_json::Value;

const IDENTITY: [&str; 3] = ["key", "id", "n"];

pub fn merge(base: &mut Value, overlay: Value) {
    match (base, overlay) {
        (Value::Object(b), Value::Object(o)) => {
            for (k, v) in o {
                match b.get_mut(&k) {
                    Some(slot) => merge(slot, v),
                    None => {
                        b.insert(k, v);
                    }
                }
            }
        }
        (Value::Array(b), Value::Array(o)) => match identity_field(b, &o) {
            Some(field) => {
                for item in o {
                    let id = item.get(field).cloned();
                    match b.iter_mut().find(|e| e.get(field) == id.as_ref()) {
                        Some(slot) => merge(slot, item),
                        None => b.push(item),
                    }
                }
            }
            None if !o.is_empty() && b.iter().chain(&o).all(|e| e.get("op").is_some()) => {
                b.extend(o)
            }
            None => *b = o,
        },
        (slot, v) => *slot = v,
    }
}

fn identity_field(base: &[Value], overlay: &[Value]) -> Option<&'static str> {
    IDENTITY.into_iter().find(|f| {
        !overlay.is_empty()
            && base
                .iter()
                .chain(overlay)
                .all(|e| e.get(*f).is_some_and(|v| !v.is_null()))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn panel_ops_are_appended() {
        let mut base = json!({"ops": [{"panel": "out", "op": "print", "text": "ok"}]});
        merge(&mut base, json!({"ops": [{"panel": "tree", "op": "node", "id": "a"}]}));
        assert_eq!(base["ops"].as_array().map(Vec::len), Some(2));
    }

    #[test]
    fn arrays_merge_by_key() {
        let mut base = json!({"tests": [{"key": "t01", "a": 1}, {"key": "t02", "a": 2}]});
        merge(&mut base, json!({"tests": [{"key": "t02", "b": 3}]}));
        assert_eq!(
            base,
            json!({"tests": [{"key": "t01", "a": 1}, {"key": "t02", "a": 2, "b": 3}]})
        );
    }

    #[test]
    fn plain_arrays_are_replaced() {
        let mut base = json!({"hl": [1, 2], "x": {"y": 1}});
        merge(&mut base, json!({"hl": [3], "x": {"z": 2}}));
        assert_eq!(base, json!({"hl": [3], "x": {"y": 1, "z": 2}}));
    }
}
