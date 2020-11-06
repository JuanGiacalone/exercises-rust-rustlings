// macros1.rs
// Make me compile! Execute `rustlings hint macros1` for hints :)


macro_rules! MyMacro  {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    MyMacro!();
}
