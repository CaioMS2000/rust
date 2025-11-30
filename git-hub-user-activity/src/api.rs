// Este módulo gerencia a comunicação com a API do GitHub
// Demonstra como fazer requisições HTTP e integrar módulos em Rust

// CONCEITO: use e crate::
// 'use' importa itens de outros módulos
// 'crate::' é o caminho absoluto a partir da raiz do nosso projeto
use crate::error::ActivityError;
use crate::models::GitHubEvent;
use crate::parser;

// CONCEITO: const
// Constantes são imutáveis e conhecidas em tempo de compilação
// Por convenção, usam SCREAMING_SNAKE_CASE
const GITHUB_API_BASE: &str = "https://api.github.com";

// GitHub requer um User-Agent header em todas as requisições
// Isso identifica nossa aplicação
const USER_AGENT: &str = "github-activity-cli/1.0";

// Função principal que busca eventos de um usuário
// CONCEITO: Assinatura de função com Result
// -> Result<Vec<GitHubEvent>, ActivityError> significa:
// "Esta função pode retornar Ok(Vec de eventos) ou Err(erro)"
pub fn fetch_user_events(username: &str) -> Result<Vec<GitHubEvent>, ActivityError> {
    // Valida o username antes de fazer a requisição
    // O operador ? propaga o erro se a validação falhar
    validate_username(username)?;

    // CONCEITO: format! macro
    // Cria uma String interpolando valores
    // {} é substituído pelos argumentos
    let url = format!("{}/users/{}/events", GITHUB_API_BASE, username);

    // Faz a requisição HTTP
    let response_text = make_http_request(&url)?;

    // Parseia o JSON usando nosso parser manual
    // parser::parse_events refere-se à função parse_events do módulo parser
    let events = parser::parse_events(&response_text)?;

    // Retorna os eventos parseados
    Ok(events)
}

// Valida se o username é válido
// Em Rust, Result<(), E> significa "sucesso sem valor" ou erro
fn validate_username(username: &str) -> Result<(), ActivityError> {
    // CONCEITO: is_empty() em strings
    // Verifica se a string tem comprimento 0
    if username.is_empty() {
        return Err(ActivityError::InvalidUsername(
            "Username cannot be empty".to_string(),
        ));
    }

    // Validações básicas de username do GitHub
    // Username não pode conter espaços ou caracteres especiais
    if username.contains(' ') {
        return Err(ActivityError::InvalidUsername(
            "Username cannot contain spaces".to_string(),
        ));
    }

    // GitHub usernames têm no máximo 39 caracteres
    if username.len() > 39 {
        return Err(ActivityError::InvalidUsername(
            "Username is too long (max 39 characters)".to_string(),
        ));
    }

    // Tudo certo!
    Ok(())
}

// Faz uma requisição HTTP GET e retorna o corpo da resposta como String
fn make_http_request(url: &str) -> Result<String, ActivityError> {
    // CONCEITO: ureq - Cliente HTTP simples
    // ureq::get() cria uma requisição GET
    // .set() adiciona headers
    // .call() executa a requisição
    //
    // O tipo de retorno de .call() é Result<Response, Error>
    // Usamos ? para propagar erros automaticamente
    let response = ureq::get(url)
        .set("User-Agent", USER_AGENT)  // Header obrigatório para GitHub
        .call()
        // CONCEITO: map_err para converter erros
        // ureq retorna ureq::Error, mas nossa função espera ActivityError
        // map_err transforma um tipo de erro em outro
        .map_err(|e| {
            // Box::new é necessário porque ureg::Error não implementa From
            ActivityError::from(Box::new(e))
        })?;

    // CONCEITO: into_string()
    // Converte o corpo da resposta HTTP em String
    // Pode falhar se o corpo não for UTF-8 válido
    let body = response
        .into_string()
        .map_err(|e| ActivityError::ParseError(format!("Failed to read response: {}", e)))?;

    Ok(body)
}

// TESTES (opcional, mas boa prática)
// #[cfg(test)] significa "compile isso apenas em modo de teste"
#[cfg(test)]
mod tests {
    use super::*;

    // #[test] marca uma função como teste
    // Execute com: cargo test
    #[test]
    fn test_validate_username_valid() {
        assert!(validate_username("torvalds").is_ok());
        assert!(validate_username("github").is_ok());
        assert!(validate_username("user-name").is_ok());
        assert!(validate_username("user_name").is_ok());
    }

    #[test]
    fn test_validate_username_invalid() {
        assert!(validate_username("").is_err());
        assert!(validate_username("user name").is_err());
        assert!(validate_username(&"a".repeat(40)).is_err());
    }
}
