use std::rc::Rc;

#[derive(Debug, Clone, Copy)] // Añadimos Clone y Copy para comodidad
enum OperacionesAritmeticas {
    Suma,
    Resta,
    Mult,
    Divi,
}

// Cambiamos Box por Rc en los campos derecho e izquierdo
enum Expresion {
    Operacion {
        op: OperacionesAritmeticas,
        derecho: Rc<Expresion>,
        izquierdo: Rc<Expresion>,
    },
    Valor(i64),
}

// Ahora eval recibe una referencia para no consumir la expresión
fn eval(e: &Expresion) -> i64 {
    match e {
        Expresion::Operacion { op, derecho, izquierdo } => {
            // Desreferenciamos el Rc para obtener &Expresion mediante as_ref()
            let der = eval(derecho.as_ref());
            let izq = eval(izquierdo.as_ref());
            match op {
                OperacionesAritmeticas::Suma => izq + der,
                OperacionesAritmeticas::Resta => izq - der,
                OperacionesAritmeticas::Mult => izq * der,
                OperacionesAritmeticas::Divi => izq / der,
            }
        }
        Expresion::Valor(v) => *v,
    }
}

fn main() {
    // --- Ejemplo 1: División ---
    let prueba = eval(&Expresion::Operacion {
        op: OperacionesAritmeticas::Divi,
        derecho: Rc::new(Expresion::Valor(20)),
        izquierdo: Rc::new(Expresion::Valor(40)),
    });
    println!("El resultado de 40 / 20 es {:?}", prueba);

    // --- Ejemplo 2: Expresión compuesta: (10 * 9) + ((3 - 4) * 5) ---
    let term1 = Expresion::Operacion {
        op: OperacionesAritmeticas::Mult,
        izquierdo: Rc::new(Expresion::Valor(10)),
        derecho: Rc::new(Expresion::Valor(9)),
    };

    let term2 = Expresion::Operacion {
        op: OperacionesAritmeticas::Mult,
        izquierdo: Rc::new(Expresion::Operacion {
            op: OperacionesAritmeticas::Resta,
            izquierdo: Rc::new(Expresion::Valor(3)),
            derecho: Rc::new(Expresion::Valor(4)),
        }),
        derecho: Rc::new(Expresion::Valor(5)),
    };

    let prueba2 = eval(&Expresion::Operacion {
        op: OperacionesAritmeticas::Suma,
        izquierdo: Rc::new(term1),
        derecho: Rc::new(term2),
    });
    println!("El resultado de (10*9) + ((3-4)*5) es {:?}", prueba2);

    // --- Demostración adicional: reutilización de subexpresiones ---
    // Con Rc podemos compartir un mismo nodo en varios lugares sin duplicar.
    let compartido = Rc::new(Expresion::Valor(100));
    let expr_reutilizada = Expresion::Operacion {
        op: OperacionesAritmeticas::Suma,
        izquierdo: compartido.clone(), // Clonar Rc solo incrementa el contador
        derecho: compartido,           // Movemos la última referencia
    };
    println!("Resultado de (100 + 100) = {}", eval(&expr_reutilizada));
}