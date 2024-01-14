# Pest

Rules have the form `name = optional-modifier { expression }`

Rust API:

```rust
#[derive(Parser)]
#[grammar = "path/to/grammar.pest"]
struct MyLang;
```

- This will generate the `Rule` enum as per the pest file.
- To parse as per a pest rule: `MyLang::parse(Rule::method, input)`.
- `parse` returns `Pairs`, which is an iterator over `Pair`s.
- `Pair` represents a match for a `Rule`, and is formed by `Token::Start` and
  `Token::End`. It is effectively a tree.
- `Pair::as_rule()`: the current `Rule`
- `Pair::as_str()`: the matching string
- `Pair::into_inner()`: returns `Pairs` of the sub-rules (or children of the node).

## Expressions

### Terminals

| Terminal   | Usage                                       |
|:-----------|:--------------------------------------------|
| `"a"`      | matches the exact string `"a"`              |
| `^"a"`     | matches the exact string `"a"` (ASCII only) |
| `'a'..'z'` | matches one character between `a` and `z`   |
| `a`        | matches rule `a`                            |

### Non-terminals

| Non-terminal                                           | Usage                                                      |
|:-------------------------------------------------------|:-----------------------------------------------------------|
| `(e)`                                                  | matches rule `e`                                           |
| `e1 ~ e2`                                              | matches sequence `e1 e2`                                   |
| `e1 \| e2`                                             | matches either `e1` or `e2`                                |
| `e*`, `e+`, `e{n}`, `e{, n}`, `e{n,}`, `e{m, n}`, `e?` | same as Regexp rules                                       |
| `&e`                                                   | matches `e` without a progress                             |
| `!e`                                                   | matches if `e` doesn't match, without making progress      |
| `PUSH(e)`                                              | matches `e` and pushes it's captured string down the stack |

## Modifiers

- Silent `_`: does not create token pair during parsing.

  ```pest
  // This rule
  a = _{ "a" }
  b = { a ~ "b" }
  ```

- Atomic `@`: does not accept whitespace or comments. This cascades for the nested
  rules. Rules called by Atomic do not create token pairs.
- Compound-atomic `$`: like atomic, but rules called by them can generate token pairs.
- Non-atomic `!`: Like normal rules, but stop the cascading effect of Atomic rules.
