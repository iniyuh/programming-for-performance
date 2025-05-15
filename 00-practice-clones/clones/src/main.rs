use std::env;

// must call with two command-line parameters
fn main() {
    let mut some_vec: Vec<String> = env::args().collect();
    let s = S::new(&mut some_vec);

    println!(
        "got struct S with foo {} and bar {}",
        s.foo_string, s.bar_string
    );
}

// do not change this struct definition
struct S {
    foo_string: String,
    bar_string: String,
}

// you will need to change the impl
impl S {
    fn new(args: &mut Vec<String>) -> S {
        let b = args.pop().unwrap();
        let f = args.pop().unwrap();

        S {
            foo_string: f,
            bar_string: b,
        }
    }
}
