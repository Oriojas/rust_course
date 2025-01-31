// Referencia de variables
// fn main() {
//     let s1 = String::from("hola");

//     // permite referenciar la variable y no tomar su propiedad
//     let len = calcular_longitud(&s1); // referencia a la variable

//     println!("La longitud de '{s1}' es {len}.");
// }

// fn calcular_longitud(s: &String) -> usize {
//     s.len()
// }

// Referencias Mutables
fn main() {
    let mut s = String::from("hola"); // se crea referencia mutable
    println!("{}", s);

    modificar(&mut s);
    println!("{}", s);
}

fn modificar(un_string: &mut String) { // permite mutar el valor que presta
    un_string.push_str(", mundo");
}