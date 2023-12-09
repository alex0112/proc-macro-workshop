use derive_builder::Builder;

fn main() {
    let f = Foo::builder();

    dbg!(f);
}

#[derive(Builder)]
struct Foo {
    a: String,
    b: String,
}
