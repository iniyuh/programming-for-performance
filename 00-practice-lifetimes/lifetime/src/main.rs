fn main() {
    println!("{}", g(&59));
}

// may not change the signature of g()
fn g<'a>(w: &'a i32) -> &'a i32 {
    // may not remove 'static on rv's type annotation
    let rv: &'static i32 = f(&4, w);
    rv
}

fn f(i: &i32, _j: &i32) -> &i32 {
    i
}
