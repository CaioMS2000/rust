// Este módulo define as estruturas de dados que representam eventos do GitHub
// Modelar dados com tipos fortes é uma das maiores vantagens de Rust

// CONCEITO: Structs
// Structs são tipos customizados que agrupam dados relacionados
// Similar a classes em outras linguagens, mas sem métodos (por padrão)
#[derive(Debug, Clone)]  // Deriva Debug (para imprimir) e Clone (para copiar)
pub struct GitHubEvent {
    // CONCEITO: pub
    // 'pub' torna o campo público, acessível de fora do módulo
    // Sem 'pub', campos seriam privados por padrão
    pub event_type: String,  // Tipo do evento (PushEvent, WatchEvent, etc.)
    pub repo_name: String,   // Nome completo do repositório (ex: "torvalds/linux")
    pub payload: EventPayload,  // Dados específicos do tipo de evento
}

// CONCEITO: Enums com Dados
// Diferente de enums em C/Java, enums em Rust podem carregar dados
// Cada variante pode ter dados diferentes ou nenhum dado
#[derive(Debug, Clone)]
pub enum EventPayload {
    // Variante com campo nomeado
    // Usada quando alguém faz push de commits
    Push {
        commit_count: usize,  // usize é um inteiro sem sinal do tamanho do ponteiro (32/64 bits)
    },

    // Evento de issue (aberta, fechada, etc.)
    IssuesEvent {
        action: String,  // "opened", "closed", "reopened"
    },

    // Evento de pull request
    PullRequestEvent {
        action: String,  // "opened", "closed", "merged"
    },

    // Variante sem dados
    // WatchEvent significa que alguém deu "star" no repositório
    WatchEvent,

    // Evento de fork (alguém copiou o repositório)
    ForkEvent,

    // Criação de branch ou tag
    CreateEvent {
        ref_type: String,  // "branch" ou "tag"
    },

    // Deleção de branch ou tag
    DeleteEvent {
        ref_type: String,  // "branch" ou "tag"
    },

    // Publicação de uma release
    ReleaseEvent {
        action: String,  // "published", "created", "edited"
    },

    // Comentários em issues
    IssueCommentEvent,

    // Comentários em pull requests (code review)
    PullRequestReviewCommentEvent,

    // Comentários em commits
    CommitCommentEvent,

    // Tipo desconhecido - usado quando encontramos um evento que não mapeamos
    // É uma boa prática ter um caso "catch-all" para dados externos
    Unknown,
}

// CONCEITO: Implementação de métodos
// O bloco 'impl' adiciona métodos (funções associadas) a um tipo
impl GitHubEvent {
    // Método construtor - convenção em Rust é usar 'new'
    // 'pub' torna o método público
    pub fn new(event_type: String, repo_name: String, payload: EventPayload) -> Self {
        // 'Self' é um alias para o tipo que estamos implementando (GitHubEvent)
        // Em Rust, a última expressão de uma função é automaticamente retornada
        // (não precisa de 'return' explícito)
        GitHubEvent {
            event_type,  // Sintaxe curta: event_type: event_type
            repo_name,   // O Rust permite omitir o valor se o nome do campo == nome da variável
            payload,
        }
    }
}

impl EventPayload {
    // Método auxiliar para verificar se o payload requer dados do JSON
    // Retorna true se precisarmos fazer parsing adicional do payload
    pub fn requires_payload_parsing(event_type: &str) -> bool {
        // CONCEITO: &str vs String
        // &str é uma "string slice" - uma referência imutável a uma string
        // É mais eficiente que String quando não precisamos possuir os dados
        matches!(
            event_type,
            "PushEvent"
                | "IssuesEvent"
                | "PullRequestEvent"
                | "CreateEvent"
                | "DeleteEvent"
                | "ReleaseEvent"
        )
    }
}
