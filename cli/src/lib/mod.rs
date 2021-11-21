pub mod merge {
    pub mod strategy {
        pub mod option {
            pub fn overwrite<T>(left: &mut Option<T>, right: Option<T>) {
                *left = right;
            }
        }

        pub mod hashmap {
            use std::{collections::HashMap, hash::Hash};
            pub fn recurse_option<K: Eq + Hash, V>(
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
                    *left = right;
                }
            }
        }
    }
}
