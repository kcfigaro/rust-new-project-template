/* A Marco Polo Game

If the name Marco is given, the response should be Polo. If the name Marco is not given, the response should be What?.

*/

// The macro_rules! macro is how you create macros in Rust. When you invoke a macro, it will replace the code with the contents of the macro. Let’s look at a simple example of a macro that creates a function named five that returns the value 5:
pub fn marco_polo(name: &str) -> String {
    if name == "Marco" {
        "Polo".to_owned()
    } else {
        "What?".to_owned()
    }
}
