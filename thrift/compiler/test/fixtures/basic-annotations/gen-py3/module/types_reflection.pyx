#
# Autogenerated by Thrift
#
# DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
#  @generated
#


import folly.iobuf as _fbthrift_iobuf

from thrift.py3.reflection cimport (
    NumberType as __NumberType,
    StructType as __StructType,
    Qualifier as __Qualifier,
)


cimport module.types as _module_types

from thrift.py3.types cimport (
    constant_shared_ptr,
    default_inst,
)


cdef __StructSpec get_reflection__MyStructNestedAnnotation():
    cdef _module_types.MyStructNestedAnnotation defaults = _module_types.MyStructNestedAnnotation._fbthrift_create(
        constant_shared_ptr[_module_types.cMyStructNestedAnnotation](
            default_inst[_module_types.cMyStructNestedAnnotation]()
        )
    )
    cdef __StructSpec spec = __StructSpec._fbthrift_create(
        name="MyStructNestedAnnotation",
        kind=__StructType.STRUCT,
        annotations={
        },
    )
    spec.add_field(
        __FieldSpec._fbthrift_create(
            id=1,
            name="name",
            type=str,
            kind=__NumberType.NOT_A_NUMBER,
            qualifier=__Qualifier.UNQUALIFIED,
            default=None,
            annotations={
            },
        ),
    )
    return spec
cdef __StructSpec get_reflection__MyUnion():
    cdef __StructSpec spec = __StructSpec._fbthrift_create(
        name="MyUnion",
        kind=__StructType.UNION,
        annotations={
            """cpp.adapter""": """StaticCast""",            """cpp.name""": """YourUnion""",        },
    )
    return spec
cdef __StructSpec get_reflection__MyException():
    cdef _module_types.MyException defaults = _module_types.MyException._fbthrift_create(
        constant_shared_ptr[_module_types.cMyException](
            default_inst[_module_types.cMyException]()
        )
    )
    cdef __StructSpec spec = __StructSpec._fbthrift_create(
        name="MyException",
        kind=__StructType.EXCEPTION,
        annotations={
            """cpp.adapter""": """StaticCast""",            """cpp.name""": """YourException""",        },
    )
    return spec
cdef __StructSpec get_reflection__MyStruct():
    cdef _module_types.MyStruct defaults = _module_types.MyStruct._fbthrift_create(
        constant_shared_ptr[_module_types.cMyStruct](
            default_inst[_module_types.cMyStruct]()
        )
    )
    cdef __StructSpec spec = __StructSpec._fbthrift_create(
        name="MyStruct",
        kind=__StructType.STRUCT,
        annotations={
            """android.generate_builder""": """1""",            """cpp.adapter""": """StaticCast""",            """cpp.internal.deprecated._data.method""": """1""",            """cpp.name""": """YourStruct""",            """thrift.uri""": """facebook.com/thrift/compiler/test/fixtures/basic-annotations/src/module/MyStruct""",        },
    )
    spec.add_field(
        __FieldSpec._fbthrift_create(
            id=2,
            name="major",
            type=int,
            kind=__NumberType.I64,
            qualifier=__Qualifier.UNQUALIFIED,
            default=None,
            annotations={
                """cpp.name""": """majorVer""",            },
        ),
    )
    spec.add_field(
        __FieldSpec._fbthrift_create(
            id=1,
            name="package",
            type=str,
            kind=__NumberType.NOT_A_NUMBER,
            qualifier=__Qualifier.UNQUALIFIED,
            default=None,
            annotations={
                """java.swift.name""": """_package""",            },
        ),
    )
    spec.add_field(
        __FieldSpec._fbthrift_create(
            id=3,
            name="annotation_with_quote",
            type=str,
            kind=__NumberType.NOT_A_NUMBER,
            qualifier=__Qualifier.UNQUALIFIED,
            default=None,
            annotations={
                """go.tag""": """tag:\"somevalue\"""",            },
        ),
    )
    spec.add_field(
        __FieldSpec._fbthrift_create(
            id=4,
            name="class_",
            type=str,
            kind=__NumberType.NOT_A_NUMBER,
            qualifier=__Qualifier.UNQUALIFIED,
            default=None,
            annotations={
                """java.swift.name""": """class_""",            },
        ),
    )
    spec.add_field(
        __FieldSpec._fbthrift_create(
            id=5,
            name="annotation_with_trailing_comma",
            type=str,
            kind=__NumberType.NOT_A_NUMBER,
            qualifier=__Qualifier.UNQUALIFIED,
            default=None,
            annotations={
                """custom""": """test""",            },
        ),
    )
    spec.add_field(
        __FieldSpec._fbthrift_create(
            id=6,
            name="empty_annotations",
            type=str,
            kind=__NumberType.NOT_A_NUMBER,
            qualifier=__Qualifier.UNQUALIFIED,
            default=None,
            annotations={
            },
        ),
    )
    spec.add_field(
        __FieldSpec._fbthrift_create(
            id=7,
            name="my_enum",
            type=_module_types.MyEnum,
            kind=__NumberType.NOT_A_NUMBER,
            qualifier=__Qualifier.UNQUALIFIED,
            default=None,
            annotations={
            },
        ),
    )
    spec.add_field(
        __FieldSpec._fbthrift_create(
            id=8,
            name="cpp_type_annotation",
            type=_module_types.std_deque_std_string__List__string,
            kind=__NumberType.NOT_A_NUMBER,
            qualifier=__Qualifier.UNQUALIFIED,
            default=None,
            annotations={
            },
        ),
    )
    spec.add_field(
        __FieldSpec._fbthrift_create(
            id=9,
            name="my_union",
            type=_module_types.MyUnion,
            kind=__NumberType.NOT_A_NUMBER,
            qualifier=__Qualifier.UNQUALIFIED,
            default=None,
            annotations={
            },
        ),
    )
    return spec
cdef __StructSpec get_reflection__SecretStruct():
    cdef _module_types.SecretStruct defaults = _module_types.SecretStruct._fbthrift_create(
        constant_shared_ptr[_module_types.cSecretStruct](
            default_inst[_module_types.cSecretStruct]()
        )
    )
    cdef __StructSpec spec = __StructSpec._fbthrift_create(
        name="SecretStruct",
        kind=__StructType.STRUCT,
        annotations={
        },
    )
    spec.add_field(
        __FieldSpec._fbthrift_create(
            id=1,
            name="id",
            type=int,
            kind=__NumberType.I64,
            qualifier=__Qualifier.UNQUALIFIED,
            default=None,
            annotations={
            },
        ),
    )
    spec.add_field(
        __FieldSpec._fbthrift_create(
            id=2,
            name="password",
            type=str,
            kind=__NumberType.NOT_A_NUMBER,
            qualifier=__Qualifier.UNQUALIFIED,
            default=None,
            annotations={
                """java.sensitive""": """1""",            },
        ),
    )
    return spec
cdef __ListSpec get_reflection__std_deque_std_string__List__string():
    return __ListSpec._fbthrift_create(
        value=str,
        kind=__NumberType.NOT_A_NUMBER,
    )

