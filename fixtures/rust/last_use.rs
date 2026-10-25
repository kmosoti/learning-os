fn main() {
    let mut name = String::from("Ada");
    let borrowed = &name;
    println!("{borrowed}");
    name.push('!');
    println!("{name}");
}
