/*
 * This file is part of the `rust-tuts` project.
 * Feel free to experiment!
 */

//! Chapter 06

// Give me pointers

#[test]
fn ref_ptr() {
    let mut _y;

    let x = 3;
    if true {
        _y = &x;
        print!("{} ", x);
    }
}

#[test]
fn ref_ptr2() {
    let mut _y;

    let mut _x = &3;
    if true {
        _y = 4;
        _x = &_y;
    }
}

#[test]
fn owned_ptr() {
    fn take<T>(_ptr: ~T) {
        /* no-op */
    }

    let m = ~1;
    let n = m.clone();
    take(m);
    take(n);
}

#[cfg(not(test))]
fn main() {
    fail!("This file is intended to be compiled as a test unit, i.e. with `rustc --test`.");
}