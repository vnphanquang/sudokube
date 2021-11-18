pub mod merge {
    pub mod strategy {
        use merge::Merge;
        use std::{collections::HashMap, hash::Hash};

        pub fn merge_nested_struct<T: Merge>(left: &mut Option<T>, right: Option<T>) {
            if let Some(lv) = left {
                if let Some(rv) = right {
                    lv.merge(rv);
                }
            } else {
                left.merge(right);
            }
        }

        pub fn merge_hash_map<K: Eq + Hash, V>(
            left: &mut Option<HashMap<K, V>>,
            right: Option<HashMap<K, V>>,
        ) {
            if let Some(lv) = left {
                if let Some(rv) = right {
                    for (key, value) in rv {
                        lv.insert(key, value);
                    }
                }
            } else {
                left.merge(right);
            }
        }
    }
}
