fn main() {
    let mut arreglo1: [i8;3] = [5,2,8];
    println!("{arreglo1:?}");
    arreglo1[1] = 7;
    println!("{arreglo1:?}");

    //tuplas estan definidas en el tamaño, no se pueden añadir ni quitar
    let tupla:(i8, bool)= (2,false);
    println!("{0}", tupla.0);
    for elementos in arreglo1{
        println!("{elementos}");
    }
    for i in 0 .. arreglo1.len(){
        println!("{arreglo1[i]:?}");
    }
}
