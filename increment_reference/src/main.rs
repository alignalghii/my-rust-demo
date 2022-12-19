const CONSTANT_FOUR: i32 = 4;

fn main() {
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
