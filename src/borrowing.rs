pub fn transfer_ownership(){ 
    let s1 = String::from("Hello");
    let s2 = s1;  // Ownership of the string is transferred from s1 to s2

    //println!("{}", s1); // This line would cause a compile-time error because s1 no longer owns the string.
    println!("{}", s2);  // This works perfectly, s2 now owns the data.
}
pub fn borrowing(){
    let s1 = String::from("Hello");
    let s2 = &s1;  // s2 is a reference to s1, s1 is borrowed

    println!("{}", s1); // This is perfectly fine, s1 is still valid and hasn't been moved.
    println!("{}", s2); // This is also fine, s2 is a valid reference to s1.
                        // can have unlimited references
}
pub fn try_mut(){
    let s1 = String::from("Hello");
    let s2 = &s1;
    //let mut s3 = s1;  // A C programmer might try to mutate s1 directly despite it being borrowed.

    println!("{}", s2); // This would cause a compile-time error in Rust.
} 
pub fn mut_reference(){
    let mut s1 = String::from("Hello");
    let mut s2 = &mut s1;  // s2 is a mutable reference to s1
                       // only 1 mutable reference allowed
    //let s3 = &mut s1;

    s2.push_str(", world!"); // Modifying s1 through its mutable reference s2
    println!("{}", s2); // This works and prints "Hello, world!"
}
#[derive(Debug, Clone)]
struct Book {
    title: String,
    pages: u32,
}

pub fn copy_move_clone_book() {
    let num1 = 42; // i32, which is Copy
    let num2 = num1; // num1 is copied to num2

    println!("num1: {}, num2: {}", num1, num2); // Both can be used; num1 was copied

    let book1 = Book {
        title: "Rust Programming".to_string(),
        pages: 256,
    };
    let book2 = book1; // book1 is moved to book2

    //println!("book1: {:?}", book1); // This line would cause a compile error because book1 has been moved
    println!("book2: {:?}", book2); // Only book2 can be used; book1 was moved
                                    
    let book3 = book2.clone();
    println!("book2: {:?} book3: {:?}", book2, book3);
}
