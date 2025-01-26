// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;
//         println!("Counter: {counter}");

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("final result is {result}");
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value is: {}", a[index]);

//         index += 1;
//     }
// }

fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}

// enum MyEnum {
//     Int(i32),
//     Str(&'static str), // define una variante del enum llamada Str que contiene una referencia a una cadena de texto con una vida estática.
// }

// fn main() {
//     let a = vec![
//         MyEnum::Int(10),
//         MyEnum::Int(20),
//         MyEnum::Str("treinta"),
//         MyEnum::Int(40),
//         MyEnum::Int(50),
//     ];

//     for element in &a {
//         match element {
//             MyEnum::Int(val) => println!("the value is: {}", val),
//             MyEnum::Str(val) => println!("the value is: {}", val),
//         }
//     }
// }