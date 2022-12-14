const IV: i32  = 7;
const UV: u8   = 3;
const BV: bool = true;
const TV: (u8, bool) = (127, false);
const AV: [bool; 3] = [false, true, false];
// const AM: [bool] = [false, true, false];
// const AMS: [bool; 5] = [false, true, false];

const IR: &i32  = &7;
const UR: &u8   = &3;
const BR: &bool = &true;
const BSR: &str  = "Hello";
const BSR1: &str  = &"Hello";
const BSR2: &str  = &&"Hello";
const BSR25: &&str  = &&"Hello";
const TR: &(u8, bool) = &(127, false);
const AR: &[bool] = &[false, true, false];
const AR1: &[bool; 3] = &[false, true, false];
// const AM1: &[bool; 5] = &[false, true, false];

// Mismatches:
// const BM1:  bool = &true;
// const BM2: &bool =  true;
// const AM1:  [bool; 3] = &[false, true, false];
// const AM2: &[bool] =  [false, true, false];

fn main() {
    println!("Integer value: {IV}, bool value: {BV}, unsigned byte value: {UV}, tuple value: {TV:?}, array value: {AV:?}");
    println!("Integer reference: {IR}, bool reference: {BR}, unsigned byte reference: {UR}, string slice: {BSR}  (and {BSR1:?}, {BSR2:?}, {BSR25:?}), tuple reference: {TR:?}, array reference: {AR:?} (and {AR1:?})");
}
