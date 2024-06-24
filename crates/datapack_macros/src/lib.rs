use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{format_ident, quote};
use std::collections::BTreeSet;
use syn::parse::{Parse, ParseStream};
use syn::visit::Visit;
use syn::{
    parse2, parse_macro_input, parse_quote, Data, DeriveInput, Error, Fields, GenericParam,
    Generics, ImplGenerics, Index, Lifetime, LitStr, Path, Token, TypeGenerics, WhereClause,
    WherePredicate,
};

fn deserialize_impl_generics<'a>(
    generics: &'a Generics,
    lifetime_generics: &'a mut Generics,
) -> (ImplGenerics<'a>, TypeGenerics<'a>, Option<&'a WhereClause>) {
    lifetime_generics
        .params
        .insert(0, parse2(quote! { 'de }).unwrap());
    let additional_bounds = generics.type_params().map(|type_param| {
        parse2::<WherePredicate>(quote! { #type_param: ::serde::de::Deserialize<'de> }).unwrap()
    });
    if let Some(where_clause) = &mut lifetime_generics.where_clause {
        where_clause.predicates.extend(additional_bounds);
    } else {
        lifetime_generics.where_clause =
            Some(parse2(quote! { where #(#additional_bounds,)* }).unwrap());
    }
    let (_, ty_generics, _) = generics.split_for_impl();
    let (impl_generics, _, where_clause) = lifetime_generics.split_for_impl();
    (impl_generics, ty_generics, where_clause)
}

enum DispatchDirective {
    Inlinable(Option<Path>),
    Rename(String),
}

impl Parse for DispatchDirective {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        match ident.to_string().as_str() {
            "inlinable" => {
                if input.peek(Token![=]) {
                    input.parse::<Token![=]>()?;
                    let deserialize_func: LitStr = input.parse()?;
                    let deserialize_func: Path = deserialize_func.parse()?;
                    Ok(DispatchDirective::Inlinable(Some(deserialize_func)))
                } else {
                    Ok(DispatchDirective::Inlinable(None))
                }
            }
            "rename" => {
                input.parse::<Token![=]>()?;
                let name_tok: LitStr = input.parse()?;
                let name = name_tok.value();
                if name.is_empty()
                    || name.contains(
                        |char| !matches!(char, '_' | '-' | 'a'..='z' | '0'..='9' | '/' | '.'),
                    )
                {
                    return Err(Error::new_spanned(name_tok, "invalid identifier path"));
                }
                Ok(DispatchDirective::Rename(name))
            }
            _ => Err(Error::new_spanned(ident, "unknown directive")),
        }
    }
}

#[proc_macro_derive(DispatchDeserialize, attributes(dispatch))]
pub fn derive_dispatched(item: TokenStream) -> TokenStream {
    let derive_item = parse_macro_input!(item as DeriveInput);
    let Data::Enum(derive_enum) = &derive_item.data else {
        return Error::new_spanned(derive_item.ident, "Dispatched must be an enum")
            .to_compile_error()
            .into();
    };

    let enum_name = &derive_item.ident;

    let mut lifetime_generics = derive_item.generics.clone();
    let (impl_generics, ty_generics, where_clause) =
        deserialize_impl_generics(&derive_item.generics, &mut lifetime_generics);

    let dispatch_ident = Ident::new("dispatch", Span::call_site());
    let mut inline_variant_test = None;

    let mut type_tests = Vec::new();

    for variant in &derive_enum.variants {
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

        let mut inlinable = false;
        let mut inlinable_func = None;
        for attr in &variant.attrs {
            if !attr.path().is_ident(&dispatch_ident) {
                continue;
            }
            let directive: DispatchDirective = match attr.parse_args() {
                Ok(directive) => directive,
                Err(err) => return err.into_compile_error().into(),
            };

            match directive {
                DispatchDirective::Inlinable(func) => {
                    inlinable = true;
                    inlinable_func =
                        Some(func.unwrap_or_else(
                            || parse_quote! { ::serde::de::Deserialize::deserialize },
                        ));
                }
                DispatchDirective::Rename(new_name) => {
                    identifier_name = new_name;
                }
            }
        }

        if inlinable {
            if inline_variant_test.is_some() {
                return Error::new_spanned(variant_name, "cannot have multiple inlinable variants")
                    .into_compile_error()
                    .into();
            }
            inline_variant_test = Some(quote! {
                let inline_variant_error = match #inlinable_func(value.clone()) {
                    Ok(result) => return Ok(Self::#variant_name(result)),
                    Err(err) => err,
                };
            });
        }

        type_tests.push(quote! {
            #identifier_name => Ok(Self::#variant_name(::serde::de::Deserialize::deserialize(
                ::serde_json::value::Value::Object(obj)
            ).map_err(|err| ::serde::de::Error::custom(err))?)),
        })
    }

    let not_an_object_error = if inline_variant_test.is_some() {
        quote! { ::serde::de::Error::custom(inline_variant_error) }
    } else {
        quote! {
            ::serde::de::Error::invalid_type(
                ::serde::de::Unexpected::Other("non-map"),
                &"a map",
            )
        }
    };

    let expected = format!("valid type id for {enum_name}");
    From::from(quote! {
        impl #impl_generics ::serde::de::Deserialize<'de> for #enum_name #ty_generics #where_clause {
            fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>
            {
                let mut value = ::serde_json::value::Value::deserialize(deserializer)?;
                #inline_variant_test
                let ::serde_json::value::Value::Object(mut obj) = value else {
                    return Err(#not_an_object_error);
                };
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
                        &#expected
                    )),
                }
            }
        }
    })
}

#[proc_macro_derive(UntaggedDeserialize, attributes(serde))]
pub fn derive_untagged_deserialize(item: TokenStream) -> TokenStream {
    let derive_item = parse_macro_input!(item as DeriveInput);
    let Data::Enum(derive_enum) = &derive_item.data else {
        return Error::new_spanned(
            derive_item.ident,
            "UntaggedDeserialize must be used on an enum",
        )
        .to_compile_error()
        .into();
    };

    let enum_name = &derive_item.ident;

    let mut lifetime_generics = derive_item.generics.clone();
    let (impl_generics, ty_generics, where_clause) =
        deserialize_impl_generics(&derive_item.generics, &mut lifetime_generics);

    let mut variant_deserializers = Vec::new();
    let mut error_indenters = Vec::new();
    let mut error_format = format!("data did not match any variant of untagged enum {enum_name}");

    for (variant_index, variant) in derive_enum.variants.iter().enumerate() {
        let variant_name = &variant.ident;
        let (struct_decl, struct_to_deserialize, restructure) = match &variant.fields {
            Fields::Unnamed(fields) => {
                let fields = &fields.unnamed;
                if fields.is_empty() {
                    return Error::new_spanned(
                        variant_name,
                        "unit variants not allowed in UntaggedDeserialize",
                    )
                    .to_compile_error()
                    .into();
                }
                if fields.len() == 1 {
                    let field_ty = &fields[0].ty;
                    (None, quote! { #field_ty }, quote! { (deserialized) })
                } else {
                    let types = fields.iter().map(|field| &field.ty);
                    let restructure_args = (0..fields.len())
                        .map(Index::from)
                        .map(|i| quote! { deserialized.#i });
                    (
                        None,
                        quote! { (#(#types,)*) },
                        quote! { (#(#restructure_args,)*) },
                    )
                }
            }
            Fields::Named(fields) => {
                let fields = &fields.named;
                let mut generic_references = GenericReferencesCollector::default();
                for field in fields {
                    generic_references.visit_type(&field.ty);
                }
                // panic!("Generic references: {generic_references:#?}");

                let mut proxy_generics = Vec::new();
                proxy_generics.extend(
                    derive_item
                        .generics
                        .lifetimes()
                        .filter(|lifetime| {
                            generic_references.lifetimes.contains(&lifetime.lifetime)
                        })
                        .map(|lifetime| GenericParam::Lifetime(lifetime.clone())),
                );
                proxy_generics.extend(
                    derive_item
                        .generics
                        .type_params()
                        .filter(|type_param| generic_references.idents.contains(&type_param.ident))
                        .map(|type_param| GenericParam::Type(type_param.clone())),
                );
                proxy_generics.extend(
                    derive_item
                        .generics
                        .const_params()
                        .filter(|const_param| {
                            generic_references.idents.contains(&const_param.ident)
                        })
                        .map(|const_param| GenericParam::Const(const_param.clone())),
                );
                let proxy_generics = if proxy_generics.is_empty() {
                    None
                } else {
                    Some(parse2::<Generics>(quote! { < #(#proxy_generics,)* > }).unwrap())
                };

                let proxy_type = format_ident!("__Proxy_{}", variant_name);
                let proxy_type_args = proxy_generics
                    .as_ref()
                    .map(|generics| generics.split_for_impl().1);
                let restructure_args = fields.iter().map(|field| {
                    let field_name = field.ident.as_ref().unwrap();
                    quote! { #field_name: deserialized.#field_name }
                });
                (
                    Some(
                        quote! { #[derive(::serde::Deserialize)] struct #proxy_type #proxy_generics { #fields } },
                    ),
                    quote! { #proxy_type #proxy_type_args },
                    quote! { { #(#restructure_args,)* } },
                )
            }
            Fields::Unit => {
                return Error::new_spanned(
                    variant_name,
                    "unit variants not allowed in UntaggedDeserialize",
                )
                .to_compile_error()
                .into()
            }
        };

        let error_ident = format_ident!("error_{}", variant_name);
        let maybe_clone_value = if variant_index == derive_enum.variants.len() - 1 {
            quote! { value }
        } else {
            quote! { value.clone() }
        };
        variant_deserializers.push(quote! {
            #struct_decl
            let #error_ident = match <#struct_to_deserialize>::deserialize(#maybe_clone_value) {
                Ok(deserialized) => return Ok(Self::#variant_name #restructure),
                Err(err) => err,
            };
        });
        error_indenters.push(quote! {
            let #error_ident = #error_ident.to_string().replace("\n", "\n    ");
        });
        use std::fmt::Write;
        write!(
            error_format,
            "\n    - tried to deserialize variant {variant_name} but got error: {{{error_ident}}}"
        )
        .unwrap();
    }

    From::from(quote! {
        #[allow(nonstandard_style)]
        impl #impl_generics ::serde::de::Deserialize<'de> for #enum_name #ty_generics #where_clause {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::de::Deserializer<'de> {
                let value = ::serde_json::value::Value::deserialize(deserializer)?;
                #(#variant_deserializers)*
                #(#error_indenters)*
                Err(::serde::de::Error::custom(format!(#error_format)))
            }
        }
    })
}

#[derive(Debug, Default)]
struct GenericReferencesCollector<'ast> {
    pub idents: BTreeSet<&'ast Ident>,
    pub lifetimes: BTreeSet<&'ast Lifetime>,
}

impl<'ast> Visit<'ast> for GenericReferencesCollector<'ast> {
    fn visit_ident(&mut self, i: &'ast Ident) {
        self.idents.insert(i);
    }

    fn visit_lifetime(&mut self, i: &'ast Lifetime) {
        self.lifetimes.insert(i);
    }
}
