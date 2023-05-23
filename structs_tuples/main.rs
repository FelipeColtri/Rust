// Versão modificada do exemplo no site com mais alguns testes 

// Versão classica de Struct, com nomes nos campos
struct S {
    name: String,   // nome completo
    type_s: char, // A - B
    period: u8,     // 1 - 10
    remote: bool    // T - F
}

// Versão em tupla com somente tipos de dados estáticos
struct T (String, char, u16, f32);

fn main() {
    // Instanciando as structs e tupla
    let s1 = S {
        name: String::from("Fulano de Tal"),
        remote: true,
        period: 2,
        type_s: 'A'
    };

    let t1 = T(String::from("abcd"), 'B', 999, 3.75);

    let s2 = S {
        name: String::from("Beltrano de Vraw"),
        type_s: 'B',
        remote: false,
        period: 8
    };

    let t2 = T(String::from("dcba"), 'A', 48783, 78.0302);

    // Mostrando as informações
    println!("{}, Period {}. Remote: {}. Types: {} of {}, {}, {}, {}", 
             s1.name, s1.period, s1.remote, s1.type_s, t1.0, t1.1, t1.2, t1.3);

    println!("{}, Period {}. Remote: {}. Types: {} of {}, {}, {}, {}", 
             s2.name, s2.period, s2.remote, s2.type_s, t2.0, t2.1, t2.2, t2.3);
    
}
