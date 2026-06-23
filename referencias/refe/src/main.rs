/*fn main() {
    let mut a = 'a';
    let b = 'b';
    //let mut r = a; //se pasa completamente la variable incluyendo el espacio de memoria
    let r:&char = &mut a; //se referencia el valor y ya funciona referencia mutable o exclusiva
    //println!("{}",r);
    *r = 't';
    dbg!(r); 
    dbg!(a); 
    //println!("espacio de memoria {:p}",&r);
    //println!("espacio de memoria a {:p}",&a);
    //r = &mut b;
    
    //dbg!(&r);
    //println!("espacio de memoria {:p}",&r);
}*/

/* Rebanada o slice */
/* 
fn main(){
    let a:[i32;6] = [1,2,3,4,5,6];
    
    let rebanada: &[i32] = &a[1..3];
    println!("rebanada: {rebanada:?}");
    //let rebanada: &[i32] = &a[1..a.len()]; // let rebanada: &[i32] = &a[1..]
    //let rebanada: &[i32] = &a[..3]//todo hasta el valor 3
} */
/*
fn main (){ //Las referencias no sobrevivan mas que las variables a las que apuntan
    let x_referencia = {
        let equis = 1;
        &equis// marca error porque la referencia no puede vivir mas que let equis, y let equis muere al acabar el bloque de codigo
    }
    dbg!(x_referencia);
}

//Geometria, se paractica referencias: Implementar dos funciones, la primera para calcular magnitud de un vector, y la segunda para normalizar el mismo vector

*/


fn calcular_magnitud(refvector: &[f64;3])-> f64{
    let mut sumatoria = 0.0;
    for elemento in refvector{
        sumatoria += elemento*elemento;
    }
    sumatoria.sqrt()


}

fn normalice_vector(renglonref: &mut [f64;3]){
    let magnitud = calcular_magnitud(renglonref);
    for elemento in renglonref{
        *elemento /=magnitud;   
    }
}

fn main(){
    let mut vector = [4.0,5.0,6.0];
    println!("Magnitud calculada {}",calcular_magnitud(&vector));
    normalice_vector(&mut vector);
    println!("Magnitud calculada normalizada {}",calcular_magnitud(&vector));
}

    