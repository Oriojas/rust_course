fn main() {
    print_labeled_measurement(5, 'h');


    let y = {
        let x = 3;
        x + 1
    };

    let z = 12; // sentencia

    let x_plus_one = plus_one(7);

    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
    println!("The value of x_plus_one is: {x_plus_one}");

}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1 // expresión
}