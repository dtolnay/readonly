mod m {
    #[readonly::make]
    pub struct S {
        pub n: i32,
    }

    impl S {
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
    s.n += 1;
}

fn main() {}
