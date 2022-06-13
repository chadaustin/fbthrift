// @generated by Thrift for src/module.thrift
// This file is probably not the place you want to edit!

//! Thrift type definitions for `module`.

#![allow(clippy::redundant_closure)]


pub type SetWithAdapter = ::std::collections::BTreeSet<::std::string::String>;

pub type StringWithAdapter = ::std::string::String;

pub type ListWithElemAdapter = ::std::vec::Vec<crate::types::StringWithAdapter>;

pub type ListWithElemAdapter_withAdapter = crate::types::ListWithElemAdapter;

pub type MyI64 = ::std::primitive::i64;

pub type DoubleTypedefI64 = crate::types::MyI64;

pub type MyI32 = ::std::primitive::i32;

pub type FooWithAdapter = crate::types::Foo;

pub type StructWithAdapter = crate::types::Bar;

pub type UnionWithAdapter = crate::types::Baz;

pub type AdaptedA = crate::types::A;

#[derive(Clone, PartialEq)]
pub struct Foo {
    pub intField: ::std::primitive::i32,
    pub optionalIntField: ::std::option::Option<::std::primitive::i32>,
    pub intFieldWithDefault: ::std::primitive::i32,
    pub setField: crate::types::SetWithAdapter,
    pub optionalSetField: ::std::option::Option<crate::types::SetWithAdapter>,
    pub mapField: ::std::collections::BTreeMap<::std::string::String, crate::types::ListWithElemAdapter_withAdapter>,
    pub optionalMapField: ::std::option::Option<::std::collections::BTreeMap<::std::string::String, crate::types::ListWithElemAdapter_withAdapter>>,
    pub binaryField: ::std::vec::Vec<::std::primitive::u8>,
    pub longField: crate::types::MyI64,
    pub adaptedLongField: crate::types::MyI64,
    pub doubleAdaptedField: crate::types::DoubleTypedefI64,
    // This field forces `..Default::default()` when instantiating this
    // struct, to make code future-proof against new fields added later to
    // the definition in Thrift. If you don't want this, add the annotation
    // `(rust.exhaustive)` to the Thrift struct to eliminate this field.
    #[doc(hidden)]
    pub _dot_dot_Default_default: self::dot_dot::OtherFields,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Baz {
    intField(::std::primitive::i32),
    setField(crate::types::SetWithAdapter),
    mapField(::std::collections::BTreeMap<::std::string::String, crate::types::ListWithElemAdapter_withAdapter>),
    binaryField(::std::vec::Vec<::std::primitive::u8>),
    longField(crate::types::MyI64),
    UnknownField(::std::primitive::i32),
}

#[derive(Clone, PartialEq)]
pub struct Bar {
    pub structField: crate::types::Foo,
    pub optionalStructField: ::std::option::Option<crate::types::Foo>,
    pub structListField: ::std::vec::Vec<crate::types::FooWithAdapter>,
    pub optionalStructListField: ::std::option::Option<::std::vec::Vec<crate::types::FooWithAdapter>>,
    pub unionField: crate::types::Baz,
    pub optionalUnionField: ::std::option::Option<crate::types::Baz>,
    // This field forces `..Default::default()` when instantiating this
    // struct, to make code future-proof against new fields added later to
    // the definition in Thrift. If you don't want this, add the annotation
    // `(rust.exhaustive)` to the Thrift struct to eliminate this field.
    #[doc(hidden)]
    pub _dot_dot_Default_default: self::dot_dot::OtherFields,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StructWithFieldAdapter {
    pub field: ::std::primitive::i32,
    pub shared_field: ::std::primitive::i32,
    pub opt_shared_field: ::std::option::Option<::std::primitive::i32>,
    pub opt_boxed_field: ::std::option::Option<::std::boxed::Box<::std::primitive::i32>>,
    // This field forces `..Default::default()` when instantiating this
    // struct, to make code future-proof against new fields added later to
    // the definition in Thrift. If you don't want this, add the annotation
    // `(rust.exhaustive)` to the Thrift struct to eliminate this field.
    #[doc(hidden)]
    pub _dot_dot_Default_default: self::dot_dot::OtherFields,
}

#[derive(Clone, PartialEq)]
pub struct B {
    pub a: crate::types::AdaptedA,
    // This field forces `..Default::default()` when instantiating this
    // struct, to make code future-proof against new fields added later to
    // the definition in Thrift. If you don't want this, add the annotation
    // `(rust.exhaustive)` to the Thrift struct to eliminate this field.
    #[doc(hidden)]
    pub _dot_dot_Default_default: self::dot_dot::OtherFields,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct A {
    // This field forces `..Default::default()` when instantiating this
    // struct, to make code future-proof against new fields added later to
    // the definition in Thrift. If you don't want this, add the annotation
    // `(rust.exhaustive)` to the Thrift struct to eliminate this field.
    #[doc(hidden)]
    pub _dot_dot_Default_default: self::dot_dot::OtherFields,
}












#[allow(clippy::derivable_impls)]
impl ::std::default::Default for self::Foo {
    fn default() -> Self {
        Self {
            intField: ::std::default::Default::default(),
            optionalIntField: ::std::option::Option::None,
            intFieldWithDefault: 13,
            setField: ::std::default::Default::default(),
            optionalSetField: ::std::option::Option::None,
            mapField: ::std::default::Default::default(),
            optionalMapField: ::std::option::Option::None,
            binaryField: ::std::default::Default::default(),
            longField: ::std::default::Default::default(),
            adaptedLongField: ::std::default::Default::default(),
            doubleAdaptedField: ::std::default::Default::default(),
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        }
    }
}

impl ::std::fmt::Debug for self::Foo {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter
            .debug_struct("Foo")
            .field("intField", &self.intField)
            .field("optionalIntField", &self.optionalIntField)
            .field("intFieldWithDefault", &self.intFieldWithDefault)
            .field("setField", &self.setField)
            .field("optionalSetField", &self.optionalSetField)
            .field("mapField", &self.mapField)
            .field("optionalMapField", &self.optionalMapField)
            .field("binaryField", &self.binaryField)
            .field("longField", &self.longField)
            .field("adaptedLongField", &self.adaptedLongField)
            .field("doubleAdaptedField", &self.doubleAdaptedField)
            .finish()
    }
}

unsafe impl ::std::marker::Send for self::Foo {}
unsafe impl ::std::marker::Sync for self::Foo {}

impl ::fbthrift::GetTType for self::Foo {
    const TTYPE: ::fbthrift::TType = ::fbthrift::TType::Struct;
}

impl<P> ::fbthrift::Serialize<P> for self::Foo
where
    P: ::fbthrift::ProtocolWriter,
{
    fn write(&self, p: &mut P) {
        p.write_struct_begin("Foo");
        p.write_field_begin("intField", ::fbthrift::TType::I32, 1);
        ::fbthrift::Serialize::write(&self.intField, p);
        p.write_field_end();
        if let ::std::option::Option::Some(some) = &self.optionalIntField {
            p.write_field_begin("optionalIntField", ::fbthrift::TType::I32, 2);
            ::fbthrift::Serialize::write(some, p);
            p.write_field_end();
        }
        p.write_field_begin("intFieldWithDefault", ::fbthrift::TType::I32, 3);
        ::fbthrift::Serialize::write(&self.intFieldWithDefault, p);
        p.write_field_end();
        p.write_field_begin("setField", ::fbthrift::TType::Set, 4);
        ::fbthrift::Serialize::write(&self.setField, p);
        p.write_field_end();
        if let ::std::option::Option::Some(some) = &self.optionalSetField {
            p.write_field_begin("optionalSetField", ::fbthrift::TType::Set, 5);
            ::fbthrift::Serialize::write(some, p);
            p.write_field_end();
        }
        p.write_field_begin("mapField", ::fbthrift::TType::Map, 6);
        ::fbthrift::Serialize::write(&self.mapField, p);
        p.write_field_end();
        if let ::std::option::Option::Some(some) = &self.optionalMapField {
            p.write_field_begin("optionalMapField", ::fbthrift::TType::Map, 7);
            ::fbthrift::Serialize::write(some, p);
            p.write_field_end();
        }
        p.write_field_begin("binaryField", ::fbthrift::TType::String, 8);
        ::fbthrift::Serialize::write(&self.binaryField, p);
        p.write_field_end();
        p.write_field_begin("longField", ::fbthrift::TType::I64, 9);
        ::fbthrift::Serialize::write(&self.longField, p);
        p.write_field_end();
        p.write_field_begin("adaptedLongField", ::fbthrift::TType::I64, 10);
        ::fbthrift::Serialize::write(&self.adaptedLongField, p);
        p.write_field_end();
        p.write_field_begin("doubleAdaptedField", ::fbthrift::TType::I64, 11);
        ::fbthrift::Serialize::write(&self.doubleAdaptedField, p);
        p.write_field_end();
        p.write_field_stop();
        p.write_struct_end();
    }
}

impl<P> ::fbthrift::Deserialize<P> for self::Foo
where
    P: ::fbthrift::ProtocolReader,
{
    fn read(p: &mut P) -> ::anyhow::Result<Self> {
        static FIELDS: &[::fbthrift::Field] = &[
            ::fbthrift::Field::new("adaptedLongField", ::fbthrift::TType::I64, 10),
            ::fbthrift::Field::new("binaryField", ::fbthrift::TType::String, 8),
            ::fbthrift::Field::new("doubleAdaptedField", ::fbthrift::TType::I64, 11),
            ::fbthrift::Field::new("intField", ::fbthrift::TType::I32, 1),
            ::fbthrift::Field::new("intFieldWithDefault", ::fbthrift::TType::I32, 3),
            ::fbthrift::Field::new("longField", ::fbthrift::TType::I64, 9),
            ::fbthrift::Field::new("mapField", ::fbthrift::TType::Map, 6),
            ::fbthrift::Field::new("optionalIntField", ::fbthrift::TType::I32, 2),
            ::fbthrift::Field::new("optionalMapField", ::fbthrift::TType::Map, 7),
            ::fbthrift::Field::new("optionalSetField", ::fbthrift::TType::Set, 5),
            ::fbthrift::Field::new("setField", ::fbthrift::TType::Set, 4),
        ];
        let mut field_intField = ::std::option::Option::None;
        let mut field_optionalIntField = ::std::option::Option::None;
        let mut field_intFieldWithDefault = ::std::option::Option::None;
        let mut field_setField = ::std::option::Option::None;
        let mut field_optionalSetField = ::std::option::Option::None;
        let mut field_mapField = ::std::option::Option::None;
        let mut field_optionalMapField = ::std::option::Option::None;
        let mut field_binaryField = ::std::option::Option::None;
        let mut field_longField = ::std::option::Option::None;
        let mut field_adaptedLongField = ::std::option::Option::None;
        let mut field_doubleAdaptedField = ::std::option::Option::None;
        let _ = p.read_struct_begin(|_| ())?;
        loop {
            let (_, fty, fid) = p.read_field_begin(|_| (), FIELDS)?;
            match (fty, fid as ::std::primitive::i32) {
                (::fbthrift::TType::Stop, _) => break,
                (::fbthrift::TType::I32, 1) => field_intField = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (::fbthrift::TType::I32, 2) => field_optionalIntField = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (::fbthrift::TType::I32, 3) => field_intFieldWithDefault = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (::fbthrift::TType::Set, 4) => field_setField = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (::fbthrift::TType::Set, 5) => field_optionalSetField = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (::fbthrift::TType::Map, 6) => field_mapField = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (::fbthrift::TType::Map, 7) => field_optionalMapField = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (::fbthrift::TType::String, 8) => field_binaryField = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (::fbthrift::TType::I64, 9) => field_longField = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (::fbthrift::TType::I64, 10) => field_adaptedLongField = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (::fbthrift::TType::I64, 11) => field_doubleAdaptedField = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (fty, _) => p.skip(fty)?,
            }
            p.read_field_end()?;
        }
        p.read_struct_end()?;
        ::std::result::Result::Ok(Self {
            intField: field_intField.unwrap_or_default(),
            optionalIntField: field_optionalIntField,
            intFieldWithDefault: field_intFieldWithDefault.unwrap_or_else(|| 13),
            setField: field_setField.unwrap_or_default(),
            optionalSetField: field_optionalSetField,
            mapField: field_mapField.unwrap_or_default(),
            optionalMapField: field_optionalMapField,
            binaryField: field_binaryField.unwrap_or_default(),
            longField: field_longField.unwrap_or_default(),
            adaptedLongField: field_adaptedLongField.unwrap_or_default(),
            doubleAdaptedField: field_doubleAdaptedField.unwrap_or_default(),
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        })
    }
}



impl ::std::default::Default for Baz {
    fn default() -> Self {
        Self::UnknownField(-1)
    }
}

impl ::fbthrift::GetTType for Baz {
    const TTYPE: ::fbthrift::TType = ::fbthrift::TType::Struct;
}

impl<P> ::fbthrift::Serialize<P> for Baz
where
    P: ::fbthrift::ProtocolWriter,
{
    fn write(&self, p: &mut P) {
        p.write_struct_begin("Baz");
        match self {
            Baz::intField(inner) => {
                p.write_field_begin("intField", ::fbthrift::TType::I32, 1);
                ::fbthrift::Serialize::write(inner, p);
                p.write_field_end();
            }
            Baz::setField(inner) => {
                p.write_field_begin("setField", ::fbthrift::TType::Set, 4);
                ::fbthrift::Serialize::write(inner, p);
                p.write_field_end();
            }
            Baz::mapField(inner) => {
                p.write_field_begin("mapField", ::fbthrift::TType::Map, 6);
                ::fbthrift::Serialize::write(inner, p);
                p.write_field_end();
            }
            Baz::binaryField(inner) => {
                p.write_field_begin("binaryField", ::fbthrift::TType::String, 8);
                ::fbthrift::Serialize::write(inner, p);
                p.write_field_end();
            }
            Baz::longField(inner) => {
                p.write_field_begin("longField", ::fbthrift::TType::I64, 9);
                ::fbthrift::Serialize::write(inner, p);
                p.write_field_end();
            }
            Baz::UnknownField(_) => {}
        }
        p.write_field_stop();
        p.write_struct_end();
    }
}

impl<P> ::fbthrift::Deserialize<P> for Baz
where
    P: ::fbthrift::ProtocolReader,
{
    fn read(p: &mut P) -> ::anyhow::Result<Self> {
        static FIELDS: &[::fbthrift::Field] = &[
            ::fbthrift::Field::new("binaryField", ::fbthrift::TType::String, 8),
            ::fbthrift::Field::new("intField", ::fbthrift::TType::I32, 1),
            ::fbthrift::Field::new("longField", ::fbthrift::TType::I64, 9),
            ::fbthrift::Field::new("mapField", ::fbthrift::TType::Map, 6),
            ::fbthrift::Field::new("setField", ::fbthrift::TType::Set, 4),
        ];
        let _ = p.read_struct_begin(|_| ())?;
        let mut once = false;
        let mut alt = ::std::option::Option::None;
        loop {
            let (_, fty, fid) = p.read_field_begin(|_| (), FIELDS)?;
            match (fty, fid as ::std::primitive::i32, once) {
                (::fbthrift::TType::Stop, _, _) => break,
                (::fbthrift::TType::I32, 1, false) => {
                    once = true;
                    alt = ::std::option::Option::Some(Baz::intField(::fbthrift::Deserialize::read(p)?));
                }
                (::fbthrift::TType::Set, 4, false) => {
                    once = true;
                    alt = ::std::option::Option::Some(Baz::setField(::fbthrift::Deserialize::read(p)?));
                }
                (::fbthrift::TType::Map, 6, false) => {
                    once = true;
                    alt = ::std::option::Option::Some(Baz::mapField(::fbthrift::Deserialize::read(p)?));
                }
                (::fbthrift::TType::String, 8, false) => {
                    once = true;
                    alt = ::std::option::Option::Some(Baz::binaryField(::fbthrift::Deserialize::read(p)?));
                }
                (::fbthrift::TType::I64, 9, false) => {
                    once = true;
                    alt = ::std::option::Option::Some(Baz::longField(::fbthrift::Deserialize::read(p)?));
                }
                (fty, _, false) => p.skip(fty)?,
                (badty, badid, true) => return ::std::result::Result::Err(::std::convert::From::from(::fbthrift::ApplicationException::new(
                    ::fbthrift::ApplicationExceptionErrorCode::ProtocolError,
                    format!(
                        "unwanted extra union {} field ty {:?} id {}",
                        "Baz",
                        badty,
                        badid,
                    ),
                ))),
            }
            p.read_field_end()?;
        }
        p.read_struct_end()?;
        ::std::result::Result::Ok(alt.unwrap_or_default())
    }
}

#[allow(clippy::derivable_impls)]
impl ::std::default::Default for self::Bar {
    fn default() -> Self {
        Self {
            structField: ::std::default::Default::default(),
            optionalStructField: ::std::option::Option::None,
            structListField: ::std::default::Default::default(),
            optionalStructListField: ::std::option::Option::None,
            unionField: ::std::default::Default::default(),
            optionalUnionField: ::std::option::Option::None,
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        }
    }
}

impl ::std::fmt::Debug for self::Bar {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter
            .debug_struct("Bar")
            .field("structField", &self.structField)
            .field("optionalStructField", &self.optionalStructField)
            .field("structListField", &self.structListField)
            .field("optionalStructListField", &self.optionalStructListField)
            .field("unionField", &self.unionField)
            .field("optionalUnionField", &self.optionalUnionField)
            .finish()
    }
}

unsafe impl ::std::marker::Send for self::Bar {}
unsafe impl ::std::marker::Sync for self::Bar {}

impl ::fbthrift::GetTType for self::Bar {
    const TTYPE: ::fbthrift::TType = ::fbthrift::TType::Struct;
}

impl<P> ::fbthrift::Serialize<P> for self::Bar
where
    P: ::fbthrift::ProtocolWriter,
{
    fn write(&self, p: &mut P) {
        p.write_struct_begin("Bar");
        p.write_field_begin("structField", ::fbthrift::TType::Struct, 1);
        ::fbthrift::Serialize::write(&self.structField, p);
        p.write_field_end();
        if let ::std::option::Option::Some(some) = &self.optionalStructField {
            p.write_field_begin("optionalStructField", ::fbthrift::TType::Struct, 2);
            ::fbthrift::Serialize::write(some, p);
            p.write_field_end();
        }
        p.write_field_begin("structListField", ::fbthrift::TType::List, 3);
        ::fbthrift::Serialize::write(&self.structListField, p);
        p.write_field_end();
        if let ::std::option::Option::Some(some) = &self.optionalStructListField {
            p.write_field_begin("optionalStructListField", ::fbthrift::TType::List, 4);
            ::fbthrift::Serialize::write(some, p);
            p.write_field_end();
        }
        p.write_field_begin("unionField", ::fbthrift::TType::Struct, 5);
        ::fbthrift::Serialize::write(&self.unionField, p);
        p.write_field_end();
        if let ::std::option::Option::Some(some) = &self.optionalUnionField {
            p.write_field_begin("optionalUnionField", ::fbthrift::TType::Struct, 6);
            ::fbthrift::Serialize::write(some, p);
            p.write_field_end();
        }
        p.write_field_stop();
        p.write_struct_end();
    }
}

impl<P> ::fbthrift::Deserialize<P> for self::Bar
where
    P: ::fbthrift::ProtocolReader,
{
    fn read(p: &mut P) -> ::anyhow::Result<Self> {
        static FIELDS: &[::fbthrift::Field] = &[
            ::fbthrift::Field::new("optionalStructField", ::fbthrift::TType::Struct, 2),
            ::fbthrift::Field::new("optionalStructListField", ::fbthrift::TType::List, 4),
            ::fbthrift::Field::new("optionalUnionField", ::fbthrift::TType::Struct, 6),
            ::fbthrift::Field::new("structField", ::fbthrift::TType::Struct, 1),
            ::fbthrift::Field::new("structListField", ::fbthrift::TType::List, 3),
            ::fbthrift::Field::new("unionField", ::fbthrift::TType::Struct, 5),
        ];
        let mut field_structField = ::std::option::Option::None;
        let mut field_optionalStructField = ::std::option::Option::None;
        let mut field_structListField = ::std::option::Option::None;
        let mut field_optionalStructListField = ::std::option::Option::None;
        let mut field_unionField = ::std::option::Option::None;
        let mut field_optionalUnionField = ::std::option::Option::None;
        let _ = p.read_struct_begin(|_| ())?;
        loop {
            let (_, fty, fid) = p.read_field_begin(|_| (), FIELDS)?;
            match (fty, fid as ::std::primitive::i32) {
                (::fbthrift::TType::Stop, _) => break,
                (::fbthrift::TType::Struct, 1) => field_structField = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (::fbthrift::TType::Struct, 2) => field_optionalStructField = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (::fbthrift::TType::List, 3) => field_structListField = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (::fbthrift::TType::List, 4) => field_optionalStructListField = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (::fbthrift::TType::Struct, 5) => field_unionField = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (::fbthrift::TType::Struct, 6) => field_optionalUnionField = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (fty, _) => p.skip(fty)?,
            }
            p.read_field_end()?;
        }
        p.read_struct_end()?;
        ::std::result::Result::Ok(Self {
            structField: field_structField.unwrap_or_default(),
            optionalStructField: field_optionalStructField,
            structListField: field_structListField.unwrap_or_default(),
            optionalStructListField: field_optionalStructListField,
            unionField: field_unionField.unwrap_or_default(),
            optionalUnionField: field_optionalUnionField,
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        })
    }
}


#[allow(clippy::derivable_impls)]
impl ::std::default::Default for self::StructWithFieldAdapter {
    fn default() -> Self {
        Self {
            field: ::std::default::Default::default(),
            shared_field: ::std::default::Default::default(),
            opt_shared_field: ::std::option::Option::None,
            opt_boxed_field: ::std::option::Option::None,
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        }
    }
}

impl ::std::fmt::Debug for self::StructWithFieldAdapter {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter
            .debug_struct("StructWithFieldAdapter")
            .field("field", &self.field)
            .field("shared_field", &self.shared_field)
            .field("opt_shared_field", &self.opt_shared_field)
            .field("opt_boxed_field", &self.opt_boxed_field)
            .finish()
    }
}

unsafe impl ::std::marker::Send for self::StructWithFieldAdapter {}
unsafe impl ::std::marker::Sync for self::StructWithFieldAdapter {}

impl ::fbthrift::GetTType for self::StructWithFieldAdapter {
    const TTYPE: ::fbthrift::TType = ::fbthrift::TType::Struct;
}

impl<P> ::fbthrift::Serialize<P> for self::StructWithFieldAdapter
where
    P: ::fbthrift::ProtocolWriter,
{
    fn write(&self, p: &mut P) {
        p.write_struct_begin("StructWithFieldAdapter");
        p.write_field_begin("field", ::fbthrift::TType::I32, 1);
        ::fbthrift::Serialize::write(&self.field, p);
        p.write_field_end();
        p.write_field_begin("shared_field", ::fbthrift::TType::I32, 2);
        ::fbthrift::Serialize::write(&self.shared_field, p);
        p.write_field_end();
        if let ::std::option::Option::Some(some) = &self.opt_shared_field {
            p.write_field_begin("opt_shared_field", ::fbthrift::TType::I32, 3);
            ::fbthrift::Serialize::write(some, p);
            p.write_field_end();
        }
        if let ::std::option::Option::Some(some) = &self.opt_boxed_field {
            p.write_field_begin("opt_boxed_field", ::fbthrift::TType::I32, 4);
            ::fbthrift::Serialize::write(some, p);
            p.write_field_end();
        }
        p.write_field_stop();
        p.write_struct_end();
    }
}

impl<P> ::fbthrift::Deserialize<P> for self::StructWithFieldAdapter
where
    P: ::fbthrift::ProtocolReader,
{
    fn read(p: &mut P) -> ::anyhow::Result<Self> {
        static FIELDS: &[::fbthrift::Field] = &[
            ::fbthrift::Field::new("field", ::fbthrift::TType::I32, 1),
            ::fbthrift::Field::new("opt_boxed_field", ::fbthrift::TType::I32, 4),
            ::fbthrift::Field::new("opt_shared_field", ::fbthrift::TType::I32, 3),
            ::fbthrift::Field::new("shared_field", ::fbthrift::TType::I32, 2),
        ];
        let mut field_field = ::std::option::Option::None;
        let mut field_shared_field = ::std::option::Option::None;
        let mut field_opt_shared_field = ::std::option::Option::None;
        let mut field_opt_boxed_field = ::std::option::Option::None;
        let _ = p.read_struct_begin(|_| ())?;
        loop {
            let (_, fty, fid) = p.read_field_begin(|_| (), FIELDS)?;
            match (fty, fid as ::std::primitive::i32) {
                (::fbthrift::TType::Stop, _) => break,
                (::fbthrift::TType::I32, 1) => field_field = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (::fbthrift::TType::I32, 2) => field_shared_field = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (::fbthrift::TType::I32, 3) => field_opt_shared_field = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (::fbthrift::TType::I32, 4) => field_opt_boxed_field = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (fty, _) => p.skip(fty)?,
            }
            p.read_field_end()?;
        }
        p.read_struct_end()?;
        ::std::result::Result::Ok(Self {
            field: field_field.unwrap_or_default(),
            shared_field: field_shared_field.unwrap_or_default(),
            opt_shared_field: field_opt_shared_field,
            opt_boxed_field: field_opt_boxed_field,
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        })
    }
}


#[allow(clippy::derivable_impls)]
impl ::std::default::Default for self::B {
    fn default() -> Self {
        Self {
            a: ::std::default::Default::default(),
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        }
    }
}

impl ::std::fmt::Debug for self::B {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter
            .debug_struct("B")
            .field("a", &self.a)
            .finish()
    }
}

unsafe impl ::std::marker::Send for self::B {}
unsafe impl ::std::marker::Sync for self::B {}

impl ::fbthrift::GetTType for self::B {
    const TTYPE: ::fbthrift::TType = ::fbthrift::TType::Struct;
}

impl<P> ::fbthrift::Serialize<P> for self::B
where
    P: ::fbthrift::ProtocolWriter,
{
    fn write(&self, p: &mut P) {
        p.write_struct_begin("B");
        p.write_field_begin("a", ::fbthrift::TType::Struct, 1);
        ::fbthrift::Serialize::write(&self.a, p);
        p.write_field_end();
        p.write_field_stop();
        p.write_struct_end();
    }
}

impl<P> ::fbthrift::Deserialize<P> for self::B
where
    P: ::fbthrift::ProtocolReader,
{
    fn read(p: &mut P) -> ::anyhow::Result<Self> {
        static FIELDS: &[::fbthrift::Field] = &[
            ::fbthrift::Field::new("a", ::fbthrift::TType::Struct, 1),
        ];
        let mut field_a = ::std::option::Option::None;
        let _ = p.read_struct_begin(|_| ())?;
        loop {
            let (_, fty, fid) = p.read_field_begin(|_| (), FIELDS)?;
            match (fty, fid as ::std::primitive::i32) {
                (::fbthrift::TType::Stop, _) => break,
                (::fbthrift::TType::Struct, 1) => field_a = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (fty, _) => p.skip(fty)?,
            }
            p.read_field_end()?;
        }
        p.read_struct_end()?;
        ::std::result::Result::Ok(Self {
            a: field_a.unwrap_or_default(),
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        })
    }
}


#[allow(clippy::derivable_impls)]
impl ::std::default::Default for self::A {
    fn default() -> Self {
        Self {
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        }
    }
}

impl ::std::fmt::Debug for self::A {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter
            .debug_struct("A")
            .finish()
    }
}

unsafe impl ::std::marker::Send for self::A {}
unsafe impl ::std::marker::Sync for self::A {}

impl ::fbthrift::GetTType for self::A {
    const TTYPE: ::fbthrift::TType = ::fbthrift::TType::Struct;
}

impl<P> ::fbthrift::Serialize<P> for self::A
where
    P: ::fbthrift::ProtocolWriter,
{
    fn write(&self, p: &mut P) {
        p.write_struct_begin("A");
        p.write_field_stop();
        p.write_struct_end();
    }
}

impl<P> ::fbthrift::Deserialize<P> for self::A
where
    P: ::fbthrift::ProtocolReader,
{
    fn read(p: &mut P) -> ::anyhow::Result<Self> {
        static FIELDS: &[::fbthrift::Field] = &[
        ];
        let _ = p.read_struct_begin(|_| ())?;
        loop {
            let (_, fty, fid) = p.read_field_begin(|_| (), FIELDS)?;
            match (fty, fid as ::std::primitive::i32) {
                (::fbthrift::TType::Stop, _) => break,
                (fty, _) => p.skip(fty)?,
            }
            p.read_field_end()?;
        }
        p.read_struct_end()?;
        ::std::result::Result::Ok(Self {
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        })
    }
}


mod dot_dot {
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OtherFields(pub(crate) ());

    #[allow(dead_code)] // if serde isn't being used
    pub(super) fn default_for_serde_deserialize() -> OtherFields {
        OtherFields(())
    }
}
