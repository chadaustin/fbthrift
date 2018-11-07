#
# Autogenerated by Thrift
#
# DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
#  @generated
#

import folly.iobuf as __iobuf
import thrift.py3.types
import thrift.py3.client
import thrift.py3.common
import typing as _typing
from types import TracebackType

import module.types as _module_types


_SimpleServiceT = _typing.TypeVar('_SimpleServiceT', bound='SimpleService')


class SimpleService(thrift.py3.client.Client):

    async def __aenter__(self: _SimpleServiceT) -> _SimpleServiceT: ...
    async def __aexit__(
        self,
        exc_type: _typing.Optional[_typing.Type[BaseException]],
        exc: _typing.Optional[BaseException],
        tb: _typing.Optional[TracebackType],
    ) -> _typing.Optional[bool]: ...

    def set_persistent_header(self, key: str, value: str) -> None: ...

    async def get_five(
        self,
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> int: ...

    async def add_five(
        self,
        num: int,
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> int: ...

    async def do_nothing(
        self,
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> None: ...

    async def concat(
        self,
        first: str,
        second: str,
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> str: ...

    async def get_value(
        self,
        simple_struct: _module_types.SimpleStruct,
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> int: ...

    async def negate(
        self,
        input: bool,
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> bool: ...

    async def tiny(
        self,
        input: int,
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> int: ...

    async def small(
        self,
        input: int,
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> int: ...

    async def big(
        self,
        input: int,
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> int: ...

    async def two(
        self,
        input: float,
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> float: ...

    async def expected_exception(
        self,
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> None: ...

    async def unexpected_exception(
        self,
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> int: ...

    async def sum_i16_list(
        self,
        numbers: _typing.Sequence[int],
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> int: ...

    async def sum_i32_list(
        self,
        numbers: _typing.Sequence[int],
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> int: ...

    async def sum_i64_list(
        self,
        numbers: _typing.Sequence[int],
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> int: ...

    async def concat_many(
        self,
        words: _typing.Sequence[str],
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> str: ...

    async def count_structs(
        self,
        items: _typing.Sequence[_module_types.SimpleStruct],
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> int: ...

    async def sum_set(
        self,
        numbers: _typing.AbstractSet[int],
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> int: ...

    async def contains_word(
        self,
        words: _typing.AbstractSet[str],
        word: str,
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> bool: ...

    async def get_map_value(
        self,
        words: _typing.Mapping[str, str],
        key: str,
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> str: ...

    async def map_length(
        self,
        items: _typing.Mapping[str, _module_types.SimpleStruct],
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> int: ...

    async def sum_map_values(
        self,
        items: _typing.Mapping[str, int],
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> int: ...

    async def complex_sum_i32(
        self,
        counter: _module_types.ComplexStruct,
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> int: ...

    async def repeat_name(
        self,
        counter: _module_types.ComplexStruct,
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> str: ...

    async def get_struct(
        self,
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> _module_types.SimpleStruct: ...

    async def fib(
        self,
        n: int,
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> _typing.Sequence[int]: ...

    async def unique_words(
        self,
        words: _typing.Sequence[str],
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> _typing.AbstractSet[str]: ...

    async def words_count(
        self,
        words: _typing.Sequence[str],
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> _typing.Mapping[str, int]: ...

    async def set_enum(
        self,
        in_enum: _module_types.AnEnum,
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> _module_types.AnEnum: ...

    async def list_of_lists(
        self,
        num_lists: int,
        num_items: int,
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> _typing.Sequence[_typing.Sequence[int]]: ...

    async def word_character_frequency(
        self,
        sentence: str,
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> _typing.Mapping[str, _typing.Mapping[str, int]]: ...

    async def list_of_sets(
        self,
        some_words: str,
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> _typing.Sequence[_typing.AbstractSet[str]]: ...

    async def nested_map_argument(
        self,
        struct_map: _typing.Mapping[str, _typing.Sequence[_module_types.SimpleStruct]],
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> int: ...

    async def make_sentence(
        self,
        word_chars: _typing.Sequence[_typing.Sequence[str]],
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> str: ...

    async def get_union(
        self,
        sets: _typing.Sequence[_typing.AbstractSet[int]],
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> _typing.AbstractSet[int]: ...

    async def get_keys(
        self,
        string_map: _typing.Sequence[_typing.Mapping[str, str]],
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> _typing.AbstractSet[str]: ...

    async def lookup_double(
        self,
        key: int,
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> float: ...

    async def retrieve_binary(
        self,
        something: bytes,
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> bytes: ...

    async def contain_binary(
        self,
        binaries: _typing.Sequence[bytes],
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> _typing.AbstractSet[bytes]: ...

    async def contain_enum(
        self,
        the_enum: _typing.Sequence[_module_types.AnEnum],
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> _typing.Sequence[_module_types.AnEnum]: ...



_DerivedServiceT = _typing.TypeVar('_DerivedServiceT', bound='DerivedService')


class DerivedService(SimpleService):

    async def __aenter__(self: _DerivedServiceT) -> _DerivedServiceT: ...
    async def __aexit__(
        self,
        exc_type: _typing.Optional[_typing.Type[BaseException]],
        exc: _typing.Optional[BaseException],
        tb: _typing.Optional[TracebackType],
    ) -> _typing.Optional[bool]: ...

    def set_persistent_header(self, key: str, value: str) -> None: ...

    async def get_six(
        self,
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> int: ...



_RederivedServiceT = _typing.TypeVar('_RederivedServiceT', bound='RederivedService')


class RederivedService(DerivedService):

    async def __aenter__(self: _RederivedServiceT) -> _RederivedServiceT: ...
    async def __aexit__(
        self,
        exc_type: _typing.Optional[_typing.Type[BaseException]],
        exc: _typing.Optional[BaseException],
        tb: _typing.Optional[TracebackType],
    ) -> _typing.Optional[bool]: ...

    def set_persistent_header(self, key: str, value: str) -> None: ...

    async def get_seven(
        self,
        rpc_options: _typing.Optional[thrift.py3.common.RpcOptions]=None
    ) -> int: ...

