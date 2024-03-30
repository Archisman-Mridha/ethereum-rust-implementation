#![allow(non_snake_case)]

use syn::Data;

type FieldName= String;
type FieldType= String;
type FieldCompressable= bool;

type StructFieldDescriptor = (FieldName, FieldType, FieldCompressable, UseAlternative);

enum Field {
  StructField(StructFieldDescriptor)
}

fn getFields(data: &Data) { }