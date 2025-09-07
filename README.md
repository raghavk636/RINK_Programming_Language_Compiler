# Rink Programming Language Compiler

Rink is a small, experimental programming language and compiler that I’m building to explore how programming languages work under the hood.  
The goal of Rink is to provide a simple but powerful foundation for learning about **parsing, compilation, and execution** while keeping the language itself approachable.

---

## 🚀 Features

- **Custom Language Syntax** – Rink programs use a simple, C-like syntax with clear rules.
- **Compiler Implementation** – Written in [Rust/Java/C/etc. — pick your language], the compiler handles:
  - Lexical analysis (tokenizing)
  - Parsing into an Abstract Syntax Tree (AST)
  - Semantic checks
  - Code generation (currently targeting [bytecode/assembly/VM/other])
- **Basic Control Flow** – `if/else` statements, loops, and functions.
- **Arithmetic & Variables** – Support for integers, variables, and expressions.
- **Error Handling** – Meaningful error messages for common mistakes.
- **(Planned)** Type system, standard library, and optimizations.

---

## 📦 Installation

Clone the repository:

```bash
git clone https://github.com/<your-username>/rink-compiler.git
cd rink-compiler

Build the compiler:

cargo build --release   # if Rust
# OR
javac RinkCompiler.java # if Java

Compile a Rink program:

./rink examples/hello.rk


Run the compiled output:

./rinkc examples/hello.rk


Example Rink program (hello.rk):

func main() {
    let x = 5;
    let y = 10;
    if (x < y) {
        print("x is less than y!");
    }
}
