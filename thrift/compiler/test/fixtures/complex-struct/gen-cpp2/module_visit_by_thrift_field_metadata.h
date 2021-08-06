/**
 * Autogenerated by Thrift for src/module.thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated
 */
#pragma once

#include <thrift/lib/cpp2/visitation/visit_by_thrift_field_metadata.h>
#include "thrift/compiler/test/fixtures/complex-struct/gen-cpp2/module_metadata.h"

namespace apache {
namespace thrift {
namespace detail {

template <>
struct VisitByFieldId<::cpp2::MyStructFloatFieldThrowExp> {
  template <typename F, typename T>
  void operator()(FOLLY_MAYBE_UNUSED F&& f, int32_t fieldId, FOLLY_MAYBE_UNUSED T&& t) const {
    switch (fieldId) {
    case 1:
      return f(0, static_cast<T&&>(t).myLongField_ref());
    case 2:
      return f(1, static_cast<T&&>(t).MyByteField_ref());
    case 3:
      return f(2, static_cast<T&&>(t).myStringField_ref());
    case 4:
      return f(3, static_cast<T&&>(t).myFloatField_ref());
    default:
      throwInvalidThriftId(fieldId, "::cpp2::MyStructFloatFieldThrowExp");
    }
  }
};

template <>
struct VisitByFieldId<::cpp2::MyStructMapFloatThrowExp> {
  template <typename F, typename T>
  void operator()(FOLLY_MAYBE_UNUSED F&& f, int32_t fieldId, FOLLY_MAYBE_UNUSED T&& t) const {
    switch (fieldId) {
    case 1:
      return f(0, static_cast<T&&>(t).myLongField_ref());
    case 2:
      return f(1, static_cast<T&&>(t).mapListOfFloats_ref());
    default:
      throwInvalidThriftId(fieldId, "::cpp2::MyStructMapFloatThrowExp");
    }
  }
};

template <>
struct VisitByFieldId<::cpp2::MyDataItem> {
  template <typename F, typename T>
  void operator()(FOLLY_MAYBE_UNUSED F&& f, int32_t fieldId, FOLLY_MAYBE_UNUSED T&& t) const {
    switch (fieldId) {
    default:
      throwInvalidThriftId(fieldId, "::cpp2::MyDataItem");
    }
  }
};

template <>
struct VisitByFieldId<::cpp2::MyStruct> {
  template <typename F, typename T>
  void operator()(FOLLY_MAYBE_UNUSED F&& f, int32_t fieldId, FOLLY_MAYBE_UNUSED T&& t) const {
    switch (fieldId) {
    case 1:
      return f(0, static_cast<T&&>(t).MyIntField_ref());
    case 2:
      return f(1, static_cast<T&&>(t).MyStringField_ref());
    case 3:
      return f(2, static_cast<T&&>(t).MyDataField_ref());
    case 4:
      return f(3, static_cast<T&&>(t).myEnum_ref());
    case 5:
      return f(4, static_cast<T&&>(t).MyBoolField_ref());
    case 6:
      return f(5, static_cast<T&&>(t).MyByteField_ref());
    case 7:
      return f(6, static_cast<T&&>(t).MyShortField_ref());
    case 8:
      return f(7, static_cast<T&&>(t).MyLongField_ref());
    case 9:
      return f(8, static_cast<T&&>(t).MyDoubleField_ref());
    case 10:
      return f(9, static_cast<T&&>(t).lDouble_ref());
    case 11:
      return f(10, static_cast<T&&>(t).lShort_ref());
    case 12:
      return f(11, static_cast<T&&>(t).lInteger_ref());
    case 13:
      return f(12, static_cast<T&&>(t).lLong_ref());
    case 14:
      return f(13, static_cast<T&&>(t).lString_ref());
    case 15:
      return f(14, static_cast<T&&>(t).lBool_ref());
    case 16:
      return f(15, static_cast<T&&>(t).lByte_ref());
    case 17:
      return f(16, static_cast<T&&>(t).mShortString_ref());
    case 18:
      return f(17, static_cast<T&&>(t).mIntegerString_ref());
    case 19:
      return f(18, static_cast<T&&>(t).mStringMyStruct_ref());
    case 20:
      return f(19, static_cast<T&&>(t).mStringBool_ref());
    case 21:
      return f(20, static_cast<T&&>(t).mIntegerInteger_ref());
    case 22:
      return f(21, static_cast<T&&>(t).mIntegerBool_ref());
    case 23:
      return f(22, static_cast<T&&>(t).sShort_ref());
    case 24:
      return f(23, static_cast<T&&>(t).sMyStruct_ref());
    case 25:
      return f(24, static_cast<T&&>(t).sLong_ref());
    case 26:
      return f(25, static_cast<T&&>(t).sString_ref());
    case 27:
      return f(26, static_cast<T&&>(t).sByte_ref());
    case 28:
      return f(27, static_cast<T&&>(t).mListList_ref());
    default:
      throwInvalidThriftId(fieldId, "::cpp2::MyStruct");
    }
  }
};

template <>
struct VisitByFieldId<::cpp2::SimpleStruct> {
  template <typename F, typename T>
  void operator()(FOLLY_MAYBE_UNUSED F&& f, int32_t fieldId, FOLLY_MAYBE_UNUSED T&& t) const {
    switch (fieldId) {
    case 1:
      return f(0, static_cast<T&&>(t).age_ref());
    case 2:
      return f(1, static_cast<T&&>(t).name_ref());
    default:
      throwInvalidThriftId(fieldId, "::cpp2::SimpleStruct");
    }
  }
};

template <>
struct VisitByFieldId<::cpp2::ComplexNestedStruct> {
  template <typename F, typename T>
  void operator()(FOLLY_MAYBE_UNUSED F&& f, int32_t fieldId, FOLLY_MAYBE_UNUSED T&& t) const {
    switch (fieldId) {
    case 1:
      return f(0, static_cast<T&&>(t).setOfSetOfInt_ref());
    case 2:
      return f(1, static_cast<T&&>(t).listofListOfListOfListOfEnum_ref());
    case 3:
      return f(2, static_cast<T&&>(t).listOfListOfMyStruct_ref());
    case 4:
      return f(3, static_cast<T&&>(t).setOfListOfListOfLong_ref());
    case 5:
      return f(4, static_cast<T&&>(t).setOfSetOfsetOfLong_ref());
    case 6:
      return f(5, static_cast<T&&>(t).mapStructListOfListOfLong_ref());
    case 7:
      return f(6, static_cast<T&&>(t).mKeyStructValInt_ref());
    case 8:
      return f(7, static_cast<T&&>(t).listOfMapKeyIntValInt_ref());
    case 9:
      return f(8, static_cast<T&&>(t).listOfMapKeyStrValList_ref());
    case 10:
      return f(9, static_cast<T&&>(t).mapKeySetValLong_ref());
    case 11:
      return f(10, static_cast<T&&>(t).mapKeyListValLong_ref());
    case 12:
      return f(11, static_cast<T&&>(t).mapKeyMapValMap_ref());
    case 13:
      return f(12, static_cast<T&&>(t).mapKeySetValMap_ref());
    case 14:
      return f(13, static_cast<T&&>(t).NestedMaps_ref());
    case 15:
      return f(14, static_cast<T&&>(t).mapKeyIntValList_ref());
    case 16:
      return f(15, static_cast<T&&>(t).mapKeyIntValSet_ref());
    case 17:
      return f(16, static_cast<T&&>(t).mapKeySetValInt_ref());
    case 18:
      return f(17, static_cast<T&&>(t).mapKeyListValSet_ref());
    default:
      throwInvalidThriftId(fieldId, "::cpp2::ComplexNestedStruct");
    }
  }
};

template <>
struct VisitByFieldId<::cpp2::MyUnion> {
  template <typename F, typename T>
  void operator()(FOLLY_MAYBE_UNUSED F&& f, int32_t fieldId, FOLLY_MAYBE_UNUSED T&& t) const {
    switch (fieldId) {
    case 1:
      return f(0, static_cast<T&&>(t).myEnum_ref());
    case 2:
      return f(1, static_cast<T&&>(t).myStruct_ref());
    case 3:
      return f(2, static_cast<T&&>(t).myDataItem_ref());
    case 4:
      return f(3, static_cast<T&&>(t).complexNestedStruct_ref());
    case 5:
      return f(4, static_cast<T&&>(t).longValue_ref());
    case 6:
      return f(5, static_cast<T&&>(t).intValue_ref());
    default:
      throwInvalidThriftId(fieldId, "::cpp2::MyUnion");
    }
  }
};

template <>
struct VisitByFieldId<::cpp2::defaultStruct> {
  template <typename F, typename T>
  void operator()(FOLLY_MAYBE_UNUSED F&& f, int32_t fieldId, FOLLY_MAYBE_UNUSED T&& t) const {
    switch (fieldId) {
    case 1:
      return f(0, static_cast<T&&>(t).myLongDFset_ref());
    case 2:
      return f(1, static_cast<T&&>(t).myLongDF_ref());
    case 3:
      return f(2, static_cast<T&&>(t).portDFset_ref());
    case 4:
      return f(3, static_cast<T&&>(t).portNum_ref());
    case 5:
      return f(4, static_cast<T&&>(t).myBinaryDFset_ref());
    case 6:
      return f(5, static_cast<T&&>(t).myBinary_ref());
    case 7:
      return f(6, static_cast<T&&>(t).myByteDFSet_ref());
    case 8:
      return f(7, static_cast<T&&>(t).myByte_ref());
    case 9:
      return f(8, static_cast<T&&>(t).myDoubleDFset_ref());
    case 10:
      return f(9, static_cast<T&&>(t).myDoubleDFZero_ref());
    case 12:
      return f(10, static_cast<T&&>(t).myDouble_ref());
    case 13:
      return f(11, static_cast<T&&>(t).field3_ref());
    case 14:
      return f(12, static_cast<T&&>(t).myList_ref());
    case 15:
      return f(13, static_cast<T&&>(t).mySet_ref());
    case 16:
      return f(14, static_cast<T&&>(t).simpleStruct_ref());
    case 17:
      return f(15, static_cast<T&&>(t).listStructDFset_ref());
    case 18:
      return f(16, static_cast<T&&>(t).myUnion_ref());
    case 19:
      return f(17, static_cast<T&&>(t).listUnionDFset_ref());
    case 20:
      return f(18, static_cast<T&&>(t).mapNestlistStructDfSet_ref());
    case 21:
      return f(19, static_cast<T&&>(t).mapJavaTypeDFset_ref());
    case 22:
      return f(20, static_cast<T&&>(t).emptyMap_ref());
    case 23:
      return f(21, static_cast<T&&>(t).enumMapDFset_ref());
    default:
      throwInvalidThriftId(fieldId, "::cpp2::defaultStruct");
    }
  }
};

template <>
struct VisitByFieldId<::cpp2::MyStructTypeDef> {
  template <typename F, typename T>
  void operator()(FOLLY_MAYBE_UNUSED F&& f, int32_t fieldId, FOLLY_MAYBE_UNUSED T&& t) const {
    switch (fieldId) {
    case 1:
      return f(0, static_cast<T&&>(t).myLongField_ref());
    case 2:
      return f(1, static_cast<T&&>(t).myLongTypeDef_ref());
    case 3:
      return f(2, static_cast<T&&>(t).myStringField_ref());
    case 4:
      return f(3, static_cast<T&&>(t).myStringTypedef_ref());
    case 5:
      return f(4, static_cast<T&&>(t).myMapField_ref());
    case 6:
      return f(5, static_cast<T&&>(t).myMapTypedef_ref());
    case 7:
      return f(6, static_cast<T&&>(t).myListField_ref());
    case 8:
      return f(7, static_cast<T&&>(t).myListTypedef_ref());
    case 9:
      return f(8, static_cast<T&&>(t).myMapListOfTypeDef_ref());
    default:
      throwInvalidThriftId(fieldId, "::cpp2::MyStructTypeDef");
    }
  }
};

template <>
struct VisitByFieldId<::cpp2::MyUnionFloatFieldThrowExp> {
  template <typename F, typename T>
  void operator()(FOLLY_MAYBE_UNUSED F&& f, int32_t fieldId, FOLLY_MAYBE_UNUSED T&& t) const {
    switch (fieldId) {
    case 1:
      return f(0, static_cast<T&&>(t).myEnum_ref());
    case 2:
      return f(1, static_cast<T&&>(t).setFloat_ref());
    case 3:
      return f(2, static_cast<T&&>(t).myDataItem_ref());
    case 4:
      return f(3, static_cast<T&&>(t).complexNestedStruct_ref());
    default:
      throwInvalidThriftId(fieldId, "::cpp2::MyUnionFloatFieldThrowExp");
    }
  }
};

template <>
struct VisitByFieldId<::cpp2::TypeRemapped> {
  template <typename F, typename T>
  void operator()(FOLLY_MAYBE_UNUSED F&& f, int32_t fieldId, FOLLY_MAYBE_UNUSED T&& t) const {
    switch (fieldId) {
    case 1:
      return f(0, static_cast<T&&>(t).lsMap_ref());
    case 2:
      return f(1, static_cast<T&&>(t).ioMap_ref());
    case 3:
      return f(2, static_cast<T&&>(t).BigInteger_ref());
    case 4:
      return f(3, static_cast<T&&>(t).binaryTestBuffer_ref());
    default:
      throwInvalidThriftId(fieldId, "::cpp2::TypeRemapped");
    }
  }
};

template <>
struct VisitByFieldId<::cpp2::emptyXcep> {
  template <typename F, typename T>
  void operator()(FOLLY_MAYBE_UNUSED F&& f, int32_t fieldId, FOLLY_MAYBE_UNUSED T&& t) const {
    switch (fieldId) {
    default:
      throwInvalidThriftId(fieldId, "::cpp2::emptyXcep");
    }
  }
};

template <>
struct VisitByFieldId<::cpp2::reqXcep> {
  template <typename F, typename T>
  void operator()(FOLLY_MAYBE_UNUSED F&& f, int32_t fieldId, FOLLY_MAYBE_UNUSED T&& t) const {
    switch (fieldId) {
    case 1:
      return f(0, static_cast<T&&>(t).message_ref());
    case 2:
      return f(1, static_cast<T&&>(t).errorCode_ref());
    default:
      throwInvalidThriftId(fieldId, "::cpp2::reqXcep");
    }
  }
};

template <>
struct VisitByFieldId<::cpp2::optXcep> {
  template <typename F, typename T>
  void operator()(FOLLY_MAYBE_UNUSED F&& f, int32_t fieldId, FOLLY_MAYBE_UNUSED T&& t) const {
    switch (fieldId) {
    case 1:
      return f(0, static_cast<T&&>(t).message_ref());
    case 2:
      return f(1, static_cast<T&&>(t).errorCode_ref());
    default:
      throwInvalidThriftId(fieldId, "::cpp2::optXcep");
    }
  }
};

template <>
struct VisitByFieldId<::cpp2::complexException> {
  template <typename F, typename T>
  void operator()(FOLLY_MAYBE_UNUSED F&& f, int32_t fieldId, FOLLY_MAYBE_UNUSED T&& t) const {
    switch (fieldId) {
    case 1:
      return f(0, static_cast<T&&>(t).message_ref());
    case 2:
      return f(1, static_cast<T&&>(t).listStrings_ref());
    case 3:
      return f(2, static_cast<T&&>(t).errorEnum_ref());
    case 4:
      return f(3, static_cast<T&&>(t).unionError_ref());
    case 5:
      return f(4, static_cast<T&&>(t).structError_ref());
    case 6:
      return f(5, static_cast<T&&>(t).lsMap_ref());
    default:
      throwInvalidThriftId(fieldId, "::cpp2::complexException");
    }
  }
};
} // namespace detail
} // namespace thrift
} // namespace apache
