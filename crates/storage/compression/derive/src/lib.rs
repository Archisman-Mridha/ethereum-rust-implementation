#![allow(non_snake_case, unused)]

use syn::{Data, PathSegment};

type FieldName= String;
type FieldType= String;
type IsFieldCompressable= bool;
type IsFieldTypeFixedSizeBytes= bool;

type StructFieldDetails = (FieldName, FieldType, IsFieldCompressable, IsFieldTypeFixedSizeBytes);

enum Field {
  StructField(StructFieldDetails),
  EnumVariant(String),
  EnumUnnamedField((FieldType, IsFieldTypeFixedSizeBytes))
}

type Fields= Vec<Field>;

// Extract fields from the given struct / enum.
fn getFields(data: &Data) -> Fields {
  let mut fields= vec!{ };

  match data {
    Data::Union(data) => unimplemented!( ),

    Data::Struct(data) => match data.fields {
      // struct Logger;
      syn::Fields::Unit => unimplemented!( ),

      // struct Person(String);
      syn::Fields::Unnamed(ref data) => pushToFields(&data.unnamed[0], &mut fields, false),

      // struct Person { name: String }
      syn::Fields::Named(ref data) =>
        &data.named.iter( ).map(|field| pushToFields(field, &mut fields, false))
    },

    Data::Enum(data) => {
      for variant in &data.variants {
        fields.push(Field::EnumVariant(variant.ident.to_string( )));
        match &variant.fields {
          // enum Mammal { Person }
          syn::Fields::Unit => ( ),

          // enum Mammal { Person(String) }
          syn::Fields::Unnamed(ref data) => pushToFields(&data.unnamed[0], &mut fields, false),

          // enum Mammal { Person { name: String } }
          syn::Fields::Named(_) => panic!("Not allowed to have Enum Variants with named fields.")
        }
      }
    }
  }

  fields
}

fn pushToFields(field: &syn::Field, fields: &mut Fields, isEnumField: bool) {
  if let syn::Type::Path(ref typePath)= field.ty {
    let typePathSegments= &typePath.path.segments;
    if typePathSegments.is_empty( ) { return }

    let mut fieldTypeAsString= String::new( );
    let mut isFieldTypeFixedSizeBytes: IsFieldCompressable= false;

    for (i, typePathSegment) in typePathSegments.iter( ).enumerate( ) {
      fieldTypeAsString.push_str(&typePathSegment.ident.to_string( ));

      if i < (typePathSegments.len( ) - 1) {
        fieldTypeAsString.push_str("::");
      }

      isFieldTypeFixedSizeBytes= useMethodsForFixedSizeBytes(&fieldTypeAsString, typePathSegment);
    }

    if isEnumField {
      fields.push(Field::EnumUnnamedField((fieldTypeAsString, isFieldTypeFixedSizeBytes)));}

    else {
      fields.push(Field::StructField((
        field.ident.as_ref( ).map(|i| i.to_string( )).unwrap_or_default( ),
        fieldTypeAsString,
        todo!( ),
        isFieldTypeFixedSizeBytes,
      )));
    }
  }
}

// Returns true if the given field is of type fixed size bytes.
fn useMethodsForFixedSizeBytes(fieldTypeAsString: &str, typePathSegment: &PathSegment) -> bool {
  if fieldTypeAsString == "Vec" || fieldTypeAsString == "Option" {
    if let syn::PathArguments::AngleBracketed(ref typePathSegmentArgs)= typePathSegment.arguments {
      if let Some(syn::GenericArgument::Type(syn::Type::Path(concreteTypePath))) = typePathSegmentArgs.args.last( ) {
        if let (Some(concreteType),                     1)=
               (concreteTypePath.path.segments.first( ), concreteTypePath.path.segments.len( ))
        {
          let isFixedSizeBytesType= ["B256", "Address", "Bloom", "TxHash", "BlockHash"]
                                      .contains(&concreteType.ident.to_string( ).as_str( ));
          if isFixedSizeBytesType { return true }
        }
      }
    }
  }

  false
}