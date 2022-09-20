use crate::bean::{base::parse_index, traits::PointerExt};

impl PointerExt for serde_json::Value {
    type Context = serde_json::Value;
    fn p(&self, pointer: &str) -> Option<&Self::Context> {
        if pointer.is_empty() {
            return Some(self);
        }
        if !pointer.starts_with('/') {
            return None;
        }
        pointer
            .split('/')
            .skip(1)
            .map(|x| x.replace("~1", "/").replace("~0", "~"))
            .try_fold(self, |target, token| match target {
                serde_json::Value::Object(map) => map.get(&token),
                serde_json::Value::Array(list) => parse_index(&token).and_then(|x| list.get(x)),
                _ => None,
            })
    }
}
