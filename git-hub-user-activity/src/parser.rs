// Este módulo implementa parsing manual de JSON
// É uma abordagem educativa para entender manipulação de strings em Rust
// Em produção, normalmente usaríamos 'serde_json', mas fazer manualmente
// ensina muito sobre borrowing, string slicing, e error handling

use crate::error::ActivityError;
use crate::models::{EventPayload, GitHubEvent};

// CONCEITO: Result<T, E>
// Result é um enum que representa sucesso (Ok) ou falha (Err)
// É como o sistema de tipos força você a lidar com erros explicitamente
pub fn parse_events(json_text: &str) -> Result<Vec<GitHubEvent>, ActivityError> {
    // CONCEITO: Vec<T>
    // Vec é um vetor dinâmico (como ArrayList em Java ou list em Python)
    // Cresce conforme necessário
    let mut events = Vec::new();

    // Encontra o início e fim do array JSON
    // trim() remove espaços em branco nas pontas
    let trimmed = json_text.trim();

    // Validação básica: deve começar com [ e terminar com ]
    if !trimmed.starts_with('[') || !trimmed.ends_with(']') {
        return Err(ActivityError::ParseError(
            "Expected JSON array".to_string(),
        ));
    }

    // Remove os colchetes [ e ]
    let content = &trimmed[1..trimmed.len() - 1].trim();

    // Se o array está vazio, retorna vetor vazio
    if content.is_empty() {
        return Ok(events);
    }

    // Divide o JSON em objetos individuais
    // Esta é a parte mais complexa do parsing manual
    let objects = split_json_objects(content)?;

    // CONCEITO: Iterators
    // for..in é sintaxe açucarada para iterators
    // Cada 'obj' é uma referência &str para um objeto JSON
    for obj in objects {
        // Tenta parsear cada objeto como um evento
        // O operador ? propaga erros automaticamente
        // Se parse_event retornar Err, toda a função retorna esse erro
        match parse_event(obj) {
            Ok(event) => events.push(event),  // push adiciona ao final do Vec
            Err(_) => continue,  // Ignora eventos que não conseguimos parsear
        }
    }

    Ok(events)  // Sucesso! Retorna o vetor de eventos
}

// Função auxiliar para dividir objetos JSON em um array
// Esta é uma versão simplificada que funciona para o caso específico da API do GitHub
fn split_json_objects(content: &str) -> Result<Vec<&str>, ActivityError> {
    let mut objects = Vec::new();
    let mut depth = 0;  // Rastreia nível de aninhamento de { }
    let mut start = 0;

    // CONCEITO: chars() e enumerate()
    // chars() retorna um iterator sobre os caracteres Unicode
    // enumerate() adiciona o índice (posição) a cada elemento
    for (i, ch) in content.chars().enumerate() {
        match ch {
            '{' => {
                if depth == 0 {
                    start = i;  // Marca início de um objeto
                }
                depth += 1;
            }
            '}' => {
                depth -= 1;
                if depth == 0 {
                    // Fim de um objeto no nível raiz
                    // CONCEITO: String slicing [start..end]
                    // Cria uma fatia (slice) da string original
                    // É uma referência, não cópia - muito eficiente!
                    let obj = &content[start..=i];
                    objects.push(obj.trim());
                }
            }
            _ => {}  // Ignora outros caracteres
        }
    }

    Ok(objects)
}

// Parseia um único objeto JSON representando um evento
fn parse_event(json_obj: &str) -> Result<GitHubEvent, ActivityError> {
    // Extrai campos obrigatórios
    // CONCEITO: Option<T>
    // Option representa um valor que pode existir (Some) ou não (None)
    // É como null em outras linguagens, mas type-safe
    let event_type = extract_string_value(json_obj, "type")
        .ok_or_else(|| ActivityError::ParseError("Missing 'type' field".to_string()))?;

    // repo.name está aninhado: {"repo": {"name": "..."}}
    // Primeiro extraímos o objeto "repo"
    let repo_obj = extract_nested_object(json_obj, "repo")
        .ok_or_else(|| ActivityError::ParseError("Missing 'repo' field".to_string()))?;

    // Depois extraímos "name" de dentro dele
    let repo_name = extract_string_value(repo_obj, "name")
        .ok_or_else(|| ActivityError::ParseError("Missing 'repo.name' field".to_string()))?;

    // Parseia o payload específico do tipo de evento
    let payload = parse_payload(json_obj, &event_type)?;

    Ok(GitHubEvent::new(event_type, repo_name, payload))
}

// Parseia o campo "payload" baseado no tipo de evento
fn parse_payload(json_obj: &str, event_type: &str) -> Result<EventPayload, ActivityError> {
    // CONCEITO: Pattern matching com strings
    // match em &str compara o conteúdo da string
    match event_type {
        "PushEvent" => {
            // Extrai o objeto payload
            let payload_obj = extract_nested_object(json_obj, "payload")
                .unwrap_or("");  // unwrap_or retorna valor padrão se None

            // NOTA: O endpoint /users/{username}/events não inclui a lista de commits
            // Em produção, usaríamos size se disponível no payload
            // Para fins educacionais, vamos extrair size ou usar 1 como padrão
            let commit_count = extract_number_value(payload_obj, "size")
                .unwrap_or(1);  // Padrão: assume 1 commit

            Ok(EventPayload::Push { commit_count })
        }
        "IssuesEvent" => {
            let payload_obj = extract_nested_object(json_obj, "payload").unwrap_or("");
            let action = extract_string_value(payload_obj, "action")
                .unwrap_or_else(|| "unknown".to_string());

            Ok(EventPayload::IssuesEvent { action })
        }
        "PullRequestEvent" => {
            let payload_obj = extract_nested_object(json_obj, "payload").unwrap_or("");
            let action = extract_string_value(payload_obj, "action")
                .unwrap_or_else(|| "unknown".to_string());

            Ok(EventPayload::PullRequestEvent { action })
        }
        "WatchEvent" => Ok(EventPayload::WatchEvent),
        "ForkEvent" => Ok(EventPayload::ForkEvent),
        "CreateEvent" => {
            let payload_obj = extract_nested_object(json_obj, "payload").unwrap_or("");
            let ref_type = extract_string_value(payload_obj, "ref_type")
                .unwrap_or_else(|| "unknown".to_string());

            Ok(EventPayload::CreateEvent { ref_type })
        }
        "DeleteEvent" => {
            let payload_obj = extract_nested_object(json_obj, "payload").unwrap_or("");
            let ref_type = extract_string_value(payload_obj, "ref_type")
                .unwrap_or_else(|| "unknown".to_string());

            Ok(EventPayload::DeleteEvent { ref_type })
        }
        "ReleaseEvent" => {
            let payload_obj = extract_nested_object(json_obj, "payload").unwrap_or("");
            let action = extract_string_value(payload_obj, "action")
                .unwrap_or_else(|| "published".to_string());

            Ok(EventPayload::ReleaseEvent { action })
        }
        "IssueCommentEvent" => Ok(EventPayload::IssueCommentEvent),
        "PullRequestReviewCommentEvent" => Ok(EventPayload::PullRequestReviewCommentEvent),
        "CommitCommentEvent" => Ok(EventPayload::CommitCommentEvent),
        _ => Ok(EventPayload::Unknown),  // Tipos não mapeados
    }
}

// FUNÇÕES AUXILIARES DE PARSING
// Estas funções fazem o trabalho pesado de extrair valores do JSON

// Extrai um valor numérico de um campo JSON
// Exemplo: "size": 3 -> Some(3)
fn extract_number_value(json: &str, key: &str) -> Option<usize> {
    let pattern = format!("\"{}\":", key);
    let start_pos = json.find(&pattern)?;

    let after_colon = &json[start_pos + pattern.len()..].trim_start();

    // Encontra onde o número termina (próximo caractere não-numérico)
    let mut end_pos = 0;
    for ch in after_colon.chars() {
        if ch.is_numeric() {
            end_pos += ch.len_utf8();
        } else {
            break;
        }
    }

    if end_pos == 0 {
        return None;
    }

    // Parse a string para número
    after_colon[..end_pos].parse().ok()
}

// Extrai um valor string de um campo JSON
// Exemplo: "name": "torvalds/linux" -> Some("torvalds/linux")
fn extract_string_value(json: &str, key: &str) -> Option<String> {
    // Monta o padrão de busca: "key":
    // CONCEITO: format! macro
    // Similar ao println!, mas retorna uma String ao invés de imprimir
    let pattern = format!("\"{}\":", key);

    // CONCEITO: find() retorna Option<usize>
    // Some(posição) se encontrar, None se não encontrar
    let start_pos = json.find(&pattern)?;

    // Pula para depois do ":"
    let after_colon = &json[start_pos + pattern.len()..].trim_start();

    // Verifica se o valor é uma string (começa com ")
    if !after_colon.starts_with('"') {
        return None;
    }

    // Encontra o fim da string (próximo " que não é escapado)
    let value_start = 1;  // Pula o primeiro "
    let mut end_pos = value_start;
    let chars: Vec<char> = after_colon.chars().collect();

    // CONCEITO: Loop while com condições
    while end_pos < chars.len() {
        if chars[end_pos] == '"' && chars[end_pos - 1] != '\\' {
            // Encontrou o " final não-escapado
            break;
        }
        end_pos += 1;
    }

    // Extrai a substring
    let value: String = chars[value_start..end_pos].iter().collect();
    Some(value)
}

// Extrai um objeto aninhado
// Exemplo: "repo": {...} -> Some("{...}")
// CONCEITO: Lifetimes
// 'a indica que a string retornada vive tanto quanto a string json de entrada
// Isso é necessário porque retornamos uma fatia (slice) de json
fn extract_nested_object<'a>(json: &'a str, key: &str) -> Option<&'a str> {
    let pattern = format!("\"{}\":", key);
    let start_pos = json.find(&pattern)?;

    let after_colon = &json[start_pos + pattern.len()..].trim_start();

    // Objeto deve começar com {
    if !after_colon.starts_with('{') {
        return None;
    }

    // Encontra o { correspondente rastreando profundidade
    let mut depth = 0;
    let mut end_pos = 0;

    for (i, ch) in after_colon.chars().enumerate() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    end_pos = i + 1;
                    break;
                }
            }
            _ => {}
        }
    }

    if end_pos == 0 {
        return None;
    }

    Some(&after_colon[..end_pos])
}

// Extrai o tamanho de um array JSON
// Exemplo: "commits": [{...}, {...}] -> Some(2)
fn extract_array_length(json: &str, key: &str) -> Option<usize> {
    let pattern = format!("\"{}\":", key);
    let start_pos = json.find(&pattern)?;

    let after_colon = &json[start_pos + pattern.len()..].trim_start();

    // Array deve começar com [
    if !after_colon.starts_with('[') {
        return None;
    }

    // Conta objetos separados por vírgula no nível raiz do array
    let mut depth = 0;
    let mut count = 0;
    let mut in_array = false;

    for ch in after_colon.chars() {
        match ch {
            '[' => {
                depth += 1;
                in_array = true;
            }
            ']' => {
                depth -= 1;
                if depth == 0 {
                    // Fim do array
                    // Se encontramos pelo menos um caractere não-whitespace, conta como 1 item
                    // Arrays vazios [] têm count = 0
                    break;
                }
            }
            '{' => {
                depth += 1;
                if depth == 2 {
                    // Profundidade 2 significa um objeto dentro do array
                    count += 1;
                }
            }
            '}' => depth -= 1,
            _ => {}
        }
    }

    if !in_array {
        return None;
    }

    Some(count)
}
