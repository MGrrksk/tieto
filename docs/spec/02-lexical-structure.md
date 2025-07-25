# 2. Lexical Structure

## 2.1. Source Code Structure

Source code is a valid UTF-8 text. Every code point is distintct, which means that uppercase and lowercase leters are different characters, and also code point of a letter with a mark is distinct from two code points representing separately a letter and a mark. All newlines in a source code are normalized to the `U+000A` Unicode code point before any other operation is performed.

Characters in source code are classified accordingly to these rules:

```ebnf
character = `Any UTF-8 character` .
letter = "A" ... "Z" | "a" ... "z" | "_" .
digit = "0" ... "9" .
whitespace = "\u0009" | "\u000D" | "\u0020" .
end_of_line = "\u000A" | `End Of File` .
```

All lexical elements of the source code are made of only ASCII characters, except for *[comments](#23-comments)* and *[string literals](#253-string-literals)*.

Whitespace characters are ignored, except for situations when they separate two distinct *[tokens](#24-tokens)* that would be parsed as one otherwise. *[Newline](#261-newline)* is not ignored and generates its own token.

## 2.2. Lexical Elements

### 2.2.1. Comments

Comments are strings of characters which act the same way as whitespace. They start with a hash character (`#`) and stop at the end of the line. Comments are not recognized inside *[string literals](#253-string-literals)*.

```ebnf
comment = "#" { character } end_of_line .
```

### 2.2.2. Tokens

Tokens are main components of the Tieto syntax. There are four classes of tokens: *[identifiers](#23-identifiers-and-keywords)*, *[operators and delimiters](#24-operators-and-delimiters)*, *[literals](#25-literals)*, and *[special tokens](#254-special-tokens)*. When several different tokens can be generated from the input, the longest possible match is used.

## 2.3. Identifiers and Keywords

Identifiers function as names for program elements. They consist of string of letters or digits, starting from a letter. 

```ebnf
identifier = letter { letter | digit } .
```

Keywords are special words reserved by the language for special purposes. Their syntax is the same as identifiers' syntax, but cannot be used as identifiers. These are Tieto's keywords:

```
u1         null
u8         and
u16        or
u32        dyn
u64        if
i8         else
i16        for
i32        while
i64        var
f32        func
f64        return
```

## 2.4. Operators and Delimiters

These character combinations serve as operators and delimiters in Tieto's grammar:

```
+    ,    (    )    ?    =    ==   =>
-    .    [    ]    %    >    >=   !
*    **   {    }    ..   <    <=   !=
/    //   :    ;    |>
```

## 2.5. Literals

Literals represent primitive constant values in the source code. There are three literals which are recognized during lexical analysis: *[integer literals](#251-integer-literals)*, *[floating-point literals](#252-floating-point-literals)* and *[string literals](#253-string-literals)*. These are also parsed at this stage, which means that the values they represent are computed and token contains this value.

### 2.5.1. Integer Literals

Integer literals are sequences of digits representing decimal 64-bit integer number values. They are described by this rule:

```ebnf
integer = digit + .
```

### 2.5.2. Floating-Point Literals

Floating-point literals consist of two sequences of digits separated by a dot (`.`). Both of these sequences are required. These literals are parsed into 64-bit floating-point number values.

```ebnf
float = digit + "." digit + .
```

### 2.5.3. String Literals

String literals consist of zero or more UTF-8 characters enclosed with a double-quote characters (`"`). End of line inside string literal is disallowed and results in an *[error token](#262-error-token)*. They are parsed into arrays of unsigned bytes representing UTF-8 code points.

```ebnf
string = "\"" { character - end_of_line } "\"" .
```

## 2.6. Special Tokens

### 2.6.1. Newline

Newline token is generated from the `U+000A` character and serves mostly as a separator in the source code. It is not recognized inside *[string literal](#253-string-literals)*, even if invalid.

```ebnf
newline = "\u000A" .
```

### 2.6.2. Error Token

This token is generated when lexical analyser encounters an unrecognized character or ill-formed token, like uterminated string. Error token contains string value representing a message describing the cause of an error.

### 2.6.3. EOF Token

EOF is emitted as the last token in the source code and terminates the sequence of tokens.

```ebnf
eof = `End Of File` .
```

## 2.7. Other Characters

Every character that does not match any of the lexical structure production rules generates an *[error token](#262-error-token)*.

[< Previous chapter: Introduction](./01-introduction.md)