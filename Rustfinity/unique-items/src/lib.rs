use std::collections::HashSet;
use std::hash::Hash;

pub fn unique_items<I, T>(items: I) -> Vec<String>
where
    I: Iterator<Item = T>,
    T: AsRef<str> + Eq + Hash,
{
    let mut hash_set: HashSet<String> = HashSet::new();
    let filtered = items.filter_map(|item| {
        let trimmed = item.as_ref().trim();
        if !trimmed.is_empty() {
            Some(trimmed.to_string())
        } else {
            None
        }
    });

    for item in filtered {
        hash_set.insert(item);
    }

    let mut v = Vec::from_iter(hash_set);
    v.sort();

    v
}

/// Example usage
pub fn main() {
    let product_ids = vec![
        "abc123".to_string(),
        "  ".to_string(),
        "def456".to_string(),
        "abc123".to_string(),
        "ghi789".to_string(),
        "ghi789".to_string(),
        "   def456".to_string(),
    ];

    let unique_ids = unique_items(product_ids.into_iter());
    assert_eq!(unique_ids, vec!["abc123", "def456", "ghi789"]);
}
