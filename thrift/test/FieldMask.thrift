/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package "apache.org/thrift/test"
include "thrift/annotation/cpp.thrift"
cpp_include "thrift/test/AdapterTest.h"

struct Foo {
  1: i32 field1;
  3: i32 field2;
}

struct Bar {
  2: list<Foo> foos;
  1: Foo foo;
}

struct Baz {
  @cpp.Adapter{name = "TemplatedTestAdapter"}
  1: Foo foo;
}

struct Foo2 {
  1: optional i32 field_1;
  2: optional i32 field_2;
}

struct Bar2 {
  1: optional Foo2 field_3;
  2: string field_4;
}