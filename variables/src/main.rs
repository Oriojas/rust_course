// // Codigo de ejemplo que no funciona

// fn main() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// Codigo arreglado

fn main() {

    println!("============mutability==============");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The new value of x is: {x}");

    println!("============shadowing===============");

    let y = 5;

    let y = y + 1; // y = 6

    {
        let y = y * 2; // y = 12
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");
}