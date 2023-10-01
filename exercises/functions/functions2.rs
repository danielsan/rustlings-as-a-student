// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT_DONE

fn main() {
  call_me(-3);
}

fn call_me(num: i8) {
  let x = if num < 0 { -num } else { num };

  if num < 0 {
    for i in num..0 {
      println!("Ring! Call number {} {}", i + 1, x);
    }
  } else {
    for i in 0..num {
      println!("Ring! Call number {} {}", i + 1, x);
    }
  }
}
