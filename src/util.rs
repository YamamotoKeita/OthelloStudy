// These may be faster than "for" statement.
// On my computer, these were about 8 times faster than "for" statement in 0.5 billion loops.

pub fn loop_n<F: FnMut()>(n: i32, mut f: F) {
    let mut i = 0;
    loop {
        f();
        i += 1;
        if i == n {
            return;
        }
    }
}

pub fn for_n<F: FnMut(i32)>(n: i32, mut f: F) {
    let mut i = 0;
    loop {
        f(i);
        i += 1;
        if i == n {
            return;
        }
    }
}