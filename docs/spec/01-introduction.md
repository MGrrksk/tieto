
# 1. Introduction

This is the official language specification for Tieto v0.1.0. It is intended to serve all the important technical informations about the language and its toolkit, including lexical structure, syntax, semantics, language constructs, type system, memory model, bytecode structure, standard library and tools.

Tieto is designed for develpoment of systems and software, but is basically a general-purpose language, with the main goals being efficiency and expressiveness. It has a strong static type system with additional dynamic type `dyn` for values with unspecified type. Tieto is compiled to bytecode and then executed by Tieto Virtual Machine.

> **Warning:** Tieto is currently in an experimental phase with early access. It is not suitable for production environments yet. Expect frequent changes in features and behavior and use with caution.

# 1.1. Notation

Lexical structure and syntax are described using a variant of the Extended Backus-Naur Form (EBNF). It consists of **terminals** - literal elements, which are strings of characters in lexical structure or *[tokens](./02-lexical-structure.md#24-tokens)* in syntax, written as double-quoted strings, and **non-terminals** - name referencing a production rule. Production rules define how terminals and non-terminals can combine into valid sequences. There is an example syntax of EBNF:

```ebnf
; this is a comment
text = { word | number } .
word = letter + .
letter = "A" ... "Z" | "a" ... "z" .
number = "0" | ( ["-"] "0" ... "9" + ) .
```

Production expressions consist of these terms and following operators, in decreasing precedence (`_` symbolizes an operand):

| operator  | meaning                      |
| :-------: | :--------------------------- |
| `( _ )`   | grouping                     |
| `[ _ ]`   | none or once                 |
| `{ _ }`   | none or more                 |
| `_ ... _` | any within range (inclusive) |
| `_ +`     | once or more                 |
| `_ | _`   | alternation                  |
| `_ - _`   | exception                    |

Some characters can be represented by notation `\uXXXX`, where `XXXX` is four-digit hexadecimal number of the Unicode code point, for example:

```ebnf
newline = "\u000A" .
```

Following escape codes can be used to avoid special meanings of some characters:

| escape code | result character |
| :---------: | :--------------: |
| `\"`        | `"`              |
| `\\`        | `\`              |

When listing all the possibilities is unpractical, descriptive phrase embraced with backticks (`\``) will be used, for instance:

```ebnf
character = `Any Unicode character` .
```

[< Previous chapter: Index](./README.md)
[Next chapter: Lexical Structure >](./02-lexical-structure.md)