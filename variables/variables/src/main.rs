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

fn fibo (n:u32){
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
    
}