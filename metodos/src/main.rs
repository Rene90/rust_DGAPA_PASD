#[derive(Clone)] //Trait 
struct Carro {
    modelo: String,
    atributos:Vec<i32>,
}

/*  struct Vec{
    valor: T
}
    impl Vec{
     fn new (valor...)
    }
*/

impl Carro{
    // funcion constructora
    fn new(name: &str)-> Self{
        Self { modelo: String::from(name), atributos: Vec::new()}
    }
    //funcion que modifica campo de carro
    fn adicionar_atributo(&mut self, atributo: i32){
        self.atributos.push(atributo);
    }
    //funcion que lee solamente un campo de carro
    fn leer_atributos(&self){
        println!("Numero de atributos {} y este es el modelo del carro {}", self.atributos.len(),self.modelo );
        for elemento in self.atributos.iter(){
            println!("{}", elemento);
        }
    }
}



fn main() {
    let mut carroRene = Carro::new("Chevy");
    let mut carroNuevoOswaldo = carroRene.clone();
    carroRene.adicionar_atributo(10);
    carroRene.adicionar_atributo(20);
    carroRene.leer_atributos();
}
