// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.

// I AM NOT_DONE
// https://doc.rust-lang.org/reference/expressions/array-expr.html#array-and-slice-indexing-expressions

#[test]
fn slice_out_of_array() {
  let a = [1, 2, 3, 4, 5];
  let nice_slice = &a[1..4];

  assert_eq!([2, 3, 4], nice_slice);

  let b = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
  let b_slice = &b[2..5];

  assert_eq!(['c', 'd', 'e'], b_slice);

  let c = String::from(["a", "b", "c", "d", "e", "f", "g"].join(""));
  let c_slice = &c[2..5];

  assert_eq!("cde", c_slice);

  let c = "abcdefg";
  let c_slice = &c[2..5];

  assert_eq!("cde", c_slice);
}