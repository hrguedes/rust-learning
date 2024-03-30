use std::process::exit;

use utils::terminal::{exibir_menu, limpar_tela};

mod utils;
mod fundamentos;
mod tipos;
mod controle;
mod ownership;
mod funcoes;

fn main() {
    loop {
        
        let itens = ["Fundamentos", "Tipos", "Controle", "Ownership"];

        let selecionado = exibir_menu("Menu Principal", &itens, true);

        limpar_tela();

        match selecionado {
            1 => fundamentos::execultar(),
            2 => tipos::executar(),
            3 => controle::executar(),
            4 => funcoes::executar(),
            5 => ownership::executar(),
            _ => exit(0),
        }
    }
}
