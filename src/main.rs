#![allow(unused)]

mod borrowing;
mod integer_overflow;
mod static_types;
mod lifetime_tracking;
mod error_handling;
mod safe_concurrency;
mod zero_cost_abstraction;

fn main() {
    //Begin with static types.
    //Also variables immutable by default -> no accidental modification.
    static_types::i32_in_i64(); // It's a static type language cant even auto cast i32 into i64
    static_types::signdness(); //Basically fixes the signedness problem due to explicitly, Also no unused variables or methods -> can annotate them with _ to make it explicit.

    //Features of handling over/under-flow
    //integer_overflow::overflow_how_not(); //wont even compile cus smart compiler
    //integer_overflow::overflow_how_not_2(); //will panic in debug and wrap around in release but defined behaviour
    integer_overflow::overflow(); //Features of dealing with it

    //Zero-cost-abstractions
    zero_cost_abstraction::iterator(); //use of iterators to avoid of by one or out of bound
    zero_cost_abstraction::message_passing(); //pass messages safely between threads
    zero_cost_abstraction::option_type(); //if there is a value or no value -> option type
    zero_cost_abstraction::filter(); //avoid explicit loops that might lead to errors (off by one, boundary etc.). Separates conditional checks.

    //Error Handling
    //There are no Exceptions in classical sense
    //error_handling::get_file_unsave(); //will panic because result type is simply unwrapped. But explicitly so.
    error_handling::get_file(); //matches the result type to the result or error thus can gracefully handle errors.

    //Ownership and Borrowing
    borrowing::copy_type(); //primitive types where simply the bytes can be copied to get a copy can be copied.
    borrowing::transfer_ownership(); //other more complex types like strings or structs using one or more non copy type transfer ownership.
    borrowing::borrowing(); //instead of moving the ownership I can borrow a reference to a variable.
    borrowing::try_mut(); //but i cant mutate it by default
    borrowing::mut_reference(); //I can get a mutable reference. But there is only ever one mutable reference allowed.
    borrowing::copy_move_clone_book(); //Recap some types are copy ez. Some types aren't. If I want a copy I can deeply clone but performance.

    //Memory Safety -> lifetime tracking
    //no garbage collector but also no free after use like in c.
    //works in conjunction with ownership and borrowing.
    //compiler tries to infer lifetimes from ownerships and scopes. If it cant you have to define the lifetime.
    println!("{}", lifetime_tracking::get_first_word("Bla tst aldfk")); //Lifetime is inferred. No free after use needed.
    println!("{}", lifetime_tracking::longest("TEST", "more test bla bal")); //Lifetime is annotated. No free after use needed.

    //safe concurrency
    //mostly won't compile if there can be data races.
    //safe_concurrency::how_not_to_create_increment_threads(); //sending a reference to a ver between threads doesn't work. We need to use an Arc and Mutex to guarantee exclusivity
    safe_concurrency::create_increment_threads();
}


