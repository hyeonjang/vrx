extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashSet as Set;
use syn::parse::{Parse, ParseStream, Result};
use syn::parse_macro_input;
use syn::punctuated::Punctuated;

type SimpleExpr = (syn::Ident, syn::token::Eq, syn::Lit);

#[derive(Debug)]
struct GLSLBufferAttributes {
    // vars : Set<syn::ExprAssign>
    set: u32,
    binding: u32,
}

impl Parse for GLSLBufferAttributes {
    fn parse(input: ParseStream) -> Result<Self> {
        let vars = Punctuated::<syn::ExprAssign, syn::Token![,]>::parse_terminated(input)?;

        let mut set = 0;
        let mut binding = 0;

        // parse values
        vars.iter().for_each(|f| {
            let (l_expr, r_expr) = (&(*f.left), &(*f.right));
            match (l_expr, r_expr) {
                (syn::Expr::Path(l_expr), syn::Expr::Lit(r_expr)) => {
                    let lit = &r_expr.lit;
                    let lit_value = match lit {
                        syn::Lit::Int(lit) => lit.base10_parse::<u32>(),
                        &_ => todo!(),
                    };

                    if l_expr.path.segments[0].ident.to_string() == "set" {
                        set = lit_value.unwrap();
                    } else if l_expr.path.segments[0].ident.to_string() == "binding" {
                        binding = lit_value.unwrap();
                    }
                }
                _ => {}
            }
        });

        Ok(GLSLBufferAttributes { set, binding })
    }
}

#[proc_macro_attribute]
pub fn bind(input_attrib: TokenStream, input_struct: TokenStream) -> TokenStream {
    // struct declaration
    let struct_parse = parse_macro_input!(input_struct as syn::ItemStruct);
    let struct_ = quote! { #struct_parse };

    // trait declartion
    let traits_ = quote! {
        pub trait DescriptorMethods {
            fn get_descriptor_set_layout_binding() -> VkDescriptorSetLayoutBinding;
        }
    };

    let ident = struct_parse.ident;

    let attrib = parse_macro_input!(input_attrib as GLSLBufferAttributes);
    let binding = attrib.binding;

    let traits_impl = quote! {
        impl DescriptorMethods for #ident {
            fn get_descriptor_set_layout_binding() -> VkDescriptorSetLayoutBinding {
                VkDescriptorSetLayoutBindingBuilder::new()
                    .binding(#binding)
                    .build()
            }
        }
    };

    let qq = quote! {
        #struct_
        #traits_
        #traits_impl
    };

    let return_token = TokenStream::from(qq);

    // println!("{}", return_token);

    return_token
}
