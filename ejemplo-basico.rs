fn main() {

    // Este es un vector de numeros enteros
    let a = vec![1, 2, 3, 4];

    // el metodo iter() se encarga de crear un iterador.
    let mut mi_iterador = a.iter();

    // el iterador toma posesion del vector y utiliza next() para conseguir el siguiente
    // valor, se utiliza Some porque cuando el iterador llega a su fin
    // puede retornar un None
    assert_eq!( mi_iterador.next() , Some(&1) );
    assert_eq!( mi_iterador.next() , Some(&2) );
    assert_eq!( mi_iterador.next() , Some(&3) );
    assert_eq!( mi_iterador.next() , Some(&4) );
    assert_eq!( mi_iterador.next() , None );

}
