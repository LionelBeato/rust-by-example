// here we defined 

mod comments;
mod mod_test;

// initialized functions are created using the keyword "fn"
// these are the building blocks of any and all code
// this main() function is an entry point and serves as the way we run our program
fn main() {
    
    // this is print function is a macro
    // macros are easily accessed pieces of code that are denoted with the exclamation point
    println!("Hello, world!");

    let x:i32 = 2; 
    println!("{} is greater than {}", x, 1);


    comments::comment_on_comments();
    mod_test::test::hello();
    mod_test::hello();


}
