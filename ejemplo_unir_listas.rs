fn main() {

    let lista1 = vec!["primer elemento lista 1", "segundo elemento lista 1", "tercer elemento lista 1"];
    let lista2 = vec!["primer elemento lista 2", "segundo elemento lista 2", "tercer elemento lista 2"];

    let iterador_lista1 = lista1.iter();
    let iterador_lista2 = lista2.iter();

    let mut lista_unificada = iterador_lista1.zip(iterador_lista2);

    while let Some((elemento_lista1, elemento_lista2)) = lista_unificada.next() {
        println!("[{}, {}]", elemento_lista1, elemento_lista2);
    }

}
