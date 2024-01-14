# Pest

- Rules have the form `name = optional-modifier { expression }`.
- Refer examples: [csv.pest](./src/csv.pest) and [ini.pest](./src/ini.pest).

## Parsing expression grammar (PEG)

- Eager / greedy. Repetition rules consume as much as they can and pass on the remaining
  input to the next step.
- No backtracking. If a rule fails, the parser won't backtrack to un-consume and retry.
- Unambiguous.

## Rust API:

```rust
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "path/to/grammar.pest"]
struct MyParser;

// Or define the grammar inline
#[derive(Parser)]
#[grammar_inline="..."]
struct MyInlineParser;
```

- This will generate the `Rule` enum as per the pest file, and implement the `Parser`
  trait for `MyParser`.
- To parse as per a pest rule: `MyParser::parse(Rule::method, input)`.
- `parse` returns `Pairs`, which is an iterator over `Pair`s.
- `Pair` represents a match for a `Rule`, and is formed by `Token::Start` and
  `Token::End`. It is effectively a tree.
- `Pair::as_rule()`: the current `Rule`
- `Pair::as_str()`: the matching string
- `Pair::into_inner()`: returns `Pairs` of the sub-rules (or children of the node).

## Pest Grammar

### Expressions

- Terminals

  | Terminal   | Usage                                                          |
  |:-----------|:---------------------------------------------------------------|
  | `"a"`      | matches the exact string `"a"`                                 |
  | `^"a"`     | matches the exact string `"a"` case-insensitively (ASCII only) |
  | `'a'..'z'` | matches one character between `a` and `z`                      |
  | `a`        | matches rule `a`                                               |

- Sequence: `first ~ and_then`. If `first` succeeds, it attempts to parse the
  _remaining_ with `and_then`.
- Ordered-choice: `first | or_else`. If `first` succeeds, then the expression succeeds.
  Else `or_else` is attempted from the same position as `first`.
  - Ordering can matter in case of the choice-operator. `"a" | "ab"` will match only
    `"a"` from the string `"abc"`, and leave `"bc"` unparsed. Heuristic: put the longest
    or the most specific one first.
- Repetition: `e*`, `e+`, `e{n}`, `e{, n}`, `e{n,}`, `e{m, n}`, `e?`. Same as Regular
  Expressions.
- Predicates: `&e` (positive match) and `!e` (negative match) match without consuming
  input - like "lookahead".
  - `not_space_or_tab = { !( " " | "\t" ) ~ ANY }`: consume if the character is not
    space or tab.

### Precedence

1. Repetition operators
2. Sequence operator
3. Ordered choice

### Modifiers

The left curly bracket `{` defining a rule can be preceded by symbols that affect its
operation:

- Silent `_`: does not create token pair during parsing.

  ```pest
  // This rule won't generate a token pair
  a = _{ "a" }
  b = { a ~ "b" }
  ```

- Atomic `@`: does not accept whitespace or comments. This cascades for the nested
  rules. Rules called by Atomic do not create token pairs.
- Compound-atomic `$`: like atomic, but rules called by them can generate token pairs.
- Non-atomic `!`: Like normal rules, but stop the cascading effect of Atomic rules.

### Built-in Rules

- `ASCII_DIGIT`: equivalent to `'0'..'9'`
- `ASCII_NONZERO_DIGIT`: equivalent to `'1'..'9'`
- `ASCII_BIN_DIGIT`, `ASCII_OCT_DIGIT`,`ASCII_HEX_DIGIT`
- `ASCII_ALPHA_LOWER`, `ASCII_ALPHA_UPPER`,`ASCII_ALPHA`
- `ASCII_ALPHANUMERIC`
- `NEWLINE`: `"\n" | "\r\n" | "\r"`

### Special Rules

- `WHITESPACE`: runs between rules and sub-rules
- `COMMENT`: runs between rules and sub-rules
- `ANY`: any one char
- `SOI`: start-of-input
- `EOI`: end-of-input
- `POP`: pops a string from the stack and matches it
- `POP_ALL`: pops the entire state of the stack and matches it
- `PEEK`: peeks a string from the stack and matches it
- `PEEK[a..b]`: peeks part of the stack and matches it
- `PEEK_ALL`: peeks the entire state of the stack and matches it
- `DROP`: drops the top of the stack (fails to match if the stack is empty)

WHITESPACE and COMMENT should be defined manually if needed. All other rules cannot be overridden.

### `WHITESPACE` AND `COMMENT`

- Active only when defined.
- Does not apply to atomic rules.
- Applies to sequences (`~`), repetitions (`*`, `+`), and between expressions.
- Should be defined to match only one whitespace or comment, since the rule runs in
  repetition.
- `a = { b ~ c }` is effectively transformed to:
  ```pest
  a = { b ~ WHITESPACE* ~ (COMMENT ~ WHITESPACE*)* ~ c }
  ```
