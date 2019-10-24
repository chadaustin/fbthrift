/*
 * Copyright (c) Facebook, Inc. and its affiliates.
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

#pragma once

#include <folly/portability/GMock.h>

#include <folly/io/async/ScopedEventBaseThread.h>
#include <thrift/lib/cpp2/transport/rsocket/test/util/gen-cpp2/StreamService.h>

namespace testutil {
namespace testservice {

class TestServiceMock : public StreamServiceSvIf {
 public:
  TestServiceMock() {}

  int32_t echo(int32_t value) override;

  apache::thrift::Stream<int32_t> range(int32_t from, int32_t to) override;
  apache::thrift::Stream<int32_t>
  slowRange(int32_t from, int32_t to, int32_t millis) override;
  apache::thrift::Stream<int32_t> slowCancellation() override;
  apache::thrift::Stream<Message> returnNullptr() override;
  apache::thrift::ResponseAndStream<int, Message> throwError() override;

  apache::thrift::ResponseAndStream<int32_t, int32_t> leakCheck(
      int32_t from,
      int32_t to) override;
  apache::thrift::ResponseAndStream<int32_t, int32_t>
  leakCheckWithSleep(int32_t from, int32_t to, int32_t sleepMs) override;
  int32_t instanceCount() override;

  apache::thrift::ResponseAndStream<int32_t, int32_t> sleepWithResponse(
      int32_t timeMs) override;

  apache::thrift::Stream<int32_t> sleepWithoutResponse(int32_t timeMs) override;

  apache::thrift::ResponseAndStream<int32_t, int32_t> streamServerSlow()
      override;

  void sendMessage(int32_t messageId, bool complete, bool error) override;
  apache::thrift::Stream<int32_t> registerToMessages() override;

  apache::thrift::Stream<Message> streamThrows(int32_t whichEx) override;
  apache::thrift::ResponseAndStream<int32_t, Message> responseAndStreamThrows(
      int32_t whichEx) override;

  apache::thrift::Stream<int32_t> requestWithBlob(
      std::unique_ptr<folly::IOBuf> val) override;

 protected:
  folly::ScopedEventBaseThread executor_;

  std::unique_ptr<apache::thrift::StreamPublisher<int32_t>> messages_;
};

} // namespace testservice
} // namespace testutil
