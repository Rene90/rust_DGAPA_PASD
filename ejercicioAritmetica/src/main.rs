enum OperacionesAritmeticas {
    Suma,
    Resta,
    Mult,
    Divi,
}
enum Expresion{
    Operacion{op: OperacionesAritmeticas, derecho:Box<Expresion>, izquierdo:Box<Expresion>},
    Valor(i64)
}
fn eval(e:Expresion)->i64{  
    //Usar un match para poder comparar e con las operacion del enum OperacionesAritmeticas
    match e {
        Expresion::Operacion{op,derecho,izquierdo}=>{
            let derecho = eval(*derecho);
            let izquierdo = eval(*izquierdo);
            match op {
                OperacionesAritmeticas::Suma => izquierdo + derecho,
                OperacionesAritmeticas::Resta => izquierdo - derecho,
                OperacionesAritmeticas::Mult => izquierdo * derecho,
                OperacionesAritmeticas::Divi => izquierdo / derecho,
            }
        }
        Expresion::Valor(v)=> v,
    }
}


fn main() {
   let prueba = eval(Expresion::Operacion{op:OperacionesAritmeticas::Divi, derecho:Box::new(Expresion::Valor(20)), izquierdo:Box::new(Expresion::Valor(40))});
   println!("El resultado es {:?}",prueba);
   let term1 = Expresion::Operacion {
        op: OperacionesAritmeticas::Mult,
        izquierdo: Box::new(Expresion::Valor(10)),
        derecho: Box::new(Expresion::Valor(9)),
    };
    let term2 = Expresion::Operacion {
        op: OperacionesAritmeticas::Mult,
        izquierdo: Box::new(Expresion::Operacion {
        op: OperacionesAritmeticas::Resta,
        izquierdo: Box::new(Expresion::Valor(3)),
        derecho: Box::new(Expresion::Valor(4)),
    }),
        derecho: Box::new(Expresion::Valor(5)),
    };
    let prueba2 = eval(Expresion::Operacion{op: OperacionesAritmeticas::Suma,izquierdo:Box::new(term1),derecho:Box::new(term2)});
    println!("El resultado es {:?}",prueba2);

}
