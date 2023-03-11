//! `pascal_string` exports a procedural macro to convert an identifier to a string literal in pascal case
//! ## Motivation
//! I like my identifiers snake-y. The Windows APIs don't. So, this macro helps me with converting the function names
//! to pascal case string literals before passing them to `GetProcAddress`
//! ## Example
//! ```
//! let my_rusty_ident = pascal_string!(my_rusty_ident);
//! assert_eq!(my_rusty_ident, "MyRustyIdent");
//! ```

use inflector::Inflector;
use proc_macro::{Literal, TokenStream, TokenTree};

/// Convert an identifier to a pascal case string literal
/// e.g. `my_rusty_identifier` becomes `MyRustyIdentifier`
#[proc_macro]
pub fn pascal_string(stream: TokenStream) -> TokenStream {
    stream
        .into_iter()
        .map(|token_tree| match token_tree {
            TokenTree::Ident(ref ident) => {
                let ident = ident.to_string().to_pascal_case();
                Literal::string(&ident).into()
            }
            _ => token_tree,
        })
        .collect()
}
