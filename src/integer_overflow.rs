pub fn overflow(){
    let a: i32 = i32::MAX;

    // Checked addition
    let checked_sum = a.checked_add(1);
    println!("Checked sum: {:?}", checked_sum);  // Outputs "None"

    // Wrapping addition
    let wrapping_sum = a.wrapping_add(1);
    println!("Wrapping sum: {}", wrapping_sum);  // Outputs the minimum value for i32

    // Saturating addition
    let saturating_sum = a.saturating_add(1);
    println!("Saturating sum: {}", saturating_sum);  // Outputs the maximum value for i32

    // Overflowing addition
    let (overflowing_sum, overflowed) = a.overflowing_add(1);
    println!("Overflowing sum: {}, overflowed: {}", overflowing_sum, overflowed);  // Outputs the minimum value for i32 and true
}
pub fn overflow_how_not(){
    //let a: i32 = i32::MAX;
    let a: i32 = 3; //comment out this line and comment in above
    let b = a + 1;  // This line will cause a panic in debug mode or wrap in release mode -> always defined behaviour

    println!("{}", b);
}
pub fn overflow_how_not_2(){
    let mut a: i32 = 1;
    
    for _i in 1..=1000{
        a += a;
    }

    println!("{}", a);
}

