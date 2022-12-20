fn main() {
    let mut s = String::from("Hello");
    println!("{s}");
    println!("{s}"); // a macro does not necessarily move
    s.push_str("Bello"); // method does not move its "self" object: it is not like a value argument, it is much more like a reference
    println!("{s}");
    s.push_str("Bello");
    println!("{s}");
    s.push_str("Bello");
    println!("{s}");
    let s2 = id(s);
    // println!("{s}");
    println!("{s2}");
}

fn id(a: String) -> String {a}
