fn main() {
    let mut name = String::from("Ada");
    let borrowed = &name;
    name.push('!');
    println!("{borrowed}");
}
