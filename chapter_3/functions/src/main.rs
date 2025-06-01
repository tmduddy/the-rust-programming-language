fn five() -> i32 {
    5
}

fn main() {
    let x = plus_one(five());

    println!("the value of x is {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
