use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Describe)]
pub fn derive_describe(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_derive_describe(&ast)
}

fn impl_derive_describe(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = match &ast.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(f) => &f.named,
            _ => panic!("Only named fields"),
        },
        _ => panic!("Only structs"),
    };

    let field_strings = fields.iter().map(|f| {
        let fname = f.ident.as_ref().unwrap();
        let fname_str = fname.to_string();
        quote! { format!("{}: {:?}", #fname_str, self.#fname) }
    });

    let gen_code = quote! {
        impl Describe for #name {
            fn describe(&self) -> String {
                let fields = vec![#(#field_strings),*];
                format!("{} {{ {} }}", stringify!(#name), fields.join(", "))
            }
        }
    };
    gen_code.into()
}

// Example Test
//#[test]
//fn test_example() {
//    #[derive(Describe)]
//    struct Person {
//        name: String,
//        age: u32,
//    }
//
//    let person = Person {
//        name: "Alice".to_string(),
//        age: 30,
//    };
//
//    assert_eq!(person.describe(), "Person { name: \"Alice\", age: 30 }");
//}
