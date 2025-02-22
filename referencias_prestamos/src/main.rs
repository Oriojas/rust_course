// fn main() {
//     let s1 = String::from("hola");

//     let len = calcular_longitud(&s1);

//     println!("La longitud de '{s1}' es {len}.");
// }

// fn calcular_longitud(s: &String) -> usize {
//     s.len()
// }

fn main(){
    let mut s = String::from("hello");

    let r1 = &s; // no hay problema
    let r2 = &s; // no hay problema
    println!("{r1} y {r2}");
    // variables r1 y r2 no se usaran más a partir de aquí

    let r3 = &mut s; // no hay problema
    println!("{r3}");

}