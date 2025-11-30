# GitHub User Activity CLI

Uma ferramenta de linha de comando escrita em Rust para visualizar a atividade recente de usuários do GitHub.

Este projeto foi criado como ferramenta de aprendizado para demonstrar conceitos fundamentais de Rust em um contexto prático.

## 📋 Sobre o Projeto

Este CLI consome a [API pública do GitHub](https://docs.github.com/en/rest/activity/events) para buscar e exibir os eventos recentes de qualquer usuário.

**Desafio original**: [roadmap.sh - GitHub User Activity](https://roadmap.sh/projects/github-user-activity)

## 🚀 Como Usar

### Pré-requisitos

- Rust instalado (versão 1.70 ou superior recomendada)
  - Se não tiver, instale via [rustup](https://rustup.rs/)

### Instalação e Execução

```bash
# Clone ou navegue até o diretório do projeto
cd git-hub-user-activity

# Compile o projeto (modo debug)
cargo build

# Execute a aplicação
cargo run -- <username>

# Ou compile em modo release (otimizado) e execute
cargo build --release
./target/release/git-hub-user-activity <username>
```

### Exemplos de Uso

```bash
# Ver atividade de Linus Torvalds
cargo run -- torvalds

# Ver atividade da organização GitHub
cargo run -- github

# Ver sua própria atividade
cargo run -- seu-username
```

### Saída Esperada

```
Fetching recent activity for 'torvalds'...

Recent activity for 'torvalds':
Found 30 events

- Pushed 2 commits to torvalds/linux
- Commented on a pull request in torvalds/linux
- Opened an issue in torvalds/subsurface
- Starred torvalds/linux
- Forked example/repository
- Created a branch in torvalds/test-project
```

## 🎓 Conceitos Rust Demonstrados

Este projeto é uma excelente introdução a Rust, cobrindo os seguintes conceitos:

### 1. **Ownership e Borrowing**
- **Onde ver**: [src/display.rs:12-20](src/display.rs#L12-L20), [src/main.rs:105](src/main.rs#L105)
- Função `format_event` recebe `&GitHubEvent` (empréstimo imutável)
- Função `display_events` recebe `&[GitHubEvent]` (slice emprestada)
- **O que aprender**: Diferença entre tomar posse (`GitHubEvent`) vs emprestar (`&GitHubEvent`)

### 2. **Error Handling**
- **Onde ver**: [src/error.rs](src/error.rs), [src/api.rs:18-24](src/api.rs#L18-L24)
- Tipo customizado `ActivityError` com variantes específicas
- Uso do operador `?` para propagação de erros
- Conversão automática entre tipos de erro com trait `From`
- **O que aprender**: `Result<T, E>`, `Option<T>`, pattern matching em erros

### 3. **Pattern Matching**
- **Onde ver**: [src/display.rs:15-87](src/display.rs#L15-L87), [src/parser.rs:89-150](src/parser.rs#L89-L150)
- Match exaustivo em `EventPayload` (o compilador garante todos os casos)
- Destructuring de enums com dados (`Push { commit_count }`)
- **O que aprender**: Como `match` força você a lidar com todos os casos possíveis

### 4. **Sistema de Módulos**
- **Onde ver**: [src/main.rs:7-11](src/main.rs#L7-L11)
- Declaração de módulos com `mod`
- Uso de `crate::` para caminhos absolutos
- Organização multi-arquivo
- **O que aprender**: Como estruturar projetos Rust maiores

### 5. **Traits**
- **Onde ver**: [src/error.rs:35-61](src/error.rs#L35-L61)
- Implementação de `Display` para formatação customizada
- Implementação de `Error` para compatibilidade com ecossistema
- Trait `From` para conversão entre tipos
- **O que aprender**: Como traits funcionam (similar a interfaces)

### 6. **Enums com Dados**
- **Onde ver**: [src/models.rs:20-60](src/models.rs#L20-L60), [src/error.rs:11-28](src/error.rs#L11-L28)
- `EventPayload` tem variantes com diferentes estruturas de dados
- `ActivityError` mostra enums com dados nomeados e simples
- **O que aprender**: Enums em Rust são muito mais poderosos que em C/Java

### 7. **Structs**
- **Onde ver**: [src/models.rs:8-14](src/models.rs#L8-L14)
- Definição de `GitHubEvent`
- Métodos associados com `impl`
- **O que aprender**: Como modelar dados em Rust

### 8. **String Handling**
- **Onde ver**: [src/parser.rs](src/parser.rs)
- Diferença entre `String` (owned) e `&str` (borrowed/slice)
- Manipulação manual: `find`, `split`, `trim`, slicing `[start..end]`
- **O que aprender**: Sistema de strings do Rust (UTF-8 garantido)

### 9. **Collections e Iterators**
- **Onde ver**: [src/parser.rs:28-33](src/parser.rs#L28-L33), [src/display.rs:93-102](src/display.rs#L93-L102)
- `Vec<T>` para arrays dinâmicos
- Iterator methods: `chars()`, `enumerate()`, `collect()`
- `for..in` loops
- **O que aprender**: Programação funcional com iterators

### 10. **Parsing Manual de JSON**
- **Onde ver**: [src/parser.rs](src/parser.rs) (todo o arquivo)
- Extração de valores com manipulação de strings
- Rastreamento de profundidade de objetos `{ }`
- **O que aprender**: Como trabalhar "na unha" antes de usar bibliotecas

### 11. **HTTP Client**
- **Onde ver**: [src/api.rs:48-69](src/api.rs#L48-L69)
- Uso de crate externa (`ureq`)
- Headers customizados (User-Agent)
- Conversão de erros
- **O que aprender**: Como integrar bibliotecas externas

### 12. **Testing**
- **Onde ver**: [src/api.rs:71-87](src/api.rs#L71-L87), [src/display.rs:107-140](src/display.rs#L107-L140)
- Módulos de teste com `#[cfg(test)]`
- Testes unitários com `#[test]`
- **O que aprender**: Como testar código Rust (`cargo test`)

## 📂 Estrutura do Projeto

```
git-hub-user-activity/
├── Cargo.toml          # Manifesto do projeto (dependências, metadata)
├── src/
│   ├── main.rs         # Entry point, argumentos CLI, orquestração
│   ├── error.rs        # Tipos de erro customizados
│   ├── models.rs       # Estruturas de dados (GitHubEvent, EventPayload)
│   ├── parser.rs       # Parsing manual de JSON (mais complexo!)
│   ├── api.rs          # Cliente HTTP, integração com GitHub API
│   └── display.rs      # Formatação e exibição de eventos
└── README.md           # Este arquivo
```

### Fluxo de Execução

1. **main.rs** → Parseia argumentos CLI
2. **api.rs** → Valida username e faz requisição HTTP
3. **parser.rs** → Parseia JSON manualmente em structs
4. **display.rs** → Formata eventos em texto legível
5. **main.rs** → Exibe resultado ou erro

## 🔧 Dependências

- **ureq** (2.9): Cliente HTTP síncrono e leve
  - Por quê? A stdlib do Rust não inclui cliente HTTP
  - Alternativa seria implementar TCP + TLS manualmente (muito complexo)

**Nenhuma** biblioteca de parsing JSON é usada - tudo é manual! 🎉

## 🧪 Testes

Execute os testes unitários:

```bash
cargo test
```

Execute testes específicos:

```bash
# Testa apenas o módulo de validação
cargo test validate

# Testa com output verbose
cargo test -- --nocapture
```

## 🐛 Tratamento de Erros

A aplicação lida com diversos cenários de erro:

- ✅ Username inválido (vazio, com espaços, muito longo)
- ✅ Usuário não encontrado (404)
- ✅ Erro de rede (sem conexão)
- ✅ Rate limit da API do GitHub
- ✅ JSON malformado
- ✅ Argumentos CLI inválidos

Todos os erros são exibidos de forma clara e informativa.

## 📚 Recursos para Aprendizado

### Documentação Oficial
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/) - O melhor recurso para iniciantes
  - Capítulo 4: Ownership e Borrowing
  - Capítulo 6: Enums e Pattern Matching
  - Capítulo 7: Sistema de Módulos
  - Capítulo 9: Error Handling
  - Capítulo 10: Traits

- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Aprenda com exemplos práticos

- [Rustlings](https://github.com/rust-lang/rustlings) - Exercícios interativos

### Conceitos Específicos

**Ownership & Borrowing:**
- [Visualizing Rust Memory Management](https://github.com/usagi/rust-memory-container-cs)
- [Common Ownership Patterns](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

**Error Handling:**
- [Error Handling in Rust](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [anyhow and thiserror crates](https://nick.groenen.me/posts/rust-error-handling/) (para projetos maiores)

**Iterators:**
- [Iterator Trait Documentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [Iterator Patterns](https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html)

### Comunidade Rust Brasil
- [Rust Brasil no Telegram](https://t.me/rustlangbr)
- [Rust Brasil no Discord](https://discord.gg/rust-br-community)
- [Fórum Oficial Rust](https://users.rust-lang.org/)

## 🚀 Próximos Passos

Após dominar este projeto, tente:

1. **Adicionar mais features:**
   - Filtrar eventos por tipo: `--type push`
   - Limitar número de eventos: `--limit 10`
   - Suporte a GitHub Personal Access Token (maior rate limit)
   - Cache de resultados em arquivo JSON

2. **Melhorar o código:**
   - Adicionar cores com crate `colored`
   - Implementar paginação de resultados
   - Usar `serde_json` para comparar com parsing manual
   - Adicionar mais testes unitários

3. **Explorar conceitos avançados:**
   - Programação assíncrona com `tokio`
   - Paralelismo com threads
   - Criar uma biblioteca (lib.rs) separada do binário
   - Publicar no crates.io

## 📝 Notas de Implementação

### Por que parsing manual de JSON?

Este projeto implementa parsing de JSON **manualmente** ao invés de usar `serde_json` para fins educacionais:

- ✅ Ensina manipulação avançada de strings
- ✅ Demonstra borrowing e lifetimes na prática
- ✅ Mostra como bibliotecas funcionam "por baixo dos panos"
- ✅ Pratica error handling complexo

Em produção, **sempre use serde_json**! É mais rápido, seguro e robusto.

### Limitações

- Rate limit: 60 requisições/hora sem autenticação
- Mostra no máximo 30 eventos (limitação da API do GitHub)
- Eventos públicos apenas

## 📄 Licença

Este projeto é livre para uso educacional.

## 🙏 Agradecimentos

- [roadmap.sh](https://roadmap.sh) pelo desafio original
- Comunidade Rust pela documentação excelente
- Você, por escolher aprender Rust! 🦀

---

**Dica**: Leia o código na ordem sugerida para melhor compreensão:
1. [src/models.rs](src/models.rs) - Entenda as estruturas de dados
2. [src/error.rs](src/error.rs) - Veja o sistema de erros
3. [src/display.rs](src/display.rs) - Formatação é simples
4. [src/parser.rs](src/parser.rs) - A parte mais complexa
5. [src/api.rs](src/api.rs) - Integração HTTP
6. [src/main.rs](src/main.rs) - Junta tudo

Bom aprendizado! 🦀
