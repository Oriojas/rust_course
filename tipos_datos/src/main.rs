fn main() {

    let sep = "====================";
    
    // Tipos escalares    

    let x_i32 = 5; // por defecto es i32
    let y_u32: u32 = 7;

    println!("{sep}");
    println!("Tipos escalares");
    println!("x_i32: {x_i32}, y_u32: {y_u32}");

    let z = x_i32 + y_u32;

    println!("Suma de x_i32 y y_u32: {z}");
    println!("{sep}");


    // Coma flotante

    let x1_f64 = 5.01;
    let y1_f64: f64 = 7.0;

    println!("{sep}");
    println!("Tipos de coma flotante");
    println!("x1_f64: {:.1}, y1_f64: {:.1}", x1_f64, y1_f64);

    let z1 = x1_f64 + y1_f64;    

    println!("Suma de x1_f64 y y1_f64: {:.2}", z1); // Hacer el ejercicio sin formatear el resultado
    println!("{sep}");

    // Tipo booleano


    let x2_bool = true;
    let y2_bool: bool = false;

    println!("{sep}");
    println!("Tipos booleanos");
    println!("x2_bool: {x2_bool}, y2_bool: {y2_bool}");    
    println!("{sep}");


    // Caracteres individuales
    let x3_char = 'a';
    let y3_char: char = 'b';

    println!("{sep}");
    println!("Tipos de caracteres");
    println!("x3_char: {x3_char}, y3_char: {y3_char}");
    println!("{sep}");

    // Cadenas de texto
    let x3_str = "Hola";
    let y3_str: &str = "Mundo";

    println!("{sep}");
    println!("Tipos de cadenas de texto");
    println!("x3_str: {x3_str}, y3_str: {y3_str}");
    println!("{} {}", x3_str, y3_str);
    println!("{sep}");   


    // Tipos compuestos

    let x4_vec = vec![1, 2, 3];
    let y4_vec: Vec<i32> = vec![4, 5, 6];

    println!("{sep}");
    println!("Tipos compuestos");
    println!("x4_vec: {x4_vec:?}, y4_vec: {y4_vec:?}");
    println!("{sep}"); 
    let pos0 = x4_vec[0];
    println!("x4_vec en la posicion 0: {pos0}");
    println!("{sep}");

    // Tipos de tuplas
    let x5_tuple = (1, 2, 3);
    let y5_tuple: (i32, i32, i32) = (4, 5, 6);   

    println!("{sep}");      
    println!("Tipos de tuplas");
    println!("x5_tuple: {x5_tuple:?}, y5_tuple: {y5_tuple:?}");
    println!("{sep}");
    let tpos0 = x5_tuple.0;
    println!("x5_tuple en la posicion 0: {tpos0}");
    println!("{sep}");
}
