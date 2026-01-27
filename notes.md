# Variáveis e Mutabilidade

## Declaração de variáveis
Por padrão, variáveis em Rust são **imutáveis**. Isso é uma das características que tornam o código mais seguro.

```rust
let x = 5;
// x = 6; // ERRO! não pode reatribuir variável imutável
```

## Variáveis mutáveis
Use `mut` para tornar uma variável mutável:
```rust
let mut x = 5;
println!("The value of x is: {x}");
x = 6; // OK!
println!("The value of x is: {x}");
```

## Shadowing (redeclaração)
Você pode declarar uma nova variável com o mesmo nome de uma anterior. A nova variável "shadowing" a anterior.

```rust
let x = 5;
let x = x + 1; // x agora é 6
{
    let x = x * 2; // x é 12 apenas neste escopo
    println!("The value of x in the inner scope is: {x}");
}
println!("The value of x is: {x}"); // x volta a ser 6
```

**Diferença entre shadowing e mut:**
- Shadowing permite mudar o **tipo** da variável:
```rust
let spaces = "   ";
let spaces = spaces.len(); // OK! mudou de &str para usize
```

- Com `mut` você **não pode** mudar o tipo:
```rust
let mut spaces = "   ";
// spaces = spaces.len(); // ERRO! tipo incompatível
```

<br/>

# Tipos de Dados

## Tipos escalares

### Inteiros
Rust tem vários tipos de inteiros, com e sem sinal:

| Tamanho | Com sinal | Sem sinal |
|---------|-----------|-----------|
| 8-bit   | i8        | u8        |
| 16-bit  | i16       | u16       |
| 32-bit  | i32       | u32       |
| 64-bit  | i64       | u64       |
| 128-bit | i128      | u128      |
| arch    | isize     | usize     |

- **Com sinal (i)**: podem armazenar números negativos e positivos
- **Sem sinal (u)**: apenas números positivos
- **isize/usize**: tamanho depende da arquitetura (64 bits em sistemas de 64 bits)

**Valores padrão:** Se não especificar, Rust usa `i32`

```rust
let x = 42; // i32 por padrão
let y: u8 = 255;
let z: i64 = -1000;
```

**Literais numéricos:**
```rust
let decimal = 98_222; // underscores para legibilidade
let hex = 0xff;
let octal = 0o77;
let binary = 0b1111_0000;
let byte = b'A'; // apenas u8
```

### Floats (ponto flutuante)
```rust
let x = 2.0; // f64 por padrão
let y: f32 = 3.0; // f32
```
- `f32`: 32 bits, precisão simples
- `f64`: 64 bits, precisão dupla (padrão)

### Char
Representa um caractere Unicode:
```rust
let c = 'z';
let z: char = 'ℤ';
let heart_eyed_cat = '😻';
```
- Usa aspas **simples** (não duplas!)
- Ocupa 4 bytes
- Suporta Unicode completo

<br/>

## Tipos compostos

### Tuplas
Agrupam valores de tipos diferentes em um único tipo composto:

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

// Desestruturação
let (x, y, z) = tup;
println!("The value of y is: {y}");

// Acesso por índice
let five_hundred = tup.0;
let six_point_four = tup.1;
let one = tup.2;
```

**Tupla vazia:** `()` é chamada de "unit" e representa valor vazio ou tipo de retorno vazio.

### Arrays
Coleção de elementos do **mesmo tipo** com tamanho **fixo**:

```rust
let a = [1, 2, 3, 4, 5];
let months = ["January", "February", "March", /* ... */];

// Especificando tipo e tamanho
let a: [i32; 5] = [1, 2, 3, 4, 5];

// Inicializar array com mesmo valor
let a = [3; 5]; // [3, 3, 3, 3, 3]

// Acesso
let first = a[0];
let second = a[1];
```

**Arrays vs Vetores:**
- Array: tamanho fixo, alocado na stack
- Vector (`Vec<T>`): tamanho dinâmico, alocado na heap

<br/>

# Strings

## String vs &str

### &str (string slice)
```rust
let s = "hello"; // tipo: &str
```
- Imutável
- Tamanho fixo
- Armazenada geralmente na memória do binário
- String literal

### String
```rust
let mut s = String::from("hello");
s.push_str(", world!");
```
- Mutável (se declarada com `mut`)
- Tamanho dinâmico
- Alocada na heap
- Possui ownership

<br/>

# Macros
Macros são identificadas pelo `!` no final:

```rust
println!("Hello, world!"); // macro
vec![1, 2, 3]; // macro para criar vetores
```

**Diferença de funções:**
- Macros podem receber número variável de parâmetros
- São expandidas em tempo de compilação
- Mais poderosas mas mais complexas

Exemplos de macros comuns:
- `println!()` - imprime com nova linha
- `print!()` - imprime sem nova linha
- `format!()` - formata string
- `vec!()` - cria vetor
- `panic!()` - termina programa com erro

<br/>

# 'statements' e 'exprensions'
É importante entender a diferença entre as duas coisas pois Rust é uma linguagem baseada em 'expression'

## statement
instrução que executa uma ação e não retorna um valor
### exemplos
-   criar uma variável e atribuir um valor com 'let'

### exemplos de código que gerariam erros
-   let x = (let y = 6);

<br/>

## expression
expressão resulta em um valor
### exemplos
-   '5 + 6' é uma 'expression' que resulta no valor 11
-   chamar uma função é um 'expression'
-   Chamar uma macro é uma 'expression'.
-   Um novo bloco de escopo criado com chaves é uma 'expression'.
```rust
let y = {
    let x = 3;
    x + 1
};
println!("The value of y is: {y}");
```
Nesse caso aquele bloco resulta em 4. Esse valor é associado a y como parte da instrução let. Observe que a linha x + 1 não tem um ponto e vírgula no final. Expressões não incluem ponto e vírgula no final. Se você adicionar um ponto e vírgula ao final de uma expressão, ela se transforma em uma instrução e, portanto, não retornará um valor.

Veja um exemplo de uma função que precisa retornar o valor e entenda como isso é feito:
```rust
fn five() -> i32 {
    5
}
```
logo isso:
```rust
let x = five();
```
é o mesmo que isso
```rust
let x = 5;
```
ambos resultam com x contendo o valor 5.

### usando 'if' com declarações 'let'
Se `if` for uma expressão, podemos usá-la no lado direito de uma instrução `let` para atribuir o resultado a uma variável.
```rust
let number = 3;
let even_odd = if number % 2 == 0 { "even" } else { "odd" };
println!("The number is {even_odd}");
// saída: The number is odd
```
```rust
let condition = true;
let number = if condition { 5 } else { 6 };
println!("The value of number is: {number}");
// saída: The value of number is: 5
```

**Importante:** Lembre-se que blocos de código avaliam para a última expressão neles, e números por si só também são expressões. O valor de toda a expressão `if` depende de qual bloco de código executa.

**Tipos devem ser consistentes:** Os valores que podem ser resultados de cada braço do `if` devem ser do **mesmo tipo**.

Exemplo de **erro** - tipos incompatíveis:
```rust
let condition = true;
let number = if condition { 5 } else { "six" }; // ERRO!
println!("The value of number is: {number}");
```

Por que isso é um erro?
- O braço `if` retorna um inteiro (`5`)
- O braço `else` retorna uma string (`"six"`)
- Variáveis devem ter um **tipo único**
- Rust precisa saber em **tempo de compilação** qual é o tipo da variável `number`
- Isso permite que o compilador verifique se o tipo é válido em todos os lugares onde `number` é usado
- Se o tipo fosse determinado apenas em tempo de execução, o compilador seria mais complexo e daria menos garantias sobre o código

<br/>

# Loops (Repetição)

Rust possui três tipos de loops: `loop`, `while`, e `for`.

## loop - loop infinito

O `loop` executa um bloco de código **para sempre** ou até você explicitamente parar com `break`:

```rust
loop {
    println!("again!");
}
```

**Controle de fluxo:**
- `break` - sai do loop
- `continue` - pula para a próxima iteração
- **ctrl-c** - interrompe o programa manualmente no terminal

### Retornando valores do loop

Você pode retornar um valor de dentro do `loop` usando `break` com um valor:

```rust
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2; // retorna counter * 2
    }
};
println!("The result is {result}"); // saída: 20
```

**Importante:** `return` sempre sai da função atual, enquanto `break` sai apenas do loop atual.

### Loop Labels (rótulos de loop)

Quando você tem loops aninhados, pode usar **labels** para especificar qual loop o `break` ou `continue` deve afetar:

```rust
let mut count = 0;
'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
        println!("remaining = {remaining}");
        if remaining == 9 {
            break; // sai apenas do loop interno
        }
        if count == 2 {
            break 'counting_up; // sai do loop externo
        }
        remaining -= 1;
    }
    count += 1;
}
println!("End count = {count}");
```

**Sintaxe:** Labels devem começar com aspas simples (`'label_name`).

<br/>

## while - loop condicional

O `while` executa enquanto a condição for **verdadeira**:

```rust
let mut number = 3;
while number != 0 {
    println!("{number}!");
    number -= 1;
}
println!("LIFTOFF!!!");
```

**Vantagem:** Elimina a necessidade de combinar `loop`, `if`, `else` e `break` manualmente.

<br/>

## for - iteração em coleções

O `for` é a forma **mais segura e concisa** de iterar sobre coleções:

```rust
let a = [10, 20, 30, 40, 50];

for element in a {
    println!("the value is: {element}");
}
```

### Por que `for` é melhor que `while` para iterar arrays?

**Com `while` (não recomendado):**
```rust
let a = [10, 20, 30, 40, 50];
let mut index = 0;
while index < 5 {
    println!("the value is: {}", a[index]);
    index += 1;
}
```

**Problemas do `while`:**
- Propenso a erros (índice incorreto pode causar panic)
- Mais lento (compilador adiciona verificações de bounds em cada iteração)
- Se mudar o tamanho do array, precisa ajustar a condição manualmente

**Com `for` (recomendado):**
- Mais seguro (não há risco de índice fora dos limites)
- Mais rápido (código de máquina mais eficiente)
- Mais conciso e legível

### Usando Range com `for`

Para repetir código um número específico de vezes, use **Range**:

```rust
// Countdown de 3 até 1
for number in (1..4).rev() {
    println!("{number}!");
}
println!("LIFTOFF!!!");
```

**Explicação:**
- `(1..4)` - gera a sequência 1, 2, 3 (não inclui o 4)
- `.rev()` - reverte a sequência para 3, 2, 1

**Por que usar `for` em vez de `while` para countdown?**
- Mesmo que você saiba exatamente quantas vezes quer repetir, `for` com Range é mais idiomático em Rust
- É o que a maioria dos Rustaceans usa
- Código mais limpo e seguro

<br/>

# Ownership (Propriedade)

Ownership é a característica mais única do Rust. Permite que Rust garanta segurança de memória **sem precisar de garbage collector**.

## Regras de Ownership

1. Cada valor em Rust tem um **owner** (dono)
2. Só pode haver **um owner** por vez
3. Quando o owner sai de escopo, o valor é **dropped** (liberado)

<br/>

## Stack vs Heap

### Stack
- Armazena valores na ordem que chegam, remove na ordem inversa (LIFO - last in, first out)
- Dados devem ter **tamanho fixo e conhecido** em tempo de compilação
- Operações são **rápidas** (push/pop sempre no topo)
- Parâmetros de função e variáveis locais ficam na stack

### Heap
- Menos organizado - você solicita espaço, o alocador encontra um local livre
- Retorna um **ponteiro** (endereço do local alocado)
- Dados podem ter **tamanho variável** ou desconhecido em tempo de compilação
- **Mais lento** que a stack (precisa buscar espaço e seguir ponteiros)

**Importante:** O propósito principal de ownership é gerenciar dados na heap.

<br/>

## Escopo de Variáveis

```rust
{                      // s não é válido aqui, ainda não foi declarado
    let s = "hello";   // s é válido daqui em diante
    // faz algo com s
}                      // escopo acabou, s não é mais válido
```

<br/>

## O Tipo String

### String literal (&str)
```rust
let s = "hello"; // tipo: &str
```
- Imutável
- Valor hardcoded no binário
- Tamanho conhecido em compilação

### String (heap)
```rust
let s = String::from("hello"); // alocado na heap
```
- Mutável (se declarado com `mut`)
- Tamanho pode mudar em runtime
- Precisa alocar memória

```rust
let mut s = String::from("hello");
s.push_str(", world!"); // append de literal
println!("{s}"); // hello, world!
```

<br/>

## Memória e Alocação

Com `String`:
1. Memória é **solicitada** do alocador em runtime (via `String::from`)
2. Memória precisa ser **devolvida** quando terminamos de usar

**Em Rust:** A memória é automaticamente liberada quando a variável sai de escopo.

```rust
{
    let s = String::from("hello"); // s é válido daqui
    // faz algo com s
}                                  // escopo acabou, Rust chama `drop` automaticamente
```

**Nota:** Em C++, esse padrão é chamado RAII (Resource Acquisition Is Initialization).

<br/>

## Move (Movimentação)

### Com tipos simples (stack)
```rust
let x = 5;
let y = x; // copia o valor, ambos x e y são válidos
println!("x = {x}, y = {y}"); // OK!
```

### Com String (heap)
```rust
let s1 = String::from("hello");
let s2 = s1; // s1 foi MOVIDO para s2

// println!("{s1}"); // ERRO! s1 não é mais válido
println!("{s2}"); // OK!
```

**O que acontece internamente:**
- String tem 3 partes na stack: ponteiro, length, capacity
- Quando `s2 = s1`, apenas esses dados da stack são copiados (não o conteúdo na heap)
- Para evitar double free, Rust **invalida** s1

**Isso é chamado de "move"**, não shallow copy.

### Reatribuição também libera memória
```rust
let mut s = String::from("hello");
s = String::from("ahoy"); // "hello" é liberado imediatamente
println!("{s}, world!"); // ahoy, world!
```

<br/>

## Clone (Cópia Profunda)

Para copiar os dados da heap também, use `clone`:

```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // copia tudo, incluindo dados na heap

println!("s1 = {s1}, s2 = {s2}"); // OK! ambos são válidos
```

**Atenção:** `clone` pode ser custoso em performance.

<br/>

## Copy Trait (Dados na Stack)

Tipos que ficam inteiramente na stack implementam o trait `Copy`:

```rust
let x = 5;
let y = x;
println!("x = {x}, y = {y}"); // OK! integers implementam Copy
```

**Tipos que implementam Copy:**
- Inteiros: `i32`, `u64`, etc.
- Booleanos: `bool`
- Floats: `f32`, `f64`
- Char: `char`
- Tuplas (se todos os elementos implementam Copy): `(i32, i32)` ✓, `(i32, String)` ✗

**Regra:** Se um tipo implementa `Drop`, não pode implementar `Copy`.

<br/>

## Ownership e Funções

Passar valores para funções segue as mesmas regras de atribuição (move ou copy):

```rust
fn main() {
    let s = String::from("hello");  // s entra em escopo
    takes_ownership(s);             // s é movido para a função
                                    // s NÃO é mais válido aqui

    let x = 5;                      // x entra em escopo
    makes_copy(x);                  // i32 implementa Copy
                                    // x ainda é válido aqui
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // some_string sai de escopo, `drop` é chamado, memória liberada

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
} // some_integer sai de escopo, nada especial acontece
```

<br/>

## Retorno de Valores e Ownership

Retornar valores também transfere ownership:

```rust
fn gives_ownership() -> String {
    let s = String::from("yours");
    s  // s é retornado e movido para quem chamou
}

fn takes_and_gives_back(s: String) -> String {
    s  // s é retornado e movido para quem chamou
}
```

<br/>

# References e Borrowing

Para usar um valor **sem tomar ownership**, use **referências**.

## Referências (&)

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // passa referência
    println!("The length of '{s1}' is {len}."); // s1 ainda é válido!
}

fn calculate_length(s: &String) -> usize { // s é uma referência
    s.len()
} // s sai de escopo, mas como não tem ownership, nada é liberado
```

**Conceito:** Criar uma referência é chamado de **borrowing** (empréstimo).

**Importante:** Referências são imutáveis por padrão:
```rust
fn change(s: &String) {
    // s.push_str(", world"); // ERRO! não pode modificar referência imutável
}
```

<br/>

## Referências Mutáveis (&mut)

Para modificar um valor emprestado, use `&mut`:

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}"); // hello, world
}

fn change(s: &mut String) {
    s.push_str(", world");
}
```

### Restrição: Apenas UMA referência mutável por vez

```rust
let mut s = String::from("hello");
let r1 = &mut s;
// let r2 = &mut s; // ERRO! não pode ter duas referências mutáveis

println!("{r1}");
```

**Por quê?** Previne **data races** em tempo de compilação.

Data race acontece quando:
1. Dois ou mais ponteiros acessam os mesmos dados simultaneamente
2. Pelo menos um está escrevendo
3. Não há sincronização

### Não pode misturar referências mutáveis e imutáveis

```rust
let mut s = String::from("hello");
let r1 = &s;     // OK
let r2 = &s;     // OK
// let r3 = &mut s; // ERRO! não pode ter &mut enquanto há &

println!("{r1}, {r2}");
```

### Escopo de referências (NLL - Non-Lexical Lifetimes)

Referências são válidas até seu **último uso**, não até o fim do bloco:

```rust
let mut s = String::from("hello");
let r1 = &s;
let r2 = &s;
println!("{r1} and {r2}"); // último uso de r1 e r2

let r3 = &mut s; // OK! r1 e r2 não são mais usados
println!("{r3}");
```

<br/>

## Dangling References (Referências Pendentes)

Rust **previne** dangling references em tempo de compilação:

```rust
fn dangle() -> &String {           // ERRO!
    let s = String::from("hello");
    &s                              // retorna referência para s
}   // s é liberado aqui, referência apontaria para memória inválida
```

**Solução:** Retorne o valor diretamente (transfere ownership):

```rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // ownership é movido para quem chamou
}
```

<br/>

## Regras de Referências (Resumo)

1. Em qualquer momento, você pode ter **OU** uma referência mutável **OU** qualquer número de referências imutáveis
2. Referências devem ser **sempre válidas**

<br/>

# Slices

Slices permitem referenciar uma **sequência contígua** de elementos em uma coleção, sem ter ownership.

## String Slices (&str)

```rust
let s = String::from("hello world");

let hello = &s[0..5];  // "hello"
let world = &s[6..11]; // "world"
```

**Sintaxe de range:**
```rust
let s = String::from("hello");

let slice = &s[0..2]; // "he"
let slice = &s[..2];  // mesmo que acima (início implícito)

let slice = &s[3..5]; // "lo"
let slice = &s[3..];  // mesmo que acima (fim implícito)

let slice = &s[0..5]; // "hello"
let slice = &s[..];   // string inteira
```

**Importante:** Índices devem estar em limites válidos de caracteres UTF-8.

<br/>

## Slices Previnem Bugs

Sem slices (problemático):
```rust
fn first_word(s: &String) -> usize {
    // retorna índice do fim da primeira palavra
    // ...
}

let mut s = String::from("hello world");
let word = first_word(&s); // word = 5
s.clear();                  // s agora é ""
// word ainda é 5, mas s está vazia! Bug!
```

Com slices (seguro):
```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

let mut s = String::from("hello world");
let word = first_word(&s); // word é um slice de s
// s.clear(); // ERRO! não pode ter &mut enquanto há & (word)
println!("the first word is: {word}");
```

<br/>

## String Literals são Slices

```rust
let s = "Hello, world!"; // tipo: &str
```

String literals são slices apontando para o binário - por isso são imutáveis.

<br/>

## Parâmetros como &str (Melhor Prática)

Prefira `&str` a `&String` em parâmetros de função:

```rust
fn first_word(s: &str) -> &str { // aceita &String e &str
    // ...
}

let my_string = String::from("hello world");
let word = first_word(&my_string[..]); // slice de String
let word = first_word(&my_string);     // &String → &str (deref coercion)

let my_literal = "hello world";
let word = first_word(&my_literal[..]); // slice de literal
let word = first_word(my_literal);      // literal já é &str
```

<br/>

## Slices de Arrays

Slices funcionam com outros tipos também:

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3]; // tipo: &[i32], contém [2, 3]
```