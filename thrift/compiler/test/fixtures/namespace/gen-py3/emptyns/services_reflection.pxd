#
# Autogenerated by Thrift
#
# DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
#  @generated
#

from thrift.py3.reflection cimport (
  InterfaceSpec as __InterfaceSpec,
)


cdef __InterfaceSpec get_reflection__TestService(bint for_clients)

cdef extern from "src/gen-cpp2/emptyns_handlers.h" namespace "::cpp2":
    cdef cppclass cTestServiceSvIf "::cpp2::TestServiceSvIf":
        pass
