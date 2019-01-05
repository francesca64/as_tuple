# `as_tuple`

View any struct as a tuple! ✨

[![crates.io badge](http://meritbadge.herokuapp.com/as_tuple)](https://crates.io/crates/as_tuple)
[![docs.rs badge](https://docs.rs/as_tuple/badge.svg)](https://docs.rs/as_tuple)
[![Travis badge](https://travis-ci.org/francesca64/as_tuple.svg?branch=mistress)](https://travis-ci.org/francesca64/as_tuple)

```toml
[dependencies]
as_tuple = "0.1"
```

```rust
use as_tuple::AsTuple;

#[derive(AsTuple, Debug)]
struct Position {
    x: f32,
    y: f32,
}

let mut position = Position { x: 6.2, y: 4.3 };
let (x, y) = position.as_tuple_mut();
*x -= 1.0;
*y += 1.0;
println!("{:#?}", position);
```

(You can try it with `cargo run --example position`)
