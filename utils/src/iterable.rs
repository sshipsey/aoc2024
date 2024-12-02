use std::collections::HashMap;

pub fn count<T: std::hash::Hash + Eq>(iter: impl Iterator<Item = T>) -> HashMap<T, usize> {
    let mut map = HashMap::new();

    for item in iter {
        *map.entry(item).or_insert(0) += 1; // Increment the count
    }

    map
}
