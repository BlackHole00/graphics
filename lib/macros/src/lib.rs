extern crate syn;
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::{punctuated::Punctuated, Token};

macro_rules! easy_derive {
    ($trait_name: ident, $input: ident) => {{
        let item = syn::parse_macro_input!($input as syn::ItemStruct);
        let struct_name = item.clone().ident;

        let output = quote! {
            impl $trait_name for #struct_name {}
        };

        return output.into();
    }};
}

struct Args {
    args: HashSet<syn::ExprCall>,
}

impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> Result<Self, syn::Error> {
        let args = Punctuated::<syn::ExprCall, Token![,]>::parse_terminated(input)?;

        Ok(Args {
            args: args.into_iter().collect(),
        })
    }
}

#[proc_macro_attribute]
pub fn main_app(args: TokenStream, input: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(input as syn::ItemStruct);
    let args = syn::parse_macro_input!(args as Args);

    let mut properties_calls = quote! {};
    for arg in args.args {
        properties_calls = quote! {
            #properties_calls.#arg
        }
    }

    let struct_name = item.clone().ident;
    let output = quote! {
        #item

        fn main() {
            use ::window::Window as AppWindow;
            use glutin::*;
            use glutin::dpi::LogicalSize;
            use glutin::window::*;
            use glutin::monitor::*;

            let window_builder = WindowBuilder::new().with_title("Application").with_inner_size(LogicalSize::new(512, 512))#properties_calls;
            let mut window = AppWindow::new(window_builder);
            window.run::<#struct_name>();
        }
    };

    output.into()
}

#[proc_macro_derive(VboObject)]
pub fn derive_vbo_object(input: TokenStream) -> TokenStream {
    easy_derive!(VboObject, input)
}

#[proc_macro_derive(VaoObject)]
pub fn derive_vao_object(input: TokenStream) -> TokenStream {
    easy_derive!(VaoObject, input)
}

#[proc_macro_derive(EboObject)]
pub fn derive_ebo_object(input: TokenStream) -> TokenStream {
    easy_derive!(EboObject, input)
}

#[proc_macro_derive(TextureObject)]
pub fn derive_texture_object(input: TokenStream) -> TokenStream {
    easy_derive!(TextureObject, input)
}
