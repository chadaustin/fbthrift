#
# Autogenerated by Thrift
#
# DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
#  @generated
#

from thrift.py3.reflection cimport (
  InterfaceSpec as __InterfaceSpec,
)


cdef __InterfaceSpec get_reflection__A(bint for_clients)

cdef extern from "src/gen-cpp2/module_handlers.h" namespace "::cpp2":
    cdef cppclass cASvIf "::cpp2::ASvIf":
        pass

cdef __InterfaceSpec get_reflection__B(bint for_clients)

cdef extern from "src/gen-cpp2/module_handlers.h" namespace "::cpp2":
    cdef cppclass cBSvIf "::cpp2::BSvIf":
        pass

cdef __InterfaceSpec get_reflection__C(bint for_clients)

cdef extern from "src/gen-cpp2/module_handlers.h" namespace "::cpp2":
    cdef cppclass cCSvIf "::cpp2::CSvIf":
        pass
