// Refactoring with Tuples

fn main() {
    let rect1 = (30, 50);
    // let width1 = 30;
    // let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        // area(width1, height1)
        area(rect1)
    );
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }
fn area(dimensions: (u32, u32)) -> u32 {
    // dimensions.0 * dimensions.1
    let (width, height) = dimensions;
    width * height
// Shows 2 different ways of accessing the two values in a Tuple
}
