fn main() {
    let a = 5;
    let b = 6;
    let c = sumar(a, b);
    let d = restar(a, b);
    let e = multiplicar(a, b);
    let f = dividir(a, b);
    println!("Suma: {}", c);
    println!("Resta: {}", d);
    println!("Multiplicacion: {}", e);
    println!("Division: {}", f);
}

fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

fn restar(a: i32, b: i32) -> i32 {
    a - b
}

fn multiplicar(a: i32, b: i32) -> i32 {
    a * b
}

fn dividir(a: i32, b: i32) -> f64 {
    (a as f64) / (b as f64)
}

