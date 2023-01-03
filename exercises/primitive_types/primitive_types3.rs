// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.

// I AM DONE

fn main() {
    let a:[i32; 100] = [5; 100];

    if a.len() >= 100 {
        println!("{:?}", a);
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
