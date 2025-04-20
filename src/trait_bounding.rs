use std::cmp::Eq;
use std::fmt::Debug;
use std::hash::Hash;

fn find_duplicates<T>(items: &[T]) -> Vec<&T>
where
    T: Eq + Hash + Debug,
{
    use std::collections::HashMap;
    let mut seen = HashMap::new();
    let mut duplicates = Vec::new();

    for item in items {
        let count = seen.entry(item).or_insert(0);
        *count += 1;

        if *count == 2 {
            duplicates.push(item);
            println!("Found duplicate: {:?}", item);
        }
    }

    duplicates
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct User {
    id: u32,
    name: String,
}

pub fn demonstrate_find_duplicates() {
    let users = vec![
        User {
            id: 1,
            name: String::from("Alice"),
        },
        User {
            id: 2,
            name: String::from("Bob"),
        },
        User {
            id: 1,
            name: String::from("Alice"),
        },
    ];

    let duplicates = find_duplicates(&users);
    println!("Duplicates found: {:?}", duplicates);
}
