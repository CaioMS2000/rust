// Este módulo define tipos de erro customizados para a aplicação
// Em Rust, é uma prática comum criar tipos de erro específicos para o domínio
// usando enums, ao invés de usar strings genéricas

use std::fmt;

// CONCEITO: Enums em Rust
// Um enum (tipo enumerado) pode ter diferentes "variantes" (variants)
// Cada variante pode conter dados diferentes, tornando-os muito poderosos
#[derive(Debug)]  // Deriva automaticamente a trait Debug para facilitar impressão durante desenvolvimento
pub enum ActivityError {
    // Variante que guarda uma String descrevendo um erro de rede
    // O tipo String é "owned" (possui os dados), diferente de &str que só empresta
    NetworkError(String),

    // Variante para usuário inválido
    InvalidUsername(String),

    // Variante com campos nomeados (similar a uma struct)
    // Usada quando a API retorna um erro HTTP
    ApiError {
        status: u16,      // Código HTTP (200, 404, 500, etc.)
        message: String,  // Mensagem de erro
    },

    // Variante para erros no parsing de JSON
    ParseError(String),

    // Variante sem dados associados
    // Usada quando não há eventos para mostrar
    NoEventsFound,
}

// CONCEITO: Traits
// Traits são como interfaces em outras linguagens
// A trait Display define como um tipo deve ser formatado como texto
impl fmt::Display for ActivityError {
    // Método obrigatório da trait Display
    // &self é uma referência imutável ao próprio objeto (borrowing)
    // f é onde escrevemos o output formatado
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // CONCEITO: Pattern Matching
        // match é extremamente poderoso em Rust - permite "desconstruir" enums
        // O compilador garante que todos os casos sejam tratados (exhaustive matching)
        match self {
            // Para cada variante, extraímos os dados usando pattern matching
            // msg é uma referência aos dados dentro da variante
            ActivityError::NetworkError(msg) => {
                write!(f, "Network error: {}", msg)
            }
            ActivityError::InvalidUsername(username) => {
                write!(f, "Invalid username: '{}'", username)
            }
            // Aqui desconstruímos os campos nomeados
            ActivityError::ApiError { status, message } => {
                write!(f, "GitHub API error (status {}): {}", status, message)
            }
            ActivityError::ParseError(msg) => {
                write!(f, "Failed to parse response: {}", msg)
            }
            ActivityError::NoEventsFound => {
                write!(f, "No recent events found")
            }
        }
    }
}

// CONCEITO: Trait std::error::Error
// Esta é a trait padrão para tipos de erro em Rust
// Implementá-la permite que nosso erro seja compatível com o ecossistema Rust
impl std::error::Error for ActivityError {}

// CONCEITO: Conversão automática de erros com From
// A trait From permite conversão automática entre tipos
// Isso é útil com o operador ? para propagar erros
impl From<std::io::Error> for ActivityError {
    fn from(err: std::io::Error) -> Self {
        // Convertemos um erro de I/O em nosso tipo customizado
        // Self refere-se ao tipo que estamos implementando (ActivityError)
        ActivityError::NetworkError(err.to_string())
    }
}

// Conversão de erros do ureq (nossa biblioteca HTTP)
impl From<Box<ureq::Error>> for ActivityError {
    fn from(err: Box<ureq::Error>) -> Self {
        // Analisamos o tipo de erro HTTP
        match *err {
            // Erro de status HTTP (404, 500, etc.)
            ureq::Error::Status(code, response) => {
                // Tentamos ler o corpo da resposta para obter a mensagem de erro
                let message = response
                    .into_string()
                    .unwrap_or_else(|_| String::from("Unknown error"));

                ActivityError::ApiError {
                    status: code,
                    message,
                }
            }
            // Erro de transporte (sem conexão, timeout, etc.)
            ureq::Error::Transport(transport) => {
                ActivityError::NetworkError(transport.to_string())
            }
        }
    }
}
