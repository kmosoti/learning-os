fn main() {
    let name = String::from("Ada");
    let borrowed = &name;
    println!("{borrowed} {name}");
}
