use proc_macro::TokenStream;
use quote::format_ident;

#[proc_macro_attribute]
pub fn shader(derive_attr: TokenStream, item: TokenStream) -> TokenStream {
	let syn::ItemFn {
		attrs,
		vis,
		sig,
		block,
	} = syn::parse_macro_input!(item as syn::ItemFn);

	let args = syn::parse_macro_input!(derive_attr as syn::AttributeArgs);
	let fn_name = sig.ident.clone();
	let symbol_name = format_ident!("{}", fn_name.to_string().to_uppercase());

	let output = quote::quote! {
        // #[cfg(not(target_arch="spirv"))]
        #[allow(unused_variables)]
        #vis const #symbol_name : &str = concat!(module_path!(), "::", stringify!(#fn_name));

        // #[cfg(target_arch="spirv")]
		#[spirv( #( #args )* )]
        #(#attrs)* #vis #sig {
            #block
        }
    };

	output.into()
}
