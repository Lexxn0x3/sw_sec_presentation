#![allow(unused)]

mod borrowing;
mod integer_overflow;
mod static_types;
mod lifetime_tracking;
mod error_handling;

fn main() {
    static_types::i32_in_i64();
    static_types::signdness();
    borrowing::transfer_ownership();
    borrowing::try_mut();
    borrowing::copy_move_clone_book();

    println!("{}", lifetime_tracking::get_first_word("Bla tst aldfk"));
    println!("{}", lifetime_tracking::longest("TEST", "more test bla bal"));
    
    error_handling::get_file();
    error_handling::get_file_unsave();

}


