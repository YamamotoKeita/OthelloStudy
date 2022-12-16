// These may be faster than "for" statement.
// On my computer, these were about 8 times faster than "for" statement in 0.5 billion loops.

#[allow(dead_code)]
#[inline(always)]
pub fn loop_n<F: FnMut()>(n: u64, mut f: F) {
    let mut i = 0;
    loop {
        f();
        i += 1;
        if i == n {
            return;
        }
    }
}

#[allow(dead_code)]
#[inline(always)]
pub fn for_n<F: FnMut(u64)>(n: u64, mut f: F) {
    let mut i = 0;
    loop {
        f(i);
        i += 1;
        if i == n {
            return;
        }
    }
}