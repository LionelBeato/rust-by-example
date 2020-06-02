// primitives in any language is the most atomic type of data
// Rust makes distinction between two types
// Scalar types: the most primitive, representing a singularity
// Compound types: collection types, representing an assortment

// Scalar:

// integers:

// signed~
fn signed_types() {
    let a: i8;
    let b: i16;
    let c: i32;
    let d: i64;
    let e: i128;
    let f: isize;
}

// unsigned~
fn unsigned_types() {
    let a: u8;
    let b: u16;
    let c: u32;
    let d: u64;
    let e: u128;
    let f: usize;
}

// floating~
fn floating(){
    let a: f32;
    let b: f64;
}

// char:
fn char(){
    let a: char = 'a';
    let b: char = 'b';
}

// bool:
fn bool(){
    let a: bool = true;
    let b: bool = false;
}

// unit:
fn unit(){
    let a: (); 
}

// Compound

// arrays
fn array(){
    let a = [1,2,3];
}

// tuples
fn tuple(){
    let a = (1, true)
}
