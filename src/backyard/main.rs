// The use keyword is comparable to using in C# or import in Java.
// crate is a literal for the base of the current crate.
// Separated by the :: Operator are sub-/modules and the type to "import".
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    // Asparagus could have been referenced by using crate::garden::vegetables::Asparagus instead.
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}