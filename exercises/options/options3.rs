// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT_DONE

struct Point {
    x: i32,
    y: i32,
}

fn print_my_string (str: String) {
    print!("{}", str);
}

fn main() {
    let x: f32 = 3.0;
    let nm = "Poliana";
    let name = "Steve".to_string();

    print!("name {}", name);

    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => {panic!("no match!")},
    }
    y; // Fix without deleting this line.
}
