use std::cmp::Ordering;

// TODO: implement the `min` function used in `main`.
fn min<T: Ord>(a: T, b: T) -> T {
    // variant one:
    return a.min(b); // because Ord gives you min, not just cmp
    
    // variant two:
    if a < b {
        return a;
    } else {
        return b;
    }

    // variant 3:
    match a.cmp(&b) {
        Ordering::Less | Ordering::Equal => a,
        _ => b, // or Ordering::Greater
    }   
}

fn main() {
    test_case_impl()
}

#[test]
fn test_case() {
    test_case_impl()
}

fn test_case_impl() {
    assert_eq!(min(0, 10), 0);
    assert_eq!(min(500, 123), 123);

    assert_eq!(min('a', 'z'), 'a');
    assert_eq!(min('7', '1'), '1');

    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");
}