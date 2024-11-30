use std::collections::HashMap;
use std::hash;

/// Counter counts the number of times each value of type T has been seen.
struct Counter<T>
/* where T: Eq + std::hash::Hash <-- not required since it will just mean that struct can be used, but methods from impl will not apply */ {
    values: HashMap<T, u64>,
}

impl<T> Counter<T> where T: Eq + hash::Hash {
    /// Create a new Counter.
    fn new() -> Self {
        Counter {
            values: HashMap::new(),
        }
    }

    /// Count an occurrence of the given value.
    fn count(&mut self, value: T) {
        if self.values.contains_key(&value) {
            *self.values.get_mut(&value).unwrap() += 1;
        } else {
            self.values.insert(value, 1);
        }
    }

    /// Return the number of times the given value has been seen.
    fn times_seen(&self, value: T) -> u64 {
        self.values.get(&value).copied().unwrap_or_default()
    }
}

#[test]
fn test_i32() {
    let mut ctr = Counter::new();
    ctr.count(13);
    ctr.count(14);
    ctr.count(16);
    ctr.count(14);
    ctr.count(14);
    ctr.count(11);

    assert_eq!(ctr.times_seen(14), 3);
}

#[test]
fn test_str() {
    let mut strctr = Counter::new();
    strctr.count("apple");
    strctr.count("orange");
    strctr.count("apple");
    assert_eq!(strctr.times_seen("apple"), 2);
    assert_eq!(strctr.times_seen("orange"), 1);
}

fn main() {
    let mut ctr = Counter::new();
    ctr.count(13);
    ctr.count(14);
    ctr.count(16);
    ctr.count(14);
    ctr.count(14);
    ctr.count(11);

    for i in 10..20 {
        println!("saw {} values equal to {}", ctr.times_seen(i), i);
    }

    let mut strctr = Counter::new();
    strctr.count("apple");
    strctr.count("orange");
    strctr.count("apple");
    println!("got {} apples", strctr.times_seen("apple"));
}
