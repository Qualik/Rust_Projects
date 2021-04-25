// fn main() {

//     {
//     // annonymous scope
//     let x = 42;

//     println!("x: {}", x);
//     }
//     println!("x: {}", x); // The compiler will error as x is not in scope
//     // The variable x used in the 2nd 'println!' is not in scope
//     //drop - deconstructor
// }
#[derive(Debug)]
struct DropMe;

impl Drop for DropMe {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

fn main() {
    println!("Begin outer scope.");

    {
        println!("Begin inner scope.");

        let x = DropMe;

        println!("x: {:?}", x);
    }

    println!("End outer scope.");
}








