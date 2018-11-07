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

import hsmodule.types as _hsmodule_types

_HsTestServiceInterfaceT = _typing.TypeVar('_HsTestServiceInterfaceT', bound='HsTestServiceInterface')


class HsTestServiceInterface(
    ServiceInterface
    , metaclass=ABCMeta
):

    @staticmethod
    def pass_context_init(
        fn: _typing.Callable[
                [_HsTestServiceInterfaceT, RequestContext, int],
                _typing.Coroutine[_typing.Any, _typing.Any, int]
        ]
    ) -> _typing.Callable[
        [_HsTestServiceInterfaceT, int],
        _typing.Coroutine[_typing.Any, _typing.Any, int]
    ]: ...

    @abstractmethod
    async def init(
        self,
        int1: int
    ) -> int: ...
    pass


