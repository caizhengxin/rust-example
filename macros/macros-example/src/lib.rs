extern crate proc_macro;

use proc_macro::{TokenStream};
use quote::quote;
use syn::{
    parse as other_parse, parse_str, Item, Meta, Path, Type, Lit, DeriveInput, parse_macro_input, ItemFn, Data, DataStruct,
    Field, Fields,
    punctuated::Punctuated, token::Comma, Attribute,
    Generics,
};


// #[proc_macro_attribute]
// pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
//     let attr_string = attr.to_string();
//     let meta = parse::<Meta>(attr).unwrap_or_else(|_| panic!("{} is not a valid attribute", attr_string));

//     match meta {
//         Meta::Path(v) => {
//             println!("[DEBUG]: Path");

//             if v == parse_str::<Path>("jkc").unwrap() {
//                 println!("jkc");
//             }

//         },
//         Meta::List(_v) => {
//             println!("[DEBUG]: List");
//         },
//         Meta::NameValue(v) => {
//             println!("[DEBUG]: NameValue");

//             if v.path == parse_str::<Path>("jkc").unwrap() {
//                 match v.lit {
//                     Lit::Int(lv) => {
//                         let value = lv.base10_digits().parse::<u16>().unwrap();

//                         println!("value: {:?}", value);
//                     },
//                     Lit::Bool(lv) => {
//                         let value = lv.value;

//                         println!("value: {:?}", value);
//                     },
//                     Lit::Byte(_lv) => {},
//                     Lit::ByteStr(_lv) => {},
//                     Lit::Char(_lv) => {},
//                     Lit::Float(_lv) => {},
//                     Lit::Str(lv) => {
//                         println!("value: {:?}", lv.value());
//                         let value = parse_str::<Type>(&lv.value()).unwrap_or_else(|_| panic!("Invalid type supplied: {}", lv.value()) );
//                         println!("value: {:?}", value);
//                     },
//                     Lit::Verbatim(_lv) => {},
//                 }
//             }
//         },
//     }

//     let item_tmp;

//     match parse::<Item>(item).unwrap() {
//         Item::Struct(e) => {
//             item_tmp = e;
//         },
//         _ => { panic!("This macro only operates on structs"); },
//     }

//     let name = item_tmp.ident.clone();

//     println!("attrs: {:?}", item_tmp.attrs);
//     println!("fields: {:?}", item_tmp.fields);

//     TokenStream::from(quote!(
//         #[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
//         #item_tmp

//         impl #name {
//             pub fn jankincai(&self) {
//                 println!("jankincai");
//             }
//         }
//     ))
// }


// pub trait JkcTrait {
//     fn print_jkc(&self);
// }


// pub fn gen_for_struct(struct_name: &Ident, generics: &Generics, fields: &Punctuated<Field, Comma>, attrs: &[Attribute]) -> TokenStream {
//     let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

//     quote!(
//         impl #impl_generics JkcTrait for #struct_name #ty_generics #where_clause {
//             fn print_jkc(&self) {
//                 println!("[DEBUG]: jkc");
//             }
//         }
//     )
// }


// pub fn derive_args(input: &DeriveInput) -> TokenStream {
//     let ident = &input.ident;

//     match input.data {
//         Data::Struct(DataStruct{
//             fields: Fields::Named(ref fields),
//             ..
//         }) => gen_for_struct(&ident, &input.generics, &fields.named, &input.attrs),
//         Data::Struct(DataStruct{
//             fields: Fields::Unit,
//             ..
//         }) => gen_for_struct(&ident, &input.generics, &Punctuated::<Field, Comma>::new(), &input.attrs),
//         _ => {panic!("`#[derive(Args)]` only supports non-tuple structs");}
//     }
// }


#[proc_macro_derive(JkcDerive)]
pub fn derive_jkcderive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    dbg!(&input);

    let name = input.ident;

    let expanded = quote! {
        impl #name {
            fn print(&self) -> usize {
                println!("{}","hello from #name");

                1
           }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
