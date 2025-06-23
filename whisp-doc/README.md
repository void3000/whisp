### Grammar Specific Features

| **Feature**          | **Description**                                                                    |
| -------------------- | ---------------------------------------------------------------------------------- |
| `bool`               | Represents a boolean value (`true` or `false`).                                    |
| `int`                | Represents integer numeric values.                                                 |
| `string`             | Represents text data enclosed in double quotes (`"..."`).                          |
| `array[]`            | A list of elements, all of the same type, defined using square brackets.           |
| **Variable Scoping** | Variables have local scope—within functions or blocks—ensuring clean lifetimes.    |
| `if-else`            | Conditional branching: executes a block if a condition is true, otherwise another. |
| `while` loop         | Repeats a block as long as a condition evaluates to true.                          |
| `for` loop           | Iterates over an array, binding each item to a loop variable.                      |
| **Functions**        | Named blocks of code with parameters, reusable and optionally returning values.    |

### Grammatically Valid Statements

#### Arithmeric

```
1 + 5 - 2 * 2;
```

#### Boolean

```
true == true
true != true
true and false
true or false
```

#### String

```
"Hello world!";
```

#### Variables (Binding)

```
let a = 1 + 5;
let b = 2;
b = b + a; // 8
```

#### Arrays

```
array[1, 2, 3, 4];
```

```
let z = array[1, 2, 3, 4];
z[2]; // 3
```

#### If-statement

```
let a = 1;
let b = 0;

// Variant (1)
if a > b {
    b = 1;
} else {
    b = 2;
}
b; // 1
```
```
// Variant (2)
if a > b {
    b = 1;
}
b; // 1
```

```
// Variant (3)
if a == b {
    b = 1;
} elif a > b {
    b = 2;
}
b; // 2
```

#### While-loop-statement

```
let a = 0;

while a < 5 {
    a = a + 1;
}
a; // 5
```

#### For-loop-statement

```
let acc = 0;
let arr = array[2, 1, 3, 7];

for v in arr {
    acc = acc + v;
}
acc; // 13
```

#### User-defined functions

```
def max(a, b) {
    if a > b {
        return a;
    } else {
        return b;
    }
}
max(7, 12); // 12
```

#### Builtin functions

| Function name | Signature       | Description                         |
| ------------- | --------------- | ----------------------------------- |
| print         | `print(<args>)` | Prints all arguments to the console |
| max           | `max(a, b)`     | Returns the maximum of two integers |
| min           | `min(a, b)`     | Returns the minimum of two integers |

### LL(k) Grammar

An **LL(k) grammar** is a type of context-free grammar that can be parsed from left to right, constructing a leftmost derivation of the input string using at most **k tokens of lookahead**. The name "LL(k)" comes from this parsing strategy: the first "L" stands for scanning the input from **Left to right**, the second "L" stands for producing a **Leftmost derivation**, and "k" represents the number of lookahead tokens used to guide parsing decisions. 

LL(k) grammars are particularly important because they allow for **predictive parsing**, where the parser can choose which rule to apply based on a fixed number of upcoming tokens, without requiring backtracking or guessing. LL(1) grammars, where only one token of lookahead is needed, are the most common and desirable subclass, often used in recursive descent parsers. 

However, not all grammars are LL(1); some may require more lookahead to resolve ambiguities, making them LL(k) for higher values of k. If a grammar is not LL(k) for any finite k, it typically cannot be parsed top-down without extra techniques like backtracking or left-factoring. Compared to bottom-up parsers (like LR parsers), LL parsers are simpler but more limited in the class of languages they can handle.

#### Whisp LL(k) Grammar

For the grammar notation we will be using [BNF notation](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form)

```
Program         ::= Stmts

Stmts           ::= Stmt Stmts
                  | ε

Stmt            ::= Expr ';'
                  | LetBinding
                  | ControlFlow
                  | Function
                  | Block

Expr            ::= ArithExpr
                  | BoolExpr
                  | AssignmentExpr
                  | Literal
                  | Identifier
                  | ArrayIndex
                  | Call

AssignmentExpr  ::= Identifier '=' Expr

ArithExpr       ::= ArithTerm ArithExprTail
ArithExprTail   ::= ArithOp ArithTerm ArithExprTail
                  | ε

BoolExpr        ::= BoolTerm BoolExprTail
BoolExprTail    ::= BoolOp BoolTerm BoolExprTail
                  | ε

LogicalExpr     ::= ArithTerm LogicalOp ArithTerm

BoolTerm        ::= Bool
                  | LogicalExpr
                  | Identifier

ControlFlow     ::= IfStatement
                  | WhileStatement
                  | ForStatement
                  | Return

ArithOp         ::= '+' | '-' | '*' | '/' | '%'
BoolOp          ::= 'and' | 'or'
LogicalOp       ::= '==' | '>' | '<' | '>=' | '<='

ArithTerm       ::= Int
                  | Identifier

Literal         ::= Int
                  | String
                  | Bool
                  | Array

Array             ::= '[' ArrayElements ']'
ArrayElements     ::= Expr ArrayElementsTail
                   | ε
ArrayElementsTail ::= ',' Expr ArrayElementsTail
                   | ε
ArrayIndex        ::= Identifier '[' (Int | Identifier) ']'

LetBinding      ::= 'let' Identifier '=' Expr ';'

IfStatement     ::= 'if' BoolExpr Block IfStatementTail

IfStatementTail ::= 'elif' BoolExpr Block IfStatementTail
                  | ElseStatement
                  | ε

ElseStatement   ::= 'else' Block

WhileStatement  ::= 'while' BoolExpr Block

ForStatement    ::= 'for' Identifier 'in' Array Block

Function        ::= 'def' Identifier '(' Params ')' Block

Call            ::= Identifier '(' Args ')'

Params          ::= Identifier ParamsTrail
                  | ε

ParamsTrail     ::= ',' Identifier ParamsTrail
                  | ε

Args            ::= ArgTerm ArgsTail
                  | ε
ArgsTail        ::= ',' ArgTerm ArgsTail
                  | ε

ArgTerm         ::= Literal
                  | Identifier

Block           ::= '{' Stmts '}'

Return          ::= 'return' Expr ';'

terminal Int;
terminal Bool;
terminal String;
terminal Identifier;
```

#### What the Grammar Allows

##### Basic Program Structure
- A `Program` consists of zero or more `Stmt`s.
- Statements can be:
  - Expressions (arithmetic or boolean)
  - Literals (e.g., `true;`, `42;`, `"hi";`)
  - Variable declarations (`let`)
  - Control flow (`if`, `while`, `for`, `return`)
  - Function declarations and function calls
  - Blocks (`{ ... }`)

##### Arithmetic Expressions
- Allowed: `1 + 2`, `x - 3 * 4 % 5`, `a / b + c`
- Operands must be:
  - `Int`
  - `Identifier` (assumed to refer to integer values)
- No mixing with booleans or strings.

##### Boolean Expressions
- Allowed:
  - Combinations of `BoolTerm`s using `and`, `or`
  - Examples: `true and false`, `x and y`
- `BoolTerm`s can be:
  - Boolean literals (`true`, `false`)
  - Logical comparisons like `1 == 1`, `x > y`
  - Identifiers (e.g., `flag`, assumed to be boolean)

##### Logical Comparisons
- `LogicalExpr ::= ArithTerm LogicalOp ArithTerm`
- Allowed:
  - `3 > 2`, `x == y + 1`, `a <= b % 2`
- Must compare arithmetic terms only (no booleans or strings).
- Used within `BoolExpr` via `BoolTerm`.

##### Control Flow
- `if`, `while`, and `elif` conditions must be valid `BoolExpr`s
- Encourages clean boolean logic like: `if x > 2 and y < 5 { ... }`

##### Literals
- Can appear as standalone statements: `true;`, `42;`, `["a", "b"];`

##### Arrays
- Arrays: `[1, 2 + 3, x]`
- Elements must be valid `Expr`s

##### Functions
- Declare with: `def name(param1, param2) { ... }`
- Parameters must be identifiers
- Arguments must be `Expr`s (can be arithmetic, boolean, identifiers)

#### What the Grammar Disallows

##### Mixed-Type Arithmetic
- Invalid: `true + 1`, `"hello" - "world"`, `false * x`

##### Chained Boolean Logic on Non-Boolean Values
- Disallowed:
  - `1 and 1` (invalid: `1` is not boolean)
  - `true and 3 > 2` (invalid unless `3 > 2` is boxed properly)

##### String Comparisons
- Not allowed (e.g., `"a" == "b"` is invalid)

##### Functions as First-Class Values
- Cannot assign functions to variables or pass them as arguments

##### Complex Inline Comparisons
- `ArithTerm` does not support grouping via parentheses
- Expressions like `(x + y) > (z - 1)` not allowed unless extended

