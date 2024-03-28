use std::process::exit;

use utils::terminal::{exibir_menu, limpar_tela};

mod utils;
mod fundamentos;
mod tipos;

fn main() {
    loop {
        
        let itens = ["Fundamentos", "Tipos", "Controle", "Ownership"];

        let selecionado = exibir_menu("Menu Principal", &itens, true);

        limpar_tela();

        match selecionado {
            1 => fundamentos::execultar(),
            2 => tipos::executar(),
            3 => println!("3"),
            4 => println!("4"),
            5 => println!("5"),
            _ => exit(0),
        }
    }
}
