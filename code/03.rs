/*
 * This file is part of the `rust-tuts` project.
 * Feel free to experiment!
 */

//! Chapter 03
//!
//! Note: functions are marked as test units to
//! be checked by the Rust testing framework.

#[allow(unused_variable)];
#[allow(dead_assignment)];

// The starting point

#[test]
fn hello() {
    println!("Hello World!");
}

#[test]
fn hello_alt() {
    let out = "Hello World!";
    for c in out.chars() {
        /* spawn concurrent tasks! */
        spawn(proc() {
            print!("{}", c);
        });
    }

    println!("");
}

// The looping point

#[test]
fn looping() {
    let n = 5;

    for i in range(0, n) {
        print!("{} ", i);
    }
    for _ in range(1, n+1) {
        print!("nop ");
    }

    println!("");
}

#[test]
fn cake() {
    fn have_my_cake() -> int {
        5
    }

    let mut cake = have_my_cake();
    while cake > 0 {
        cake -= 1; // eat it
    }
}

// Types and assignment

#[test]
fn type_def() {
    let a = 3;
    let b = "code";
    let c = [1, 2, 3];
    let d = true;
}

#[test]
fn type_def2() {
    let a: uint = 3;
    let a = 3u;

    let mut n = 3;
    n = 5u;
    let mut h = ~[];
    h = ~[1u, 2, 3];
}

#[test]
fn squared() {
    fn square(x: int) -> int {
        x*x
    }
    let x = 3;
    let y = square(x);
    println!("{}", y);

    // verify that we have the expected result (cf. ch-04)
    assert_eq!(y, 9);
}

#[test]
fn hype() {
    let cake = "Jelly Bean";

    let hype =
        if cake == "KitKat" {
            10
        } else if cake == "Jelly Bean" {
            7
        } else {
            5
        };

    assert_eq!(hype, 7);
}

// A last note on types and prints

#[test]
fn hello_odd() {
    let out = ['H', 'e', 'l', 'l', 'o'];
    for c in out.iter() {
        print!("{:c}", *c);
    }

    println!("");
}

// Appendix: On functional programming

#[test]
fn iter_fold() {
    let xs = [14, 1, 5, 3, 12];
    let result = xs.iter().fold(0, |accumulator, item| accumulator - *item);

    assert_eq!(result, -35);
}

#[cfg(not(test))]
fn main() {
    fail!("This file is intended to be compiled as a test unit, i.e. with `rustc --test`.");
}