use std::io;  // Entrada por consola
use rand::Rng;  // Generar número aleatorio

fn main() {
    let caracteres = "abcdefghijklmnopqrstuvwxy0123456789zABCDEFGHIJKLMNOPQRSTUVWXYZ+-*.,()%&$#|[]";


    let mut clave_generada = String::new();

    for i in longitud_pass()..longitud_pass_2() {
        let caracter = rand::thread_rng().gen_range(0..caracteres.len());  // Genera un número aleatorio entero entre 0 y la longitud de la variable caracteres
        clave_generada.push_str(&caracteres[caracter..caracter+1]); // Inserta el caracter seleccionado al final del string
    }

    println!("{}", clave_generada);
}

fn longitud_pass() -> i16 {
    1
}
fn longitud_pass_2() -> i16 {  // Función que toma la longitud de la contraseña por consola

    println!("¿Que tan larga debe ser su contraseña?");

    let mut adivinar = String::new(); // Primeramente toma un string

    io::stdin()
        .read_line(&mut adivinar)
        .expect("Error al leer la línea");  // Se inserta el valor en consola

    let adivinar: i16 = match adivinar.trim().parse() {
        Ok(num) => num,  //  Analiza el string en busca de un valor numérico, si lo encuentra lo convierte, si no, regresa un 0
        Err(_) => 0,
    };

    adivinar + 1  // Retorna el valor convertido a integer + 1
}