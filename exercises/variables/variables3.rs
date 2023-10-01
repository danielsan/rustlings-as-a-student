// variables3.rs
//
// Execute `rustlings hint variables3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT_DONE

fn main() {
    let mut x: i64 = 16384;
    x *= x;
    x *= x;  //    72,057,594,037,927,936
    x *= 64; // 4,611,686,018,427,387,904
    println!("Number {}", x);
}
