# pascal_ident_to_string

`pascal_ident_to_string` exports a procedural macro to convert an identifier to a string literal in pascal case
### Motivation
I like my identifiers snake-y. The Windows APIs don't. So, this macro helps me with converting the function names
to pascal case string literals before passing them to `GetProcAddress`
### Installation
Add it to your project with cargo
```rust
cargo add pascal_ident_to_string
```
### Example
```rust
use pascal_ident_to_string::pascal_string;

let my_rusty_ident = pascal_string!(my_rusty_ident);
assert_eq!(my_rusty_ident, "MyRustyIdent");
```

License: MIT
