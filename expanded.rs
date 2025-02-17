#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::{path::Path, process::exit};
use config_checker_macros::Check;
use config_checker::*;
use serde::{Deserialize, Serialize};
struct Root {
    #[convert(as_str)]
    #[check(
        if(
            is_enum(self.child.gender, Gender::Male(_)),
            inside("Papy", "Papa"),
            if(is_enum(self.child.gender, Gender::Female(_)), inside("Mamie", "Maman"))
        )
    )]
    name: String,
    #[check(and(ge(0.), lt(self.child.value)))]
    value: f32,
    #[check]
    child: Child,
}
#[automatically_derived]
impl ::core::fmt::Debug for Root {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Root",
            "name",
            &self.name,
            "value",
            &self.value,
            "child",
            &&self.child,
        )
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Root {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "Root",
                false as usize + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "name",
                &self.name,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "value",
                &self.value,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "child",
                &self.child,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Root {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "name" => _serde::__private::Ok(__Field::__field0),
                        "value" => _serde::__private::Ok(__Field::__field1),
                        "child" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"name" => _serde::__private::Ok(__Field::__field0),
                        b"value" => _serde::__private::Ok(__Field::__field1),
                        b"child" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<Root>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Root;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct Root")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct Root with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        f32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Root with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<
                        Child,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct Root with 3 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(Root {
                        name: __field0,
                        value: __field1,
                        child: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<Child> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("value"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("child"),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<Child>(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("name")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("value")?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("child")?
                        }
                    };
                    _serde::__private::Ok(Root {
                        name: __field0,
                        value: __field1,
                        child: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["name", "value", "child"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "Root",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Root>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::clone::Clone for Root {
    #[inline]
    fn clone(&self) -> Root {
        Root {
            name: ::core::clone::Clone::clone(&self.name),
            value: ::core::clone::Clone::clone(&self.value),
            child: ::core::clone::Clone::clone(&self.child),
        }
    }
}
#[automatically_derived]
impl ::config_checker::ConfigCheckable for Root {
    fn check(&self) -> bool {
        use colored::Colorize;
        use ::config_checker::*;
        let mut ret = true;
        if !((((match self.child.gender {
            Gender::Male(_) => true,
            _ => false,
        })
            && <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new(["Papy", "Papa"]))
                .contains(&self.name.as_str()))
            || (((match self.child.gender {
                Gender::Female(_) => true,
                _ => false,
            })
                && <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new(["Mamie", "Maman"]),
                    )
                    .contains(&self.name.as_str())))))
        {
            {
                ::std::io::_print(
                    format_args!(
                        "{0} in field `{1}` of `{2}`\n",
                        "ERROR:".red(),
                        "name",
                        "Root",
                    ),
                );
            };
            ret = false;
        }
        if !((self.value >= 0.) && (self.value < self.child.value)) {
            {
                ::std::io::_print(
                    format_args!(
                        "{0} in field `{1}` of `{2}`\n",
                        "ERROR:".red(),
                        "value",
                        "Root",
                    ),
                );
            };
            ret = false;
        }
        if !::config_checker::check_config(&self.child) {
            ret = false;
            {
                ::std::io::_print(
                    format_args!(
                        "{0} {1} From field `{2}` of `{3}`\n",
                        "NOTE: ".blue(),
                        "\u{21b3}",
                        "child",
                        "Root",
                    ),
                );
            };
        }
        ret
    }
}
impl Default for Root {
    fn default() -> Self {
        Self {
            name: "root".to_string(),
            value: 2.,
            child: Child::default(),
        }
    }
}
pub enum Gender {
    Male(i32),
    Female(f32),
    Other(usize),
}
#[automatically_derived]
impl ::core::fmt::Debug for Gender {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Gender::Male(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Male", &__self_0)
            }
            Gender::Female(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Female", &__self_0)
            }
            Gender::Other(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Other", &__self_0)
            }
        }
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Gender {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                Gender::Male(ref __field0) => {
                    _serde::Serializer::serialize_newtype_variant(
                        __serializer,
                        "Gender",
                        0u32,
                        "Male",
                        __field0,
                    )
                }
                Gender::Female(ref __field0) => {
                    _serde::Serializer::serialize_newtype_variant(
                        __serializer,
                        "Gender",
                        1u32,
                        "Female",
                        __field0,
                    )
                }
                Gender::Other(ref __field0) => {
                    _serde::Serializer::serialize_newtype_variant(
                        __serializer,
                        "Gender",
                        2u32,
                        "Other",
                        __field0,
                    )
                }
            }
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Gender {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 3",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "Male" => _serde::__private::Ok(__Field::__field0),
                        "Female" => _serde::__private::Ok(__Field::__field1),
                        "Other" => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"Male" => _serde::__private::Ok(__Field::__field0),
                        b"Female" => _serde::__private::Ok(__Field::__field1),
                        b"Other" => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<Gender>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Gender;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "enum Gender")
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<
                                    i32,
                                >(__variant),
                                Gender::Male,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<
                                    f32,
                                >(__variant),
                                Gender::Female,
                            )
                        }
                        (__Field::__field2, __variant) => {
                            _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<
                                    usize,
                                >(__variant),
                                Gender::Other,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &["Male", "Female", "Other"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "Gender",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Gender>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::clone::Clone for Gender {
    #[inline]
    fn clone(&self) -> Gender {
        match self {
            Gender::Male(__self_0) => Gender::Male(::core::clone::Clone::clone(__self_0)),
            Gender::Female(__self_0) => {
                Gender::Female(::core::clone::Clone::clone(__self_0))
            }
            Gender::Other(__self_0) => {
                Gender::Other(::core::clone::Clone::clone(__self_0))
            }
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Gender {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Gender {
    #[inline]
    fn eq(&self, other: &Gender) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (Gender::Male(__self_0), Gender::Male(__arg1_0)) => __self_0 == __arg1_0,
                (Gender::Female(__self_0), Gender::Female(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                (Gender::Other(__self_0), Gender::Other(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                _ => unsafe { ::core::intrinsics::unreachable() }
            }
    }
}
struct Child {
    #[check(or(ge(1.), lt(0.)))]
    value: f32,
    gender: Gender,
    child: GreatChild,
}
#[automatically_derived]
impl ::core::fmt::Debug for Child {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Child",
            "value",
            &self.value,
            "gender",
            &self.gender,
            "child",
            &&self.child,
        )
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Child {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "Child",
                false as usize + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "value",
                &self.value,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "gender",
                &self.gender,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "child",
                &self.child,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Child {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "value" => _serde::__private::Ok(__Field::__field0),
                        "gender" => _serde::__private::Ok(__Field::__field1),
                        "child" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"value" => _serde::__private::Ok(__Field::__field0),
                        b"gender" => _serde::__private::Ok(__Field::__field1),
                        b"child" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<Child>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Child;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct Child")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        f32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct Child with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        Gender,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Child with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<
                        GreatChild,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct Child with 3 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(Child {
                        value: __field0,
                        gender: __field1,
                        child: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<Gender> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<GreatChild> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("value"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("gender"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<Gender>(&mut __map)?,
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("child"),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<GreatChild>(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("value")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("gender")?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("child")?
                        }
                    };
                    _serde::__private::Ok(Child {
                        value: __field0,
                        gender: __field1,
                        child: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["value", "gender", "child"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "Child",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Child>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::clone::Clone for Child {
    #[inline]
    fn clone(&self) -> Child {
        Child {
            value: ::core::clone::Clone::clone(&self.value),
            gender: ::core::clone::Clone::clone(&self.gender),
            child: ::core::clone::Clone::clone(&self.child),
        }
    }
}
#[automatically_derived]
impl ::config_checker::ConfigCheckable for Child {
    fn check(&self) -> bool {
        use colored::Colorize;
        use ::config_checker::*;
        let mut ret = true;
        if !((self.value >= 1.) || (self.value < 0.)) {
            {
                ::std::io::_print(
                    format_args!(
                        "{0} in field `{1}` of `{2}`\n",
                        "ERROR:".red(),
                        "value",
                        "Child",
                    ),
                );
            };
            ret = false;
        }
        ret
    }
}
impl Default for Child {
    fn default() -> Self {
        Self {
            value: 4.,
            gender: Gender::Other(0),
            child: GreatChild::default(),
        }
    }
}
struct GreatChild {
    name: String,
    value: f32,
}
#[automatically_derived]
impl ::core::fmt::Debug for GreatChild {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "GreatChild",
            "name",
            &self.name,
            "value",
            &&self.value,
        )
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for GreatChild {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "GreatChild",
                false as usize + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "name",
                &self.name,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "value",
                &self.value,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for GreatChild {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "name" => _serde::__private::Ok(__Field::__field0),
                        "value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"name" => _serde::__private::Ok(__Field::__field0),
                        b"value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<GreatChild>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = GreatChild;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct GreatChild",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct GreatChild with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        f32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct GreatChild with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(GreatChild {
                        name: __field0,
                        value: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<f32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("value"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("name")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("value")?
                        }
                    };
                    _serde::__private::Ok(GreatChild {
                        name: __field0,
                        value: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["name", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "GreatChild",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<GreatChild>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::clone::Clone for GreatChild {
    #[inline]
    fn clone(&self) -> GreatChild {
        GreatChild {
            name: ::core::clone::Clone::clone(&self.name),
            value: ::core::clone::Clone::clone(&self.value),
        }
    }
}
impl Default for GreatChild {
    fn default() -> Self {
        Self {
            name: "baby".to_string(),
            value: 6.,
        }
    }
}
fn main() {
    let config_path = Path::new(
        "/home/mescourrou/Dev/rust_configuration_checker/example/config1.yaml",
    );
    let config: Root = match confy::load_path(config_path) {
        Ok(config) => config,
        Err(error) => {
            {
                ::std::io::_print(
                    format_args!(
                        "Error from Confy while loading the config file : {0}\n",
                        error,
                    ),
                );
            };
            exit(-1);
        }
    };
    {
        ::std::io::_print(format_args!("Config check: {0}\n", config.check()));
    };
    if let Gender::Female(_) = config.child.gender {}
    {
        ::std::io::_print(format_args!("Loaded configuration: \n{0:#?}\n", config));
    };
}
