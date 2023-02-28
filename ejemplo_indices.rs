fn main() {

    let lista = vec!["Primer Elemento", "Segundo Element", "Tercer Elemento"];

    let mut mi_iter = lista.iter().enumerate();

    while let Some((indice, valor)) = mi_iter.next() {
        println!("{} - {}", indice, valor); 
    }

}
