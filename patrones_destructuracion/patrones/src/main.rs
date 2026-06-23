/*fn checar_orden(tu: (i32,i32,i32))-> bool{
    //destructuracion
    let (var1,var2,var3) = tu;
    var1 < var2 && var2 < var3
}
fn main() {
    let tupla= (1,2,1);


    println!("Esta es el return de la funcion checar orden, {}",checar_orden(tupla));
}*/

/* Arreglos anidados */



fn transpuesta(ma1: [[i32;3];3])->[[i32;3];3] {
    let mut matt = [[0,0,0],[0,0,0],[0,0,0]];
    //escribir el algoritmo para transponer una matriz
    for i in 0..ma1.len(){
        for j in 0..ma1[0].len(){
            matt[j][i] =ma1[i][j];
        }
    }
    return matt;
}

fn main(){
let ejemplo = [[1,2,3],[4,5,6],[7,8,9]];


let ejemplo_transpuesto = transpuesta(ejemplo);

//imprimir matriz
for renglon in ejemplo_transpuesto{
    println!("{:?}",renglon);

}
}