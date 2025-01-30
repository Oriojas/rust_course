fn main() {
    let s1 = String::from("hola");

    // permite referenciar la variable y no tomar su propiedad
    let len = calcular_longitud(&s1); // referencia a la variable

    println!("La longitud de '{s1}' es {len}.");
}

fn calcular_longitud(s: &String) -> usize {
    s.len()
}