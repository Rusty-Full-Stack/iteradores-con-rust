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

    let iterador_personas = personas.iter();
    
    let edad_minima_a_buscar = 30;

    // El resultado de un filter es otro iterador.
    let personas_encontradas = iterador_personas.filter(|&p| p.edad >= edad_minima_a_buscar);
    let mut cantidad_personas_encontradas = 0;

    for persona in personas_encontradas {
        println!("Persona Encontrada:");
        println!("Identificador: {}", persona.identificador);
        println!("Nombre: {}", persona.nombre);
        println!("Edad: {}", persona.edad);
        println!("=========================================");
        cantidad_personas_encontradas += 1;
    }

    println!("{} Personas Encontradas", cantidad_personas_encontradas);
}
