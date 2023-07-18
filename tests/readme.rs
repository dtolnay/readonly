#![allow(clippy::needless_pass_by_ref_mut)]

mod m {
    #[readonly::make]
    pub struct S {
        pub n: i32,
    }

    impl S {
        pub fn new(n: i32) -> Self {
            S { n }
        }

        pub fn demo(&mut self) {
            // Can read and write from inside the same module.
            println!("{}", self.n);
            self.n += 1;
        }
    }
}

fn demo(s: &mut m::S) {
    // From outside the module, can only read.
    println!("{}", s.n);

    // Does not compile:
    //s.n += 1;
}

#[test]
fn test() {
    let mut s = m::S::new(0);

    s.demo();

    demo(&mut s);
}
