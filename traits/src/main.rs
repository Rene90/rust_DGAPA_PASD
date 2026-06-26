trait Mascota{
    //funciones que se pueden implementar con este trait
    fn hablar(&self)->String;
    fn saludar(&self);

}

struct Perro{
    nombre:String,
    edad: i8,
}

impl Mascota for Perro{
    fn hablar(&self) ->String{
        format!("Ladrido del perro cuyo nombre es {}!!",self.nombre)
    }
    fn saludar(&self){
        println!("Que bonito perro, te estoy saludando, me puedes ladrar {}", self.hablar())
    }
}


fn main() {
    let huesos = Perro{nombre:String::from("HUESOS"),edad:10};
    dbg!(huesos.hablar());
    huesos.saludar();
}
