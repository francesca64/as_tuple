use as_tuple::AsTuple;

#[derive(AsTuple, Debug)]
struct Nothing;

#[test]
fn unit() {
    let mut nothing = Nothing;
    let _ = nothing.as_tuple_mut();
    nothing.as_tuple();
}
