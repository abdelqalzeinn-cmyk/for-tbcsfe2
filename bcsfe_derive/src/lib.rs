extern crate proc_macro;

use darling::FromAttributes;
use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, GenericArgument, PathArguments, Type};

#[derive(Debug, Copy, Clone, FromAttributes, Default)]
#[darling(default, attributes(rw))]
struct Opts {
    gvcc: bool,
    min_gv: Option<u32>,
    max_gv: Option<u32>,
    en: Option<bool>,
    jp: Option<bool>,
    kr: Option<bool>,
    tw: Option<bool>,
}

impl Opts {
    fn is_empty(&self) -> bool {
        self.min_gv.is_none()
            && self.max_gv.is_none()
            && self.en.is_none()
            && self.jp.is_none()
            && self.kr.is_none()
            && self.tw.is_none()
    }
}

fn extract_option_type(input: &Type) -> Option<Type> {
    if let Type::Path(type_path) = input {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                        return Some(inner_type.clone());
                    }
                }
            }
        }
    }
    None
}


#[proc_macro_derive(Readable, attributes(rw))]
pub fn readable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let fields = match input.data {
        Data::Struct(ref data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Readable can only be derived for structs with named fields"),
        },
        _ => panic!("Readable can only be derived for structs"),
    };

    let all_empty = fields.iter().all(|f| f.attrs.is_empty());

    let read_field_impls = fields.iter().map(|field| {
        let opts = Opts::from_attributes(&field.attrs).expect("Wrong options");
        let field_name = &field.ident;
        let field_type = &field.ty;

        let min_gv = opts.min_gv.unwrap_or(0);
        let max_gv = opts.max_gv.unwrap_or(u32::MAX);
        let en = opts.en.unwrap_or(true);
        let jp = opts.jp.unwrap_or(true);
        let kr = opts.kr.unwrap_or(true);
        let tw = opts.tw.unwrap_or(true);
        let gvcc = opts.gvcc;

        let inner_type = extract_option_type(field_type);
        
        let line = if gvcc {
            if opts.is_empty() {
                quote! {
                    crate::stream::NewResultCtx::add_context(<#field_type as crate::stream::Readable>::read(reader, args), || format!("read {} for {}", stringify!(#field_name), stringify!(#name)))?
                }
            } else {
                quote! {
                    crate::stream::NewResultCtx::add_context(<#inner_type as crate::stream::Readable>::read(reader, args), || format!("read {} for {}", stringify!(#field_name), stringify!(#name)))?
                }
            }
        } else {
            if opts.is_empty() {
                quote! {
                    crate::stream::NewResultCtx::add_context(<#field_type as crate::stream::Readable>::read(reader, ()), || format!("read {} for {}", stringify!(#field_name), stringify!(#name)))?
                }
            } else {
                quote! {
                    crate::stream::NewResultCtx::add_context(<#inner_type as crate::stream::Readable>::read(reader, ()), || format!("read {} for {}", stringify!(#field_name), stringify!(#name)))?
                }
            }
        };


        if opts.is_empty() {
            quote! {
                #field_name: #line,
            }
        }
        else {
            quote! {
                #field_name: {
                    if args.gv.0 >= #min_gv && args.gv.0 <= #max_gv && {
                        match args.cc {
                            crate::country_code::CountryCode::En => #en,
                            crate::country_code::CountryCode::Jp => #jp,
                            crate::country_code::CountryCode::Kr => #kr,
                            crate::country_code::CountryCode::Tw => #tw,
                        }
                    }
                    {
                        Some(#line)
                    }
                    else {
                        None
                    }
                },
            }
        }
    });

    let expanded = if all_empty {
        quote! {
            impl crate::stream::Readable for #name {
                type Args<'a> = ();
                fn read<R: std::io::Read + std::io::Seek>(
                    reader: &mut R,
                    _args: Self::Args<'_>,
                ) -> crate::stream::StreamResult<Self> {
                    Ok(Self {
                        #(#read_field_impls)*
                    })
                }
            }
        }
    } else {
        quote! {
            impl crate::stream::Readable for #name {
                type Args<'a> = crate::save::GVCC;
                fn read<R: std::io::Read + std::io::Seek>(
                    reader: &mut R,
                    args: Self::Args<'_>,
                ) -> crate::stream::StreamResult<Self> {
                    Ok(Self {
                        #(#read_field_impls)*
                    })
                }
            }
        }
    };

    TokenStream::from(expanded)
}
#[proc_macro_derive(Writable, attributes(rw))]
pub fn writeable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    

    let fields = match input.data {
        Data::Struct(ref data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Writeable can only be derived for structs with named fields"),
        },
        _ => panic!("Writeable can only be derived for structs"),
    };

    
    let all_empty = fields.iter().all(|f| f.attrs.is_empty());

    let write_field_impls = fields.iter().map(|field| {
        let opts = Opts::from_attributes(&field.attrs).expect("Wrong options");
        let field_name = &field.ident;
        let field_type = &field.ty;

        
        let min_gv = opts.min_gv.unwrap_or(0);
        let max_gv = opts.max_gv.unwrap_or(u32::MAX);
        let en = opts.en.unwrap_or(true);
        let jp = opts.jp.unwrap_or(true);
        let kr = opts.kr.unwrap_or(true);
        let tw = opts.tw.unwrap_or(true);
        let gvcc = opts.gvcc;

        let inner_type = extract_option_type(field_type);

        let line = if gvcc {
            if opts.is_empty() {
                quote! {
                    crate::stream::NewResultCtx::add_context(self.#field_name.write(writer, args), || format!("write {} for {}", stringify!(#field_name), stringify!(#name)))?;
                }
            }
            else {
                quote! {
                    if let Some(ref val) = self.#field_name {
                        crate::stream::NewResultCtx::add_context(val.write(writer, args), || format!("write {} for {}", stringify!(#field_name), stringify!(#name)))?;
                    }
                    else {
                        crate::stream::NewResultCtx::add_context(<#inner_type>::default().write(writer, args), || format!("write {} for {}", stringify!(#field_name), stringify!(#name)))?;
                    }
                }
            }
        }
        else {
            if opts.is_empty() {
                quote! {
                    crate::stream::NewResultCtx::add_context(self.#field_name.write(writer, ()), || format!("write {} for {}", stringify!(#field_name), stringify!(#name)))?;
                }
            }
            else {
                quote! {
                    if let Some(ref val) = self.#field_name {
                        crate::stream::NewResultCtx::add_context(val.write(writer, ()), || format!("write {} for {}", stringify!(#field_name), stringify!(#name)))?;
                    }
                    else {
                        crate::stream::NewResultCtx::add_context(<#inner_type>::default().write(writer, ()), || format!("write {} for {}", stringify!(#field_name), stringify!(#name)))?;
                    }
                }
            }
        };

        if opts.is_empty() {
            quote! {
                #line
            }
        }
        else {
            quote! {
                if args.gv.0 >= #min_gv && args.gv.0 <= #max_gv && {
                    match args.cc {
                        crate::country_code::CountryCode::En => #en,
                        crate::country_code::CountryCode::Jp => #jp,
                        crate::country_code::CountryCode::Kr => #kr,
                        crate::country_code::CountryCode::Tw => #tw,
                    }
                }
                {
                    #line
                }
            }
        }
    });

    let expanded = if all_empty {
        quote! {
            impl crate::stream::Writable for #name {
                type Args<'a> = ();
                fn write<W: std::io::Write + std::io::Seek>(
                    &self,
                    writer: &mut W,
                    _args: Self::Args<'_>,
                ) -> crate::stream::StreamResult<()> {
                    #(#write_field_impls)*
                    Ok(())
                }
            }
        }
    }
    else {
        quote! {
            impl crate::stream::Writable for #name {
                type Args<'a> = crate::save::GVCC;
                fn write<W: std::io::Write + std::io::Seek>(
                    &self,
                    writer: &mut W,
                    args: Self::Args<'_>,
                ) -> crate::stream::StreamResult<()> {
                    #(#write_field_impls)*
                    Ok(())
                }
            }
        }
    };

    TokenStream::from(expanded)
}
