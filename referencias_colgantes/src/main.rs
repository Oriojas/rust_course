fn main() {
    // let referencia_a_la_nada = colgar();
    let referencia_a_la_nada = no_colgante();
}

// fn colgar() -> &String {
//     let s = String::from("hola");

//     &s
// }

fn no_colgante() -> String {
    let s = String::from("hola");

    s
}