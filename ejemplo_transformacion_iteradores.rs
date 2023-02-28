struct Venta {
    total_sin_descuento: f32,
    total_con_descuento: f32
}

fn main() {
    let totales = vec![25.35, 12.25, 10.15, 20.55];
    let estructura_totales_con_descuentos: Vec<Venta> = totales.iter().map(
        |elemento| Venta{
            total_sin_descuento: *elemento,
            total_con_descuento: *elemento - (*elemento * 0.1) // 10 porciento menos, por facilidad
                                                               // no hacemos redondeos
        }
    ).collect(); // Es importante utilizar el collect porque los iteradores son lazy, es decir el
                 // map no se ejecutara si no se es requerido explicitamente con el collect

    // En esta ocasion por facilidad no hacemos redondeos, si quieres redondear algo
    // el crate con mejor precision para hacerlo es round
    // https://crates.io/crates/round
    for venta in estructura_totales_con_descuentos {
        println!("total: ${} - total con descuento: ${}", venta.total_sin_descuento, venta.total_con_descuento);
    }
}
