/*
 * This file is part of the `rust-tuts` project.
 * Feel free to experiment!
 */

//! Chapter 05

// Getting some RNG

#[test]
fn get_rng() {
    use std::rand::{task_rng, Rng};

    let mut cur;
    for _ in range(0, 11) {
        // nb: range upper bound is exclusive
        cur = task_rng().gen_range(1u, 101);
        println!("{}", cur);
    }
}

// Importing from the standard library

#[test]
fn module_import() {
    // use foo::{bar, baz};
    mod foo {
        pub fn bar() -> bool { true }
        pub fn baz() -> bool { true }
    }

    assert!(foo::bar());
    assert!(foo::baz());
}

#[test]
fn stdin_import() {
    use std::io::stdin;
    stdin();
}

// Calling the user... err?

#[test]
fn match_str() {
    use std::io::BufferedReader;
    use std::io::stdin;

    let mut _reader = BufferedReader::new(stdin());

    // Note: input will end with `\n`
    match Some(~"Hi\n") {
        // Handle empty input too
        Some(~"\n") => println!("\nI never asked for this..."),
        Some(thing) => println!("\nYou typed: {}", thing),
        None        => println!("\nWell, that's unexpected!")
    }
}

#[test]
fn stdout_test() {
    use std::io::stdout;

    stdout().write("Type something: ".as_bytes());
}

#[test]
fn number_input() {
    loop {
        match from_str::<uint>("123\n".trim_right_chars(&'\n')) {
            Some(x) => { assert_eq!(x, 123); break; },
            None    => println!("I'd rather have a number.")
        }
    }
}

// The Solution (well, an implementation of it)

#[test]
fn test_solution() {
    use std::io::BufferedReader;
    use std::io::stdin;

    fn input_line() -> ~str {
        let mut _reader = BufferedReader::new(stdin());
        loop {
            match Some(~"50\n") {
                Some(~"\n") => println!("\nUhm, please type something..."),
                Some(thing) => return thing,
                None => continue
            }
        }
    }

    fn input_number() -> uint {
        loop {
            match from_str::<uint>(input_line().trim_right_chars(&'\n')) {
                Some(x) => return x,
                None => println!("I'd rather have a number.")
            }
        }
    }

    let nbr = 50;
    let mut cpt = 0u;

    println!("Can you guess it?");
    loop {
        cpt += 1;
        match input_number() {
            x if x < nbr => fail!(),
            x if x > nbr => fail!(),
            x => { assert_eq!(x, nbr); break; }
        }
    }
    assert_eq!(cpt, 1u);
}

#[cfg(not(test))]
fn main() {
    fail!("This file is intended to be compiled as a test unit, i.e. with `rustc --test`.");
}