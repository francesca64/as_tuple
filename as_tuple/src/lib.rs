//! View any struct as a tuple! ✨
//!
//! See [`AsTuple`] for usage info.

pub use as_tuple_derive::AsTuple;

/// A trait for viewing a struct as a tuple. You shouldn't implement this
/// yourself, since the point of this crate is to derive it for you.
///
/// # Examples
///
/// ```
/// use as_tuple::AsTuple;
///
/// #[derive(AsTuple)]
/// struct Position {
///     x: f32,
///     y: f32,
/// }
///
/// let mut position = Position { x: 6.2, y: 4.3 };
/// {
///     let (x, y) = position.as_tuple_mut();
///     *x -= 1.0;
///     *y += 1.0;
/// }
///
/// assert_eq!(position.x, 5.2);
/// assert_eq!(position.y, 5.3);
/// ```
///
/// # Remarks
///
/// The tuple contains a reference to every field, so you'll often have
/// references to references.
///
/// Don't attempt to derive this on an enum or a union, since um... it won't
/// work.
pub trait AsTuple<'a> {
    /// The type returned by `as_tuple`. It contains an immutable reference to
    /// each field in the struct, in order.
    type Tuple;
    /// The type returned by `as_tuple_mut`. It's the same as `Tuple`, but
    /// contains mutable references.
    type TupleMut;
    /// Gets a tuple containing immutable references to every field of the struct.
    fn as_tuple(&'a self) -> Self::Tuple;
    /// Gets a tuple containing mutable references to every field of the struct.
    fn as_tuple_mut(&'a mut self) -> Self::TupleMut;
}
