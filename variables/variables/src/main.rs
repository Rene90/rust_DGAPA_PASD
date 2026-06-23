/*fn main() {
    //let x=10;
    let x=10;
    println!("x: {x}");
    println!("Esta es la direccion de x {:p} antes de mutar",&x);
    //cambiamos el valor de la variable x por 15
    //1.-Volver a declarar la variable x
    //let x=15;
    //println!("Aqui cambio x pero volvi a declarar: {x}");
    //2. Escribir mutabilidad de la variable x
    let x = 30;
    println!("Mi variable x ha mutado: {x}");
    println!("Esta es la direccion de x {:p} cuando muto",&x);

}*/
// Inferencia de tipos
/* 
fn funcion_entero_32(x: u32){
    println!("Entro a la funcion_entero_32");
}
fn funcion_entero_i8(x: i8){
    println!("Entro a la funcion_entero_i8");
}


fn main(){
  let x =64;
  let y = 40;
  funcion_entero_i8(x);

}
  */
//La serie Fibonacci empieza con 0,1 y el numero siguiente es 1 ya que 0+1 es 1, el siguiente es =2, y asi sucesivamente. El numero es igual a la suma de los dos anteriores. Crear una funcion que me calcula el numero n de la serie, 

/*fn fibo (n:u32){
        if n<2{
            println!("Tienes que escribir un numero n mayor a dos");
        }
        else{
            let mut primer_valor=0;
            let mut segundo_valor=1; //Se inicia de esta manera porque los dos primeros valores siempre van a ser 0 y 1
            //println Imprimir el numero n de la serie
            let mut sum = 0;
            for _ in 2..=n{
                sum = primer_valor+segundo_valor;
                primer_valor = segundo_valor;
                segundo_valor= sum;
            }
            println!("Esta es la suma {sum}");
        }

    }

fn main(){
    let n = 8; //Elemento 30 de la serie que quiero calcular
    fibo(n);//21
    
}*/

/* 
fn main(){

    let afuera = 5;
    let x = {
        let adentro = 1;
        9+afuera //omitimos el ; porque es el valor que regresa el bloque de codigo {}
        //esto es equivalente a return 9;
    };
    println!("Valor de {x}");
    //println!("Valor de adentro {}");

}*/

//if statement manera explicita 

/*fn main()
{
    let x=10;
    if x ==1 {
        println!("igual a 1");
    } else if x==2{
        println!("igual a 2");
    } else{
        println!("igual a otro valor");
    }

    let tamanio = if x<15 {"peque"} else {"grande"};
    println("El tamnio del valor x es {tamanio}")
}*/

//Expresion match 
/*
fn main()
{
    let x=10;
    match x {
        1 => println!("Es uno"),
        2 => println!("Es dos"),
        _ => {
            println!("Otro valor");
        }

    }
}
     */
    //ciclos
//while
/*
fn main(){
    /*let mut x = 8;
    while x >= 3{
        println!{"Se esta ejecutando este bloque de codigo"};
        x = x-1;

    }
    println!("valor final de x {x}");
     //ciclo for

     for i in 1..5{
        println!("{i}");
     }
     for elemento in [2,3,4,5]{
        println!("{elemento}");

     }*/

    let mut contador =0;
    loop {
        contador +=1;
        if contador ==5{
            continue;
        }
        if contador < 10{
            println!("Se ejecuta el loop hasta el 10: {contador}");
            
        }
        
        if contador ==10{
            break;
        }

    }
}

//Secuencia COLLATZ
Para un numero n que sea mayor a 0 

If ni is 1, then the sequence terminates at ni.
If ni is even, then ni+1 = ni / 2.
If ni is odd, then ni+1 = 3 * ni + 1.
n = 3 
n es impar entonces el siguiente numero es 3 *3 +1 = 10
10 es par entonces 10/2 = 5
5 es impar es igual a 16
16 .... 8 
8 ... 4
4...2
2 ... 1
1 aqui acaba*/
/*
fn mida_longitud_secuencia (mut n:i32) -> u32{
    // aqui se implementa
    let mut contador = 0;
    while n >1{
        if n%2 ==0{
            n = n/2;
            contador +=1;
            
        }else{
            n = n*3+1;
            contador +=1;
        }
    }
    //return contador;
    contador
}

fn main (){
    println!("esta es la longitud {}", mida_longitud_secuencia(11));
}
     */