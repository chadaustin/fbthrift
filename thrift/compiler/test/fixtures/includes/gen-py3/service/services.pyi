#
# Autogenerated by Thrift
#
# DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
#  @generated
#

import folly.iobuf as __iobuf
import typing as _typing
from thrift.py3.server import RequestContext, ServiceInterface
from abc import abstractmethod, ABCMeta

import service.types as _service_types
import module.types as _module_types
import includes.types as _includes_types
import transitive.types as _transitive_types

_MyServiceInterfaceT = _typing.TypeVar('_MyServiceInterfaceT', bound='MyServiceInterface')


class MyServiceInterface(
    ServiceInterface
    , metaclass=ABCMeta
):

    @staticmethod
    def pass_context_query(
        fn: _typing.Callable[
                [_MyServiceInterfaceT, RequestContext, _module_types.MyStruct, _includes_types.Included],
                _typing.Coroutine[_typing.Any, _typing.Any, None]
        ]
    ) -> _typing.Callable[
        [_MyServiceInterfaceT, _module_types.MyStruct, _includes_types.Included],
        _typing.Coroutine[_typing.Any, _typing.Any, None]
    ]: ...

    @abstractmethod
    async def query(
        self,
        s: _module_types.MyStruct,
        i: _includes_types.Included
    ) -> None: ...

    @staticmethod
    def pass_context_has_arg_docs(
        fn: _typing.Callable[
                [_MyServiceInterfaceT, RequestContext, _module_types.MyStruct, _includes_types.Included],
                _typing.Coroutine[_typing.Any, _typing.Any, None]
        ]
    ) -> _typing.Callable[
        [_MyServiceInterfaceT, _module_types.MyStruct, _includes_types.Included],
        _typing.Coroutine[_typing.Any, _typing.Any, None]
    ]: ...

    @abstractmethod
    async def has_arg_docs(
        self,
        s: _module_types.MyStruct,
        i: _includes_types.Included
    ) -> None: ...
    pass


