#[readonly::make]
pub struct S {
    #[readonly = "..."]
    namevalue: i32,

    #[readonly(...)]
    list: i32,
}

fn main() {}
