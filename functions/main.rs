/* Funções em RUST

fn nome_funcao (nome_variavel: tipo_variavel) -> tipo_retorno {
    if condicao == condicao {
        return retorno_com_ponto_vergula;
    }    

    retorno_sem_ponto_virgula
}

*/

fn divide_by_5(num: u32) -> u32 {
    if num == 0 {
        return 0;
    }
    num / 5
}

// Posso colocar em qualquer ordem as funções 
fn main() {
    let num = 25;
    println!("{} divided by 5 = {}", num, divide_by_5(num));
    println!("{} divided by 5 = {}", num, divide_by_5(0));
    
    let formal = "Formal: Good bye.";
    let casual = "Casual: See you later!";
    goodbye(formal);
    goodbye(casual);
}

fn goodbye(message: &str) -> bool {
    println!("\n{}", message);
    return true
}
