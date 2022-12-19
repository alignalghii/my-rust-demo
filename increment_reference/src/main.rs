fn main() {
    demo_for_integer();
    demo_for_string();
}

// Demo for strings

fn demo_for_string() {
    let mut s = String::from("Hello");
    let sref1 = &mut s; // The pointer itself is immutable, it is the referent access that is mutable
    // let sref2 = &s;
    println!("sref1: {sref1}");
    sref1.push_str(" Mark"); // althought the pointer is immutable, value reload is allowed
    println!("{}", sref1);
    sref1.push_str(" coming");
    println!("{}", sref1);

    // let mut u = String::from("Hello");
    // sref1 = &mut u; // reseating is not allowed, because the pointer itself is immutable (it is the referent access that is mutable)

    let v = String::from("Hello"); // immutable val
    let w = String::from("Bello"); // immutable val
    let mut reseatable: &String = &v; // mutable i.e. reseatable pointer on (contentually) immutable String
    println!("{reseatable}");
    reseatable = &w;
    println!("{reseatable}");
}

// Demo for integers

const CONSTANT_FOUR: i32 = 4;

fn demo_for_integer() {
    let mut n: i32 = 5;
    print_val(n);
    print_ref(&n);
    print_incremented_copied_val(n);
    inc_ref(&mut n);
    println!("{n}");
    print_ref(&n);
    print_val(n);
    print_reseated_ref(&n);
}

fn print_val(n: i32){
    println!("{n}");
}

fn print_ref(nref: &i32) {
    println!("{nref}");
}

fn print_incremented_copied_val(mut n: i32) {
    n += 1;
    println!("{n}");
}

fn inc_ref(nref: &mut i32) {
    *nref += 1;
}

fn print_reseated_ref(mut nref: &i32) {
    println!("{nref}");
    nref = &CONSTANT_FOUR; // `nr = &4` works too, but `&local_variable` bumps into lifetime problems
    println!("{nref}");
}
