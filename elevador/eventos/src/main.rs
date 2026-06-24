
enum Evento{
    BotonPresionado(Boton), //tupla variante
    CerradoPuertas, //variante
    AbiertoPuertas, //variante
}
enum Direccion{
    Arriba,
    Abajo
}
#[derive(Debug)]
enum Boton {
    IrAlLobby(Direccion,Piso)
    IrAUnPiso(Piso)
}

fn ya_llego(piso: i32)->Evento{
    Evento::Llegada()

}

fn abrir_puertas()->Evento{
    Event::AbiertoPuertas


}

fn cerrar_puertas()-Evento{
    Event::CerradoPuertas

}

fn boton_presionado_lobby(piso: i32, direc:Direccion)->Evento{
    Evento::BotonPresionado(Boton::IrAlLobby(direc,piso))
}

fn boton_presionado_elevador(piso:i32)->Evento{
    Evento::BotonPresionado(Boton::IrAUnPiso(piso))
}

fn main() {
    println!("Se ha presionado un boton {:?}",boton_presionado_lobby(0,Direccion::Arriba));
    println!("Las puertas se han abierto {:?}",abrir_puertas());
    println!("Cerrando puertas {:?}",cerrar_puertas());
}
