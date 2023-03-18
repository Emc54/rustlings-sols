// macros2.rs
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a hint.

// I AM DONE


// This macro was below the function in this exercise. 
// The solution was to move it before it gets called.

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}



fn main() {
    my_macro!();
}

