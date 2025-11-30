// Este módulo é responsável por formatar e exibir eventos do GitHub
// Demonstra pattern matching avançado e formatação de strings

use crate::models::{EventPayload, GitHubEvent};

// Formata um único evento em uma string legível
// CONCEITO: Referências
// &GitHubEvent significa que pegamos emprestado (borrow) o evento
// Não tomamos posse (ownership), apenas lemos
pub fn format_event(event: &GitHubEvent) -> String {
    // CONCEITO: Pattern Matching Exaustivo
    // match em Rust DEVE cobrir todos os casos possíveis
    // O compilador garante que não esquecemos nenhuma variante
    // & em &event.payload porque estamos fazendo match em uma referência
    match &event.payload {
        // Para variantes com dados, usamos destructuring
        // commit_count é extraído do PayloadEvent::Push
        EventPayload::Push { commit_count } => {
            // CONCEITO: Dereferencing com *
            // commit_count é &usize (referência), *commit_count é usize (valor)
            // Precisamos do valor para comparar com 1
            let plural = if *commit_count == 1 { "" } else { "s" };

            // CONCEITO: format! macro
            // Similar ao printf em C ou str.format() em Python
            format!(
                "Pushed {} commit{} to {}",
                commit_count, plural, event.repo_name
            )
        }

        EventPayload::IssuesEvent { action } => {
            // Capitaliza a primeira letra da action
            let formatted_action = capitalize_first(action);
            format!("{} an issue in {}", formatted_action, event.repo_name)
        }

        EventPayload::PullRequestEvent { action } => {
            let formatted_action = capitalize_first(action);
            format!(
                "{} a pull request in {}",
                formatted_action, event.repo_name
            )
        }

        // Variantes sem dados são simples
        EventPayload::WatchEvent => {
            format!("Starred {}", event.repo_name)
        }

        EventPayload::ForkEvent => {
            format!("Forked {}", event.repo_name)
        }

        EventPayload::CreateEvent { ref_type } => {
            // Adiciona artigo apropriado (a/an)
            let article = if ref_type == "branch" { "a" } else { "a" };
            format!(
                "Created {} {} in {}",
                article, ref_type, event.repo_name
            )
        }

        EventPayload::DeleteEvent { ref_type } => {
            let article = if ref_type == "branch" { "a" } else { "a" };
            format!(
                "Deleted {} {} in {}",
                article, ref_type, event.repo_name
            )
        }

        EventPayload::ReleaseEvent { action } => {
            let formatted_action = capitalize_first(action);
            format!("{} a release in {}", formatted_action, event.repo_name)
        }

        EventPayload::IssueCommentEvent => {
            format!("Commented on an issue in {}", event.repo_name)
        }

        EventPayload::PullRequestReviewCommentEvent => {
            format!("Commented on a pull request in {}", event.repo_name)
        }

        EventPayload::CommitCommentEvent => {
            format!("Commented on a commit in {}", event.repo_name)
        }

        EventPayload::Unknown => {
            // Para eventos desconhecidos, mostra o tipo original
            format!("Performed {} in {}", event.event_type, event.repo_name)
        }
    }
}

// Exibe uma lista de eventos no terminal
// CONCEITO: Slices
// &[GitHubEvent] é uma slice - uma referência a uma sequência de eventos
// Pode ser um array, parte de um Vec, etc.
pub fn display_events(events: &[GitHubEvent]) {
    // CONCEITO: for..in loop
    // Itera sobre cada elemento da slice
    // 'event' é automaticamente uma referência (&GitHubEvent)
    for event in events {
        // println! imprime com nova linha no final
        // - é o marcador de lista
        println!("- {}", format_event(event));
    }
}

// Função auxiliar para capitalizar a primeira letra de uma string
fn capitalize_first(s: &str) -> String {
    // CONCEITO: Iterator chains
    // Rust permite encadear operações em iterators de forma elegante

    // chars() retorna iterator sobre caracteres
    let mut chars = s.chars();

    // next() pega o primeiro caractere (Option<char>)
    match chars.next() {
        None => String::new(),  // String vazia
        Some(first) => {
            // CONCEITO: to_uppercase() pode retornar múltiplos chars
            // (ex: em alemão, ß -> SS)
            // collect::<String>() junta os caracteres

            // CONCEITO: chain()
            // Une dois iterators
            // first.to_uppercase() + chars (resto da string)
            first
                .to_uppercase()
                .chain(chars)  // Adiciona o resto dos caracteres
                .collect()     // Coleta tudo em uma String
        }
    }
}

// Exibe mensagem quando não há eventos
pub fn display_no_events(username: &str) {
    println!("No recent activity found for user '{}'", username);
    println!("This could mean:");
    println!("  - The user has no public activity in the last 90 days");
    println!("  - The user doesn't exist");
    println!("  - The user has made their activity private");
}

// Exibe cabeçalho antes da lista de eventos
pub fn display_header(username: &str, event_count: usize) {
    println!("\nRecent activity for '{}':", username);
    println!("Found {} event{}\n", event_count, if event_count == 1 { "" } else { "s" });
}

// TESTES
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::EventPayload;

    #[test]
    fn test_capitalize_first() {
        assert_eq!(capitalize_first("hello"), "Hello");
        assert_eq!(capitalize_first("world"), "World");
        assert_eq!(capitalize_first(""), "");
        assert_eq!(capitalize_first("a"), "A");
    }

    #[test]
    fn test_format_push_event_single() {
        let event = GitHubEvent::new(
            "PushEvent".to_string(),
            "user/repo".to_string(),
            EventPayload::Push { commit_count: 1 },
        );
        assert_eq!(format_event(&event), "Pushed 1 commit to user/repo");
    }

    #[test]
    fn test_format_push_event_multiple() {
        let event = GitHubEvent::new(
            "PushEvent".to_string(),
            "user/repo".to_string(),
            EventPayload::Push { commit_count: 3 },
        );
        assert_eq!(format_event(&event), "Pushed 3 commits to user/repo");
    }

    #[test]
    fn test_format_watch_event() {
        let event = GitHubEvent::new(
            "WatchEvent".to_string(),
            "torvalds/linux".to_string(),
            EventPayload::WatchEvent,
        );
        assert_eq!(format_event(&event), "Starred torvalds/linux");
    }
}
