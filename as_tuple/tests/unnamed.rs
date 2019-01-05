use as_tuple::AsTuple;
use std::fmt::Debug;

#[derive(AsTuple, Debug)]
struct Pair<'a, T: Debug>(&'a str, T);

#[test]
fn unnamed() {
    let mut pair = Pair("favorite number", 64);
    let _ = pair.as_tuple_mut();
    pair.as_tuple();
}
