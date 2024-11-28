/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(n: i32) -> u32 {
    if n == 1 {
        1
    } else if n % 2 == 1 {
        1 + collatz_length(3*n+1)
    } else {
        1 + collatz_length(n/2)
    }
}

// Without using recursion
fn collatz_length_no_recurse(mut n: i32) -> u32 {
    let mut length = 1;
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        length += 1;
    }
    length
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
    assert_eq!(collatz_length_no_recurse(11), 14);

}

fn main() {
    println!("Length: {}", collatz_length(11));
    println!("Length: {}", collatz_length_no_recurse(11));
}
