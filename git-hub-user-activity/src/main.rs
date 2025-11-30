// Este é o ponto de entrada (entry point) da aplicação
// A função main() é onde o programa começa a executar

// CONCEITO: Declaração de Módulos
// 'mod' declara que estes módulos fazem parte do nosso crate (projeto)
// O Rust procura por arquivos com esses nomes em src/
mod api;      // Lê src/api.rs
mod display;  // Lê src/display.rs
mod error;    // Lê src/error.rs
mod models;   // Lê src/models.rs
mod parser;   // Lê src/parser.rs

// CONCEITO: use
// Importa itens específicos para uso neste arquivo
// Sem 'use', teríamos que escrever std::env::args() toda vez
use std::env;      // Para acessar argumentos da linha de comando
use std::process;  // Para controlar o processo (exit codes)

// CONCEITO: fn main()
// O ponto de entrada de todo programa Rust
// Não retorna valor explicitamente (tipo de retorno é () - "unit")
fn main() {
    // CONCEITO: Coleta de argumentos CLI
    // env::args() retorna um iterator sobre os argumentos
    // collect() transforma o iterator em um Vec<String>
    // args[0] é sempre o nome do executável
    // args[1..] são os argumentos passados pelo usuário
    let args: Vec<String> = env::args().collect();

    // Validação: esperamos exatamente 1 argumento (além do nome do programa)
    if args.len() != 2 {
        // CONCEITO: eprintln!
        // Similar ao println!, mas imprime em stderr (erro padrão)
        // É uma convenção imprimir mensagens de erro em stderr
        eprintln!("Usage: {} <username>", args[0]);
        eprintln!("\nExamples:");
        eprintln!("  {} torvalds", args[0]);
        eprintln!("  {} github", args[0]);

        // CONCEITO: process::exit()
        // Termina o programa com um código de saída
        // 0 = sucesso, não-zero = erro
        // Códigos de erro ajudam em scripts shell
        process::exit(1);
    }

    // CONCEITO: Indexação e Referências
    // &args[1] cria uma referência ao segundo elemento
    // Em Rust, indexar um Vec pode causar panic se o índice não existir
    // Mas já validamos que args.len() == 2, então é seguro
    let username = &args[1];

    // CONCEITO: Match com Result
    // run() retorna Result<(), error::ActivityError>
    // Devemos lidar com Ok e Err explicitamente
    match run(username) {
        // Se sucesso, não fazemos nada
        // Ok(()) significa "sucesso sem valor de retorno"
        Ok(()) => {}

        // Se erro, imprimimos a mensagem e saímos com código 1
        // 'e' tem tipo ActivityError, que implementa Display
        Err(e) => {
            eprintln!("\nError: {}", e);
            process::exit(1);
        }
    }
}

// CONCEITO: Separação de Lógica
// É boa prática separar a lógica principal (run) do entry point (main)
// main() lida com argumentos e exit codes
// run() contém a lógica de negócio e pode retornar Result
//
// &str é uma "string slice" - referência imutável a uma string
// Result<(), error::ActivityError> significa:
//   - Ok(()) em caso de sucesso (sem valor)
//   - Err(error::ActivityError) em caso de erro
fn run(username: &str) -> Result<(), error::ActivityError> {
    // Mensagem informativa
    println!("Fetching recent activity for '{}'...", username);

    // CONCEITO: Chamada de função entre módulos
    // api::fetch_user_events está em src/api.rs
    // O operador ? propaga erros:
    //   - Se Ok(events), desempacota e continua
    //   - Se Err(e), retorna Err(e) imediatamente
    let events = api::fetch_user_events(username)?;

    // CONCEITO: Vec::is_empty()
    // Verifica se o vetor tem zero elementos
    if events.is_empty() {
        display::display_no_events(username);
        // return explícito não é necessário, mas deixa o código mais claro
        return Ok(());
    }

    // Mostra cabeçalho com contagem de eventos
    display::display_header(username, events.len());

    // CONCEITO: Passagem por Referência
    // &events empresta (borrow) o vetor para display_events
    // A função pode ler mas não modificar ou tomar posse
    // Após a chamada, ainda podemos usar 'events' aqui
    display::display_events(&events);

    // Linha em branco para melhor formatação
    println!();

    // CONCEITO: Return implícito
    // A última expressão de uma função é retornada automaticamente
    // Ok(()) indica sucesso
    Ok(())
}

// CONCEITO: Conditional Compilation
// #[cfg(test)] significa "compile apenas em modo de teste"
// Execute com: cargo test
#[cfg(test)]
mod tests {
    use super::*;  // Importa tudo do módulo pai (main)

    // Aqui você poderia adicionar testes de integração
    // Por exemplo, testar a função run() com mocks

    #[test]
    fn test_example() {
        // Este é só um placeholder
        // Em um projeto real, você testaria a lógica aqui
        assert_eq!(2 + 2, 4);
    }
}
