fn main() {
    let mut s = String::from("hello world");

    {
        let word = first_word(&s); // word obtendrá el slice "hello"
        println!("La primera palabra es '{}'", word);
        // word deja de ser válido aquí
    }

    s.clear(); // esto "vacía" el String, dejando s igual a ""
    println!("El string ahora está vacío: '{}'", s);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}