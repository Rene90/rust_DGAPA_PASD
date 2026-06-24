//Estructuras 
/*#[derive(Clone)] //trrait clone, indica que Profesor puede clonado, y habilita el metodo clone()
struct Profesor {
    nombre: String,
    edad: u8,
    jubilado: bool,
}

//estructura Tupla , cuando los nombres de los campos definidos no son importantes
struct Tuplaejemplo(i32,i32);
struct Newtons(f64);
struct LibrasFuerza(f64); //wrappers o envolturas para definir nuevos tipos de datos

fn describir_profesor(prof: Profesor){
    println!("El nombre del profesor es {}, tiene {} años", prof.nombre, prof.edad);
}


fn main() {
    let mut profesorEliuth = Profesor {
        nombre: String::from("Eliuth"),//trade String, un trade una funcion definida que se implementa con alguna variable
        edad:35,
        jubilado:false,
    };
    let tupejemplo = Tuplaejemplo(10,20);
    println!("{}",tupejemplo.0);
    let mut profesorElithClon = profesorEliuth.clone(); // opcion del compilador , usar clone
    
    
    describir_profesor(profesorEliuth); //la referencia hace posible el prestamo de la variable profesorEliuth, si no se presta, la funcion describir_profesor adquiere el ownership y la variable muere al finalizar la funcion
    profesorElithClon.edad = 60;
    profesorElithClon.jubilado = true;
    describir_profesor(profesorElithClon);
}*/
// tipo de estructura enum
/*#[derive(Debug)] //traits  
enum Direccion {
    Derecha,
    Izquierda,
}
#[derive(Debug)]  //traits
enum MovimientoJugador {
    Pase,                   //variable
    Correr(Direccion),       //tupla variante
    MovimientoRapido {x:u32, y:u32}  //estructura variante
}
//Alias
enum ObjetoDireccion{
    Enfrente,
    Atras
}
//Definir el tipo de alias

type Objeto = ObjetoDireccion;
//Esto es mas util para variables mas complejas
use std::cell::RefCell;
use std::sync::{Arc,RwLock};
type Inventario = RwLock<Vec<Arc<RefCell<Item>>>>;
//const para declarar constantes en el programa, ahorra tiempo en ejecuciion ya que son evaluada en la compilacion*/

const EDAD_PROMEDIO: i32 = 50;
const FUNCION_CONSTANTE: u8 = funcion_constante_valorentero();

const fn funcion_constante_valorentero() ->u8{

    if EDAD_PROMEDIO >10{1} else{0}
}

fn main (){
    /*let direc = Direccion::Izquierda;
    let mut movimiento_jugador: MovimientoJugador = MovimientoJugador::Correr(direc);
    movimiento_jugador = MovimientoJugador::MovimientoRapido{x:10,y:15};
    println!("El movimiento del jugador es el siguiente: {movimiento_jugador:?}");
    let movimiento_rapido_jugador:MovimientoJugador= MovimientoJugador::MovimientoRapido{x:25,y:10};
    println!("El movimiento del jugador es el siguiente: {movimiento_rapido_jugador:?}");*/
    let mut valor_funcion_constante = FUNCION_CONSTANTE;
    println!("Este valor viene de una funcion constante {valor_funcion_constante:?}");
    println!("Este es el valor constante {EDAD_PROMEDIO:?}");
}
