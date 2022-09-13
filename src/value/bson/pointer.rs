use crate::value::{base::parse_index, traits::PointerExt};

impl PointerExt for bson::Bson {
    type Context = bson::Bson;
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
                bson::Bson::Document(map) => map.get(&token),
                bson::Bson::Array(list) => parse_index(&token).and_then(|x| list.get(x)),
                _ => None,
            })
    }
}
