use std::collections::BTreeMap;

/// 用于Map类型数据操作工具类
pub trait MapExt<K, V> {}

impl<K, V> MapExt<K, V> for BTreeMap<K, V> {}
