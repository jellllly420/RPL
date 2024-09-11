use proc_macro::TokenStream;

extern crate rpl_mir_expand as expand;
extern crate rpl_mir_syntax as syntax;

#[proc_macro_attribute]
pub fn mir_pattern(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    let expanded = expand::expand(syn::parse_macro_input!(input as expand::MirPatternFn));
    expanded.into()
}

#[proc_macro]
pub fn mir(input: TokenStream) -> TokenStream {
    let expanded = expand::expand_mir(syn::parse_macro_input!(input as syntax::Mir));
    expanded.into()
}
