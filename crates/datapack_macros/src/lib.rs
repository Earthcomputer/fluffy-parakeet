use proc_macro::TokenStream;
use quote::quote;
use syn::{parse2, parse_macro_input, Data, DeriveInput, Error, Fields};

#[proc_macro_derive(DispatchDeserialize)]
pub fn derive_dispatched(item: TokenStream) -> TokenStream {
    let derive_item = parse_macro_input!(item as DeriveInput);
    let Data::Enum(derive_enum) = &derive_item.data else {
        return Error::new_spanned(derive_item.ident, "Dispatched must be an enum")
            .to_compile_error()
            .into();
    };

    let struct_name = derive_item.ident;

    let mut generics = derive_item.generics.clone();
    if generics.params.is_empty() {
        generics = parse2(quote! { <'de> }).unwrap()
    } else {
        generics.params.insert(0, parse2(quote! { 'de }).unwrap());
    }
    let where_clause = generics.where_clause;
    generics.where_clause = None;

    let type_tests: Vec<_> = derive_enum
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;

            let Fields::Unnamed(unnamed_fields) = &variant.fields else {
                return Error::new_spanned(
                    variant_name,
                    "Dispatched variant must be a single-value tuple variant",
                )
                .into_compile_error()
                .into();
            };
            if unnamed_fields.unnamed.len() != 1 {
                return Error::new_spanned(
                    variant_name,
                    "Dispatched variant must be a single-value tuple variant",
                )
                .into_compile_error()
                .into();
            }

            // convert variant name from pascal case to snake case
            let mut identifier_name = variant_name.to_string();
            for i in (1..identifier_name.len()).rev() {
                if identifier_name.as_bytes()[i].is_ascii_uppercase() {
                    identifier_name.insert(i, '_');
                }
            }
            identifier_name.make_ascii_lowercase();

            quote! {
                #identifier_name => Ok(Self::#variant_name(::serde::de::Deserialize::deserialize(
                    ::serde_json::value::Value::Object(obj)
                ).map_err(|err| ::serde::de::Error::custom(err))?)),
            }
        })
        .collect();

    From::from(quote! {
        impl #generics ::serde::de::Deserialize<'de> for #struct_name #where_clause {
            fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>
            {
                let mut obj = ::serde_json::map::Map::deserialize(deserializer)?;
                let Some(ty) = obj.remove("type") else {
                    return Err(::serde::de::Error::missing_field("type"));
                };
                let ::serde_json::value::Value::String(ty) = ty else {
                    return Err(::serde::de::Error::invalid_type(
                        ::serde::de::Unexpected::Other("non-string"),
                        &"identifier"
                    ));
                };
                match ty.strip_prefix("minecraft:").unwrap_or(&ty) {
                    #(#type_tests)*
                    _ => Err(::serde::de::Error::invalid_value(
                        ::serde::de::Unexpected::Str(&ty),
                        &"valid type"
                    )),
                }
            }
        }
    })
}
