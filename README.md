# Elsa Language Extension for Zed

Syntax highlighting for [Elsa](https://github.com/ucsd-progsys/elsa) — UCSD's lambda calculus teaching language — in the [Zed](https://zed.dev) editor.

## Features

- Syntax highlighting for `.lc` files
- Highlights: keywords (`let`, `eval`, `conf`), step operators (`=b>`, `=d>`, `=n*>`, …), lambda syntax (`\x -> …`), variables, comments
- Line comment toggling with `--`
- Bracket matching for parentheses and block comments `{- -}`

## Elsa language example

```haskell
-- define a term
let id   = \x -> x
let zero = \f x -> x
let succ = \n f x -> f (n f x)

-- prove a reduction sequence (final term must be in normal form)
eval succ_zero :
  succ zero
  =d> (\n f x -> f (n f x)) (\f x -> x)
  =b> \f x -> f ((\f x -> x) f x)
  =b> \f x -> f ((\x -> x) x)
  =b> \f x -> f x

-- confirm a partial reduction (no normal form required)
conf omega :
  (\x -> x x) (\x -> x x)
  =b> (\x -> x x) (\x -> x x)
```

### Step operators

| Operator | Meaning |
|----------|---------|
| `=a>` | Alpha equivalence |
| `=b>` | Beta reduction (single) |
| `=e>` | Eta reduction |
| `=d>` | Definition expansion |
| `=n>` | Normal order beta |
| `=p>` | Applicative order beta |
| `=*>` | Transitive closure |
| `=~>` | Normalizes to |
| `=n*>` | Normal order transitive |
| `=p*>` | Applicative order transitive |

Append `:s`, `:w`, or `:h` for strong, weak, or head normal form checks (e.g. `=b:w>`).

## Installation

### From Zed extension registry

Search for **Elsa** in `zed: extensions`.

### Local development

```bash
# In Zed, open command palette and run:
# zed: install dev extension
# Point it at the zed-elsa directory
```
