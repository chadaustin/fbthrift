#!/usr/bin/env python3
# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

import asyncio
import unittest
from typing import Mapping

from testing.thrift_types import (
    Integers,
    StringBucket,
    easy,
    hard,
)
from thrift.python.exceptions import Error
from thrift.python.serializer import Protocol
from thrift.python.serializer import (
    deserialize,
    deserialize_with_length,
    serialize,
    serialize_iobuf,
)
from thrift.python.types import StructOrUnion


class SerializerTests(unittest.TestCase):
    def test_None(self) -> None:
        with self.assertRaises(TypeError):
            # pyre-ignore[6]: intentionally introduced for testing
            serialize(None)

    def test_sanity(self) -> None:
        with self.assertRaises(TypeError):
            # pyre-ignore[6]: intentionally introduced for testing
            serialize(1, Protocol.COMPACT)

        with self.assertRaises(TypeError):
            # pyre-ignore[6]: intentionally introduced for testing
            serialize(easy(), None)

        with self.assertRaises(TypeError):
            # pyre-ignore[6]: intentionally introduced for testing
            deserialize(Protocol, b"")

    def test_from_thread_pool(self) -> None:
        control = easy(val=5, val_list=[1, 2, 3, 4])
        loop = asyncio.get_event_loop()
        coro = loop.run_in_executor(None, serialize, control)
        encoded = loop.run_until_complete(coro)
        coro = loop.run_in_executor(None, deserialize, type(control), encoded)
        decoded = loop.run_until_complete(coro)
        self.assertEqual(control, decoded)

    def test_serialize_iobuf(self) -> None:
        control = easy(val=5, val_list=[1, 2, 3, 4, 5])
        iobuf = serialize_iobuf(control)
        decoded = deserialize(type(control), iobuf)
        self.assertEqual(control, decoded)

    def test_bad_deserialize(self) -> None:
        with self.assertRaises(Error):
            deserialize(easy, b"")
        with self.assertRaises(Error):
            deserialize(easy, b"\x05AAAAAAAA")
        with self.assertRaises(Error):
            deserialize(easy, b"\x02\xDE\xAD\xBE\xEF", protocol=Protocol.BINARY)

    def thrift_serialization_round_robin(
        self, control: StructOrUnion, fixtures: Mapping[Protocol, bytes]
    ) -> None:
        for proto in Protocol:
            encoded = serialize(control, protocol=proto)
            self.assertIsInstance(encoded, bytes)
            decoded = deserialize(type(control), encoded, protocol=proto)
            self.assertIsInstance(decoded, type(control))
            self.assertEqual(control, decoded)
            self.assertEqual((proto, encoded), (proto, fixtures.get(proto)))

    def test_serialize_easy_struct(self) -> None:
        control = easy(val=5, val_list=[1, 2, 3, 4])
        fixtures: Mapping[Protocol, bytes] = {
            Protocol.COMPACT: b"\x15\n\x19E\x02\x04\x06\x08,\x00\x00",
            Protocol.BINARY: b"\x08\x00\x01\x00\x00\x00\x05\x0f\x00\x02\x08\x00\x00\x00"
            b"\x04\x00\x00\x00\x01\x00\x00\x00\x02\x00\x00\x00\x03\x00"
            b"\x00\x00\x04\x0c\x00\x04\x00\x00",
            # Protocol.JSON: b'{"val":5,"val_list":[1,2,3,4],"an_int":{}}',
        }
        self.thrift_serialization_round_robin(control, fixtures)

    def test_serialize_hard_struct(self) -> None:
        control = hard(
            val=0, val_list=[1, 2, 3, 4], name="foo", an_int=Integers(tiny=1)
        )
        fixtures: Mapping[Protocol, bytes] = {
            Protocol.COMPACT: b"\x15\x00\x19E\x02\x04\x06\x08\x18\x03foo\x1c\x13\x01"
            b"\x00\x18\x0csome default\x00",
            Protocol.BINARY: b"\x08\x00\x01\x00\x00\x00\x00\x0f\x00\x02\x08\x00\x00\x00"
            b"\x04\x00\x00\x00\x01\x00\x00\x00\x02\x00\x00\x00\x03\x00"
            b"\x00\x00\x04\x0b\x00\x03\x00\x00\x00\x03foo\x0c\x00\x04"
            b"\x03\x00\x01\x01\x00\x0b\x00\x05\x00\x00\x00\x0csome def"
            b"ault\x00",
            # Protocol.JSON: b'{"val":0,"val_list":[1,2,3,4],"name":"foo","an_int":{"tiny'
            # b'":1},"other":"some default"}',
        }
        self.thrift_serialization_round_robin(control, fixtures)

    def test_serialize_Integers_union(self) -> None:
        control = Integers(medium=1337)
        fixtures: Mapping[Protocol, bytes] = {
            Protocol.COMPACT: b"5\xf2\x14\x00",
            Protocol.BINARY: b"\x08\x00\x03\x00\x00\x059\x00",
            # Protocol.JSON: b'{"medium":1337}',
        }

        self.thrift_serialization_round_robin(control, fixtures)

    def test_deserialize_with_length(self) -> None:
        control = easy(val=5, val_list=[1, 2, 3, 4, 5])
        for proto in Protocol:
            encoded = serialize(control, protocol=proto)
            decoded, length = deserialize_with_length(
                type(control), encoded, protocol=proto
            )
            self.assertIsInstance(decoded, type(control))
            self.assertEqual(decoded, control)
            self.assertEqual(length, len(encoded))

    def test_string_with_non_utf8_data(self) -> None:
        encoded = b"\x0b\x00\x01\x00\x00\x00\x03foo\x00"
        sb = deserialize(StringBucket, encoded, protocol=Protocol.BINARY)
        self.assertEqual("foo", sb.one)

        encoded = b"\x0b\x00\x01\x00\x00\x00\x03\xfa\xf0\xef\x00"
        sb = deserialize(StringBucket, encoded, protocol=Protocol.BINARY)
        with self.assertRaises(UnicodeDecodeError):
            # Accessing the property is when the string is decoded as UTF-8.
            sb.one