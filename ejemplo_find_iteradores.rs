struct Persona {
    edad: u8,
    nombre: String,
    identificador: String
}

fn main() {
    let personas: Vec<Persona> = vec![
        Persona {
            edad: 25,
            nombre: "Juan".to_string(),
            identificador: "001".to_string()
        },
        Persona {
            edad: 18,
            nombre: "Maria".to_string(),
            identificador: "002".to_string()
        },
        Persona {
            edad: 35,
            nombre: "Ana".to_string(),
            identificador: "003".to_string()
        },
        Persona {
            edad: 30,
            nombre: "Francisco".to_string(),
            identificador: "004".to_string()
        }
    ];

    let mut iterador_personas = personas.iter();

    let identificador_a_buscar = "001".to_string();

    if let Some(persona) = iterador_personas.find(|&p| p.identificador == identificador_a_buscar) {
        println!("Persona Encontrada:");
        println!("Identificador: {}", persona.identificador);
        println!("Nombre: {}", persona.nombre);
        println!("Edad: {}", persona.edad);
    } else {
        println!("Persona no encontrada");
    }
}
