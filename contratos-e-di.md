# Contratos e Injeção de Dependência em Rust

## O Problema Central

Em linguagens como TypeScript, toda variável é implicitamente um ponteiro para a heap. Isso permite escrever:

```typescript
let operation: MathOperation = new Addition();
operation = new Subtraction(); // troca em runtime, sem problemas
```

O runtime sabe o tamanho e gerencia a memória automaticamente.

Em Rust, o compilador precisa saber **em tempo de compilação** o tamanho exato de cada variável na stack. Como cada implementação de um trait pode ter tamanho diferente, não é possível fazer:

```rust
// ERRO: dyn MathOperation não tem tamanho conhecido em compilação
let operation: dyn MathOperation = Addition;
```

A solução é sempre a mesma: colocar o valor atrás de um **ponteiro**, que tem tamanho fixo e conhecido.

---

## Contratos em Rust: Traits

O equivalente de `abstract class` / `interface` em Rust é o **trait**:

```rust
trait MathOperation {
    fn calculate(&self, a: f64, b: f64) -> f64;
}

struct Addition;
impl MathOperation for Addition {
    fn calculate(&self, a: f64, b: f64) -> f64 { a + b }
}

struct Subtraction;
impl MathOperation for Subtraction {
    fn calculate(&self, a: f64, b: f64) -> f64 { a - b }
}
```

---

## As 3 Abordagens para Usar Traits como Tipo

### 1. `dyn Trait` com ponteiros — Polimorfismo Dinâmico (runtime)

`dyn Trait` significa "algum tipo que implementa esse trait, mas eu não sei qual em compilação". Como o tamanho é desconhecido, é obrigatório usar um ponteiro. Qualquer tipo de ponteiro serve:

#### `Box<dyn Trait>` — Ownership na heap

O mais comum. O valor é alocado na heap e a variável tem ownership dele.

```rust
let mut operation: Box<dyn MathOperation> = Box::new(Addition);
println!("{}", operation.calculate(10.0, 5.0)); // 15

operation = Box::new(Subtraction); // troca em runtime
println!("{}", operation.calculate(10.0, 5.0)); // 5
```

Equivalente mais direto do padrão TypeScript.

#### `&dyn Trait` — Referência emprestada

Zero alocação na heap. O valor vive na stack (ou onde quer que o dono o tenha colocado) e você só empresta uma referência.

```rust
fn execute(op: &dyn MathOperation, a: f64, b: f64) -> f64 {
    op.calculate(a, b)
}

let addition = Addition;
let subtraction = Subtraction;
println!("{}", execute(&addition, 10.0, 5.0));    // 15
println!("{}", execute(&subtraction, 10.0, 5.0));  // 5
```

A opção mais leve, mas o valor precisa ter um dono em outro lugar e a referência não pode sobreviver a ele (lifetimes).

#### `Rc<dyn Trait>` e `Arc<dyn Trait>` — Ownership compartilhado

Para quando múltiplas partes do código precisam ser donas do mesmo valor.

```rust
use std::rc::Rc;   // single-thread
use std::sync::Arc; // thread-safe

let op: Rc<dyn MathOperation> = Rc::new(Addition);
let op_clone = Rc::clone(&op); // ambos apontam para o mesmo valor
```

### Resumo dos ponteiros com `dyn Trait`

| Ponteiro | Ownership | Alocação | Quando usar |
|---|---|---|---|
| `Box<dyn T>` | Exclusivo | Heap | Guardar em struct, retornar de função |
| `&dyn T` | Emprestado | Nenhuma | Uso temporário, parâmetro de função |
| `Rc<dyn T>` | Compartilhado | Heap | Vários donos, single-thread |
| `Arc<dyn T>` | Compartilhado | Heap | Vários donos, multi-thread |

Todos usam **dynamic dispatch**: em runtime, o programa consulta uma vtable para saber qual implementação chamar. Há um pequeno custo de indireção, mas permite polimorfismo real.

---

### 2. `impl Trait` — Polimorfismo Estático (compilação)

O compilador resolve o tipo concreto em compilação. Sem custo em runtime.

```rust
fn criar_collector() -> impl InputCollector {
    ConsoleCollector
}

let collector = criar_collector();
```

Apesar de a assinatura dizer `impl InputCollector`, o compilador sabe que é `ConsoleCollector`. O trait serve apenas para **encapsular/esconder** o tipo concreto na interface pública.

**Limitacao critica**: Nao permite polimorfismo em runtime.

```rust
// ERRO: os dois retornos são tipos diferentes
fn criar(opcao: bool) -> impl MathOperation {
    if opcao {
        Addition      // tipo A
    } else {
        Subtraction   // tipo B — conflito!
    }
}
```

O compilador precisa que `impl Trait` resolva para **um unico tipo concreto**. Se precisar retornar tipos diferentes, use `Box<dyn Trait>`.

---

### 3. Generics com Trait Bounds — Polimorfismo Estático (compilação)

O compilador gera uma versão especializada da função para cada tipo concreto (monomorphization).

```rust
fn execute<T: MathOperation>(op: &T, a: f64, b: f64) -> f64 {
    op.calculate(a, b)
}

execute(&Addition, 10.0, 5.0);    // compilador gera execute_Addition
execute(&Subtraction, 10.0, 5.0); // compilador gera execute_Subtraction
```

Máxima performance (o compilador pode até fazer inline), mas o tipo é fixado em compilação.

---

## DI em Structs

### Com `Box<dyn Trait>` — Flexível

```rust
struct Calculator {
    operation: Box<dyn MathOperation>,
}

impl Calculator {
    fn new(op: Box<dyn MathOperation>) -> Self {
        Calculator { operation: op }
    }

    fn run(&self, a: f64, b: f64) -> f64 {
        self.operation.calculate(a, b)
    }
}

let calc = Calculator::new(Box::new(Addition));
```

### Com Generics — Performático

```rust
struct Calculator<T: MathOperation> {
    operation: T,
}

impl<T: MathOperation> Calculator<T> {
    fn new(op: T) -> Self {
        Calculator { operation: op }
    }

    fn run(&self, a: f64, b: f64) -> f64 {
        self.operation.calculate(a, b)
    }
}

let calc = Calculator::new(Addition);
```

---

## Comparação com TypeScript

| TypeScript | Rust (dinâmico) | Rust (estático) |
|---|---|---|
| `interface` / `abstract class` | `trait` | `trait` |
| `let x: MathOp = new Add()` | `let x: Box<dyn MathOp> = Box::new(Add)` | `let x = Add` (tipo inferido) |
| Troca em runtime | Sim (`Box<dyn>`) | Não |
| Custo de dispatch | Sempre dinâmico | Zero (resolvido em compilação) |
| Memória | GC automático | Explícito (Box, &, Rc, Arc) |

---

## Quando Usar Cada Abordagem

- **Precisa trocar implementação em runtime?** -> `Box<dyn Trait>` (ou `&dyn Trait` se for temporário)
- **Tipo fixo, mas quer esconder na assinatura?** -> `impl Trait`
- **Máxima performance sem polimorfismo runtime?** -> Generics (`T: Trait`)
- **Vários donos do mesmo valor?** -> `Rc<dyn Trait>` ou `Arc<dyn Trait>`
