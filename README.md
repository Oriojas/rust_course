# rust_course

* Podemos crear un proyecto usando `cargo new`.
* Podemos construir un proyecto usando `cargo build`.
* Podemos construir y ejecutar un proyecto en un solo paso usando `cargo run`.
* Podemos construir un proyecto sin producir un binario para verificar errores usando `cargo check`.
* En lugar de guardar el resultado de la compilación en el mismo directorio que nuestro código, Cargo lo almacena en el directorio `target/debug`.
* Cuando tu proyecto finalmente esté listo para su lanzamiento, puedes usar `cargo build --release` para compilarlo con optimizaciones.


## Mutabilidad

https://book.rustlang-es.org/ch03-01-variables-and-mutability

Cuando una variable es inmutable, una vez que un valor está vinculado a un nombre, no puede cambiar ese valor. 

Primero, no se le permite usar mut con constantes. Las constantes no son solo inmutables por defecto, siempre son inmutables. Declara constantes usando la palabra clave const en lugar de la palabra clave let, y el tipo del valor debe estar anotado

### Shadowing

En Rust, shadowing es una característica que permite declarar una nueva variable con el mismo nombre que una variable anterior. La nueva variable "sombrea" a la anterior, lo que significa que la variable anterior deja de estar accesible en el ámbito actual. Esto puede ser útil para transformar el valor de una variable sin tener que crear un nuevo nombre.

El Shadowing es diferente de marcar una variable como mut porque obtendremos un error de tiempo de compilación si accidentalmente intentamos volver a asignar esta variable sin usar la palabra clave let. Al usar let, podemos realizar algunas transformaciones en un valor, pero la variable debe ser inmutable después de que se hayan completado esas transformaciones.

La otra diferencia entre mut y el shadowing es que, debido a que efectivamente estamos creando una nueva variable cuando usamos la palabra clave let nuevamente, podemos cambiar el tipo de valor pero reutilizar el mismo nombre. Por ejemplo, digamos que nuestro programa le pide al usuario que muestre cuántos espacios desea entre algún texto ingresando caracteres de espacio, y luego queremos almacenar esa entrada como un número:

```rust
let spaces = "   ";
let spaces = spaces.len();
```

La primera variable spaces es de tipo string y la segunda variable spaces es de tipo numérico. El shadowing nos ahorra tener que pensar en nombres diferentes, como spaces_str y spaces_num; en su lugar, podemos reutilizar el nombre más simple spaces. Sin embargo, si intentamos usar mut para esto, como se muestra aquí, obtendremos un error de tiempo de compilación:

```rust
// Error
let mut spaces = "   ";
spaces = spaces.len();

```


https://book.rustlang-es.org/ch03-02-data-types
