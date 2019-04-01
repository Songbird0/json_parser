use crate::ast::keys::Key;
use crate::ast::values::Value;

/// - `K`: the key type;
/// - `V`: the value type.
pub type EntryList<K: Key, V> = Option<Vec<Entry<K, Value<V>>>>;

/// `entry_list` is optional because a valid json file can be empty (`{}`).
/// ## Example
///
/// ```json
/// {} // valid
/// { "foo" : "bar" } // valid
/// { "foo" : "bar", } // valid
/// { "baz": [1, 2, 3] } // valid
/// ```
pub struct JsonObject<K: Key, V>{
    entry_list: EntryList<K, V>
}

pub struct Entry<K: Key, V> {
    key: K,
    value: Value<V>
}
