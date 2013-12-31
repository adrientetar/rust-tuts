/*
 * This file is part of the `rust-tuts` project.
 * Feel free to experiment!
 */

//! Chapter 04

// Tests and linkage attributes

#[cfg(test)]
mod external {
    pub fn input() -> int {
        2
    }
}

#[test]
fn try_fetch() {
    if external::input() == -1 {
        fail!("Couldn't retrieve the result!");
    }
}

// Doing it wrong

#[test]
fn lowercase_ascii() {
    use std::ascii::OwnedStrAsciiExt;

    let tmp = ~"ME";
    assert_eq!(~"me", tmp.into_ascii_lower());
}

#[test]
fn number_matching() {
    let my_number = 11;

    match my_number {
        x if x < 0 => println("something lower than zero"),
        0          => println("zero"),
        1 | 2      => println("one or two"),
        3..10      => println("three to ten"),
        _          => println("something else")
    }
}

#[test]
fn match_option() {
    fn get_option_val() -> Option<int> {
        Some(5)
    }
    let optional_arg = get_option_val();

    let result = match optional_arg {
        Some(val) => val,
        None => 0
    };
    assert_eq!(result, 5);
}

#[test]
fn unwrap_option() {
    fn get_option_val() -> Option<int> {
        Some(5)
    }
    let mut optional_arg = get_option_val();

    // unwrap our Option
    let result = optional_arg.expect("No value returned!");
    assert!(optional_arg.is_some());
    assert_eq!(result, 5);

    // stick to default value
    optional_arg = None;
    let result = optional_arg.unwrap_or(3);
    assert!(optional_arg.is_none());
    assert_eq!(result, 3);
}

// Appendix: Can you `match` it?

#[test]
fn unpack_integer() {
    let f: f32 = 1.0e-2;

    let (mantissa, _, _) = f.integer_decode();
    println!("{}", mantissa);
}

#[test]
fn match_fizzbuzz() {
    for i in range(0u, 101) {
        match (i % 3 == 0, i % 5 == 0) {
            (true, true)   => println("Fizz Buzz"),
            (true, false)  => println("Fizz"),
            (false, true)  => println("Buzz"),
            (false, false) => println!("{}", i)
        }
    }
}

#[cfg(not(test))]
fn main() {
    fail!("This file is intended to be compiled as a test unit, ie. with `rustc --test`.");
}