// -> #[derive(Debug)] -> sinaliza que a 'linha' a baixo pode ser depurada

// Define a tupla
#[derive(Debug)]
struct KeyPress(String, char);

// Define a struct
#[derive(Debug)]
struct MouseClick { x: i64, y: i64 }

// Define o enum como a estrutura e tupla dentro
#[derive(Debug)]
enum WebEvent { WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress) }

fn main() {
    // Instanciando a estrutura que vai no enum
    let click = MouseClick { x: 100, y: 250 };
    println!("Mouse click location: {}, {}", click.x, click.y);
        
    // Instanciando a tupla que vai no enum 
    let keys = KeyPress(String::from("Ctrl+"), 'N');
    println!("\nKeys pressed: {}{}", keys.0, keys.1);
        
    // Instanciando cada um dos dados dentro da enum
    let we_load = WebEvent::WELoad(true);
    let we_click = WebEvent::WEClick(click);
    let we_key = WebEvent::WEKeys(keys);
    
    // Mostra os dados do enum de uma so vez com a macro {:#?} 
    // que só pode ser usada por causa da outra macro em cima
    println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_click, we_key);
}
