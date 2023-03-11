use inflector::Inflector;
use proc_macro::{Literal, TokenStream, TokenTree};

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
