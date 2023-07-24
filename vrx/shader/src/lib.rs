extern crate proc_macro;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn descriptor(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    
    // struct declaration
    let struct_ = parse_macro_input!(item as syn::ItemStruct);

    // declartion
    let qto_struct = quote! { #struct_ };

    let ident = struct_.ident;
    let qto_trait = quote! {
        impl PossibleDescriptor for #ident {}
    };

    let tok_struct = proc_macro::TokenStream::from(qto_struct);
    let tok_trait  = proc_macro::TokenStream::from(qto_trait);

    let mut tokens = proc_macro::TokenStream::new();
    tokens.extend(tok_struct);
    tokens.extend(tok_trait);
    tokens
}
