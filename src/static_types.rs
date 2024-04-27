pub fn signdness(){
    let unsigned_val: u32 = 150;
    let signed_val: i32 = -100;

    //will panic
    //let signed: i32 = unsigned_val;
    //let unsigned u32 = signed_val;

    // Explicit casting from unsigned to signed (safe in this context)
    let signed_from_unsigned: i32 = unsigned_val as i32;
    println!("Signed from Unsigned: {}", signed_from_unsigned);

    // Explicit casting from signed to unsigned (logical error potential)
    let unsigned_from_signed: u32 = signed_val as u32;
    println!("Unsigned from Signed: {}", unsigned_from_signed); // This will print a large number due to underflow

    // Handling the conversion safely
    let safe_unsigned_from_signed: u32 = if signed_val >= 0 {
        signed_val as u32
    } else {
        0 // Alternatively, handle this case as an error or default value
    };
    println!("Safely converted Unsigned from Signed: {}", safe_unsigned_from_signed);
}
pub fn i32_in_i64(){
    let smol: i32 = 128;

    //wont work, wont even compile
    //let big: i64 = smol; 
    
    let big: i64 = smol.into();

    println!("i32 in i64: {}", big);
}
