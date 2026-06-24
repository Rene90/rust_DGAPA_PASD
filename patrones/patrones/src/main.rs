fn takes_tuple(tuple: (char, i32, bool)) {
    let achar = tuple.0;
    let bentero = tuple.1;
    let cbooleano = tuple.2;
    //destructuarion patron irretubale
    let (achar,bentero,cbooleano) = tuple;
    let (_,bentero,cbooleano) = tuple;
    let (.., cbooleano) = tuple;

}
/* 
fn main() {
    //match nos permite comparar un valor contra uno o mas patrones.
    /*if else  if else.... else -> match, uso de un switch en Python */
    let valorAcomparar = 'x';
    match valorAcomparar {
        'e' => println!("Exit"),
        'a' | 's' | 'w' =>println!("Coincide con a s o w "),
        key if key.is_lowercase() => println!("No es letra capital {key}"),
        _ => println!("Otro caso aparte"),
    }

}
*/
/* 
fn main() {
    let input = 'a';
    match input {
        key if key.is_lowercase() => println!("Uppercase"),   //aqui termina el bloque del match porque entrga un true
        key => if input == 'q' { println!("Quitting") },
        _   => println!("Bug: this is never printed"),
    }
}*/

//structs  => tambien pueden ser destructurados siguiendo su patron
/* 
struct Movimiento{
    delta: (i32,i32),
    repeat: u32,
}
fn main() {
    let mov = Movimiento{delta:(1,2),repeat:4};
    match mov{
        Movimiento {delta:(0,0),..} => println!("Solo se comparo el delta"),
        Movimiento {delta:(x,0),repeat} => println!("Solo se comparo el delta"),
        Movimiento {delta:(x,y),repeat} => println!("Realmente no nos importan los valores, siempre entrara aqui"), //caso de uso de 'enlazamientos' en  un match para desstructurar estructuras siguiendo sus patrones
        _ =>println!("Otro caso"),
    }
}*/


//Destructuracion de un enum
/*
enum Result {
    Ok(i32),
    Err(String), //Wrappers para definir nuevas variables
}
fn divisible_dos (n:i32)->Result{
    if n % 2 == 0 {
        Result::Ok(n / 2)
    } else {
        Result::Err(format!("No se puede divir entre dos"))
    }


} */
/*fn main(){
    let inputn = 33;
    match divisible_dos(inputn){
        //usar bindings para hacer las comparaciones
    }

}*/
/*if let ejecuta un codigo dependiendo de la comparacion de un valor con un patron*/
/*
use std::time::Duration;

fn dormir_por(secs: f32){
    let resultado = Duration::try_from_secs_f32(secs);
    if let Ok(duration) = resultado{
        std::thread::sleep(duration);
        println!("Se ha dormido por {duration:?}");
    }



}
fn main(){
    dormir_por(6.0);
}

    */
// enums pre-definidos
/*
enum Option{
    Some(T),
    None
}
enum Result{
    Ok(T),
    Err(T),
}*/
//let else 
fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
    let s = if let Some(s) = maybe_string {
        s
    } else {
        return Err(String::from("got None"));
    };

    let first_byte_char = if let Some(first) = s.chars().next() {
        first
    } else {
        return Err(String::from("got empty string"));
    };

    let digit = if let Some(digit) = first_byte_char.to_digit(16) {
        digit
    } else {
        return Err(String::from("not a hex digit"));
    };

    Ok(digit)
}

fn main() {
    println!("result: {:?}", hex_or_die_trying(Some(String::from("foo"))));
}
