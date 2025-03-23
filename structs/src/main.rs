
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// fn main() {
//     let _user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };


// println!("Estado: {}",_user1.active);
// println!("Nombre de usuario: {}",_user1.username);
// println!("Mail de usuario: {}",_user1.email);
// println!("Cuentas de inicio de sesion: {}",_user1.sign_in_count);

// }

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(String::from("someone@example1.com"),
     String::from("someusername123"));
    let user2 = build_user(String::from("someone@example2.com"),
     String::from("someusername223"));

    println!("===========USUARIO1==========="); 
    println!("Estado: {}", user1.active);
    println!("Nombre de usuario: {}", user1.username);
    println!("Mail de usuario: {}", user1.email);
    println!("Cuentas de inicio de sesion: {}", user1.sign_in_count);

    println!("===========USUARIO2==========="); 
    println!("Estado: {}", user2.active);
    println!("Nombre de usuario: {}", user2.username);
    println!("Mail de usuario: {}", user2.email);
    println!("Cuentas de inicio de sesion: {}", user2.sign_in_count);
}
