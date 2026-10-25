fn main() {
    let mut name = String::from("Ada");
    let first = &mut name;
    let second = &mut name;
    println!("{first} {second}");
}
