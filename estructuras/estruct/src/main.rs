//Estructuras 
#[derive(Clone)] //trrait clone, indica que Profesor puede clonado, y habilita el metodo clone()
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
}
