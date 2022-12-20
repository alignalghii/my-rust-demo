fn main() {
    let z1 = GaussComplex {re: 2, im: 3};
    let z2 = GaussComplex {re: 10, im: 50};
    let z3 = z1.add_semidestructive(z2);
    println!("(2 + 3i) + (10 + 50i) = {z3:?}");
    println!("z1 i.e. {z1:?} is still in scope: its ownership has not been moved away");
    // println!("z2 i.e. {z2:?} is no more in scope: its ownership *has* been moved away");

    let u1 = GaussComplex {re: 2, im: 3};
    let u2 = GaussComplex {re: 10, im: 50};
    let u3 = u1.add(&u2);
    println!("(2 + 3i) + (10 + 50i) = {u3:?}");
    println!("u1 i.e. {u1:?} is still in scope: its ownership has not been moved away");
    println!("u2 i.e. {u2:?} is still in scope: its ownership has not been moved away");

    let v1 = GaussComplex {re: 2, im: 3};
    let v2 = GaussComplex {re: 10, im: 50};
    let v3 = v1.add_selfdestructive(v2);
    println!("(2 + 3i) + (10 + 50i) = {v3:?}");
    // println!("v1 i.e. {v1:?} is no more in scope: its ownership *has* been moved away");
    // println!("v2 i.e. {v2:?} is no more in scope: its ownership *has* been moved away");

    let mut w1 = GaussComplex {re: 2, im: 3};
    let w2 = GaussComplex {re: 10, im: 50};
    w1.add_to(&w2);
    println!("(2 + 3i) + (10 + 50i) = {w1:?}");
    println!("w1 i.e. {w1:?} is still in scope: its ownership has not been moved away");
    println!("w2 i.e. {w2:?} is still in scope: its ownership has not been moved away");
}

#[derive(Debug)]
struct GaussComplex {re: i32, im: i32}

impl GaussComplex {
    fn add_semidestructive(&self, other: GaussComplex) -> GaussComplex {
        GaussComplex {re: self.re + other.re, im: self.im + other.im}
    }

    fn add(&self, other: &GaussComplex) -> GaussComplex {
        GaussComplex {re: self.re + other.re, im: self.im + other.im}
    }

    fn add_selfdestructive(self, other: GaussComplex) -> GaussComplex {
        GaussComplex {re: self.re + other.re, im: self.im + other.im}
    }

    fn add_to(&mut self, other: &GaussComplex) {
        self.re += other.re; // dot operator makes automatic dereferencing
        self.im += other.im; // dot operator makes automatic dereferencing
    }
}
