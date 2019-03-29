use std::fmt;

/// - `K`: the key type;
/// - `V`: the value type.
pub type EntryList<K, V> = Option<Vec<Entry<K, V>>>;

/// `entry_list` is optional because a valid json file can be empty (`{}`).
/// ## Example
///
/// ```
/// {} // valid
/// { "foo" : "bar" } // valid
/// { "foo" : "bar", } // valid
/// { "baz": [1, 2, 3] } // valid
/// ```
pub struct JsonObject<K, V> {
    entry_list: EntryList<K, V>
}

pub struct Entry<K, V> {
    key: Key<K>,
    value: Value<V>
}

/// `data` should be `isize` or `String`.
pub struct Key<K> {
    data: K
}

pub struct Value<V> {
    data: V
}


impl<K> fmt::Display for Key<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl<V> fmt::Display for Value<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}
