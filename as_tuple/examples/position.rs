use as_tuple::AsTuple;

#[derive(AsTuple, Debug)]
struct Position {
    x: f32,
    y: f32,
}

fn main() {
    let mut position = Position { x: 6.2, y: 4.3 };
    let (x, y) = position.as_tuple_mut();
    *x -= 1.0;
    *y += 1.0;
    println!("{:#?}", position);
}
