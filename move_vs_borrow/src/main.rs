fn main() {
    println!("+---------------------+");
    println!("| Without references: |");
    println!("+---------------------+");
    let s1: String = String::from("Hello");
    let mut s2: String = String::new();
    println!("s1 = {s1}, s2 = {s2}");
    s2 = s1; // move, afther that s1 is no longer valid
    // println!("{s1}, {s2}");
    println!("s1 is invalid, s2 = {s2}");
    let s3 = s2;
    println!("Noe s2 is no longer valid, it has been moved to s3: {s3}");

    println!("");
    println!("Naive functions can iinvalidate passed arguments");
    println!("");
    let b = false;
    naive_print_bool(b);
    naive_print_bool(b);
    ownership_returning_print_bool(b);
    ownership_returning_print_bool(b);
    let s4 = String::from("Hello");
    naive_print(s4);
    // naive_print(s4);
    // println!("{s4}");
    let s5 = String::from("Hello");
    println!("Length: {}", naive_length(s5)); 
    // println!("{s5}");
    println!("");
    println!("ownership-returning variants of naive functions");
    println!("");
    let s6 = String::from("Hello");
    let s62 = ownership_returning_print(s6);
    let s63 = ownership_returning_print(s62);
    let s64 = ownership_returning_print(s63);
    let s65 = ownership_returning_print(s64);
    let s7  = ownership_returning_print(s65);

    let (n, s72) = ownership_returning_length(s7);
    println!("The {s72} string has length {n}");

    println!("");
    println!("+------------------+");
    println!("| With references: |");
    println!("+------------------+");
    let sr1 = &String::from("Hello");
    let sr2 = sr1;
    println!("{sr1}, {sr2}");
}

fn naive_print(s: String) {
    println!("{s}");
}

fn naive_length(s: String) -> usize {
    s.len()
}

fn ownership_returning_print(s: String) -> String {
    println!("{s}");
    s
}

fn ownership_returning_length(s: String) -> (usize, String) {
    (s.len(), s)
}

fn naive_print_bool(b: bool) {
    println!("{b:?}");
}

fn ownership_returning_print_bool(b: bool) -> bool {
    println!("{b}");
    b
}
