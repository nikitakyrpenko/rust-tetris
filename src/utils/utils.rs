use std::collections::HashMap;
use std::hash::Hash;

pub fn group_by<I, K, F>(iter: I, key_fn: F) -> HashMap<K, Vec<I::Item>>
// Hashmap<&usize, Vec<&(usize, usize)>>
where
    I: IntoIterator,      // &Vec<(usize, usize)>
    F: Fn(&I::Item) -> K, //pass in closure as reference, to not borrow it
    K: Eq + Hash,
{
    let mut result = HashMap::new();
    for item in iter.into_iter() {
        result
            .entry(key_fn(&item))
            .or_insert_with(Vec::new)
            .push(item)
    }
    result
}
