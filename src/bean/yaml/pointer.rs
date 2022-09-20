use crate::bean::{base::parse_index, traits::PointerExt};

impl PointerExt for serde_yaml::Value {
    type Context = serde_yaml::Value;
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
                serde_yaml::Value::Mapping(map) => map.get(&token),
                serde_yaml::Value::Sequence(list) => parse_index(&token).and_then(|x| list.get(x)),
                _ => None,
            })
    }
}
