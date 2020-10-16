<?hh // strict
/**
 * Autogenerated by Thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated
 */

namespace hack_ns2;

/**
 * Original thrift service:-
 * TestService
 */
interface TestServiceAsyncIf extends \foo\hack_ns\FooHackServiceAsyncIf {
  /**
   * Original thrift definition:-
   * i32
   *   ping(1: string str_arg);
   */
  public function ping(string $str_arg): Awaitable<int>;
}

/**
 * Original thrift service:-
 * TestService
 */
interface TestServiceIf extends \foo\hack_ns\FooHackServiceIf {
  /**
   * Original thrift definition:-
   * i32
   *   ping(1: string str_arg);
   */
  public function ping(string $str_arg): int;
}

/**
 * Original thrift service:-
 * TestService
 */
interface TestServiceClientIf extends \foo\hack_ns\FooHackServiceClientIf {
  /**
   * Original thrift definition:-
   * i32
   *   ping(1: string str_arg);
   */
  public function ping(string $str_arg): Awaitable<int>;
}

/**
 * Original thrift service:-
 * TestService
 */
interface TestServiceAsyncRpcOptionsIf extends \foo\hack_ns\FooHackServiceAsyncRpcOptionsIf {
  /**
   * Original thrift definition:-
   * i32
   *   ping(1: string str_arg);
   */
  public function ping(\RpcOptions $rpc_options, string $str_arg): Awaitable<int>;
}

/**
 * Original thrift service:-
 * TestService
 */
trait TestServiceClientBase {
  require extends \ThriftClientBase;

  protected function sendImpl_ping(string $str_arg): int {
    $currentseqid = $this->getNextSequenceID();
    $args = new \hack_ns2\TestService_ping_args(
      $str_arg,
    );
    try {
      $this->eventHandler_->preSend('ping', $args, $currentseqid);
      if ($this->output_ is \TBinaryProtocolAccelerated)
      {
        \thrift_protocol_write_binary($this->output_, 'ping', \TMessageType::CALL, $args, $currentseqid, $this->output_->isStrictWrite(), false);
      }
      else if ($this->output_ is \TCompactProtocolAccelerated)
      {
        \thrift_protocol_write_compact($this->output_, 'ping', \TMessageType::CALL, $args, $currentseqid, false);
      }
      else
      {
        $this->output_->writeMessageBegin('ping', \TMessageType::CALL, $currentseqid);
        $args->write($this->output_);
        $this->output_->writeMessageEnd();
        $this->output_->getTransport()->flush();
      }
    } catch (\THandlerShortCircuitException $ex) {
      switch ($ex->resultType) {
        case \THandlerShortCircuitException::R_EXPECTED_EX:
        case \THandlerShortCircuitException::R_UNEXPECTED_EX:
          $this->eventHandler_->sendError('ping', $args, $currentseqid, $ex->result);
          throw $ex->result;
        case \THandlerShortCircuitException::R_SUCCESS:
        default:
          $this->eventHandler_->postSend('ping', $args, $currentseqid);
          return $currentseqid;
      }
    } catch (\Exception $ex) {
      $this->eventHandler_->sendError('ping', $args, $currentseqid, $ex);
      throw $ex;
    }
    $this->eventHandler_->postSend('ping', $args, $currentseqid);
    return $currentseqid;
  }

  protected function recvImpl_ping(?int $expectedsequenceid = null): int {
    try {
      $this->eventHandler_->preRecv('ping', $expectedsequenceid);
      if ($this->input_ is \TBinaryProtocolAccelerated) {
        $result = \thrift_protocol_read_binary($this->input_, '\hack_ns2\TestService_ping_result', $this->input_->isStrictRead());
      } else if ($this->input_ is \TCompactProtocolAccelerated)
      {
        $result = \thrift_protocol_read_compact($this->input_, '\hack_ns2\TestService_ping_result');
      }
      else
      {
        $rseqid = 0;
        $fname = '';
        $mtype = 0;

        $this->input_->readMessageBegin(
          inout $fname,
          inout $mtype,
          inout $rseqid,
        );
        if ($mtype == \TMessageType::EXCEPTION) {
          $x = new \TApplicationException();
          $x->read($this->input_);
          $this->input_->readMessageEnd();
          throw $x;
        }
        $result = new \hack_ns2\TestService_ping_result();
        $result->read($this->input_);
        $this->input_->readMessageEnd();
        if ($expectedsequenceid !== null && ($rseqid != $expectedsequenceid)) {
          throw new \TProtocolException("ping failed: sequence id is out of order");
        }
      }
    } catch (\THandlerShortCircuitException $ex) {
      switch ($ex->resultType) {
        case \THandlerShortCircuitException::R_EXPECTED_EX:
          $this->eventHandler_->recvException('ping', $expectedsequenceid, $ex->result);
          throw $ex->result;
        case \THandlerShortCircuitException::R_UNEXPECTED_EX:
          $this->eventHandler_->recvError('ping', $expectedsequenceid, $ex->result);
          throw $ex->result;
        case \THandlerShortCircuitException::R_SUCCESS:
        default:
          $this->eventHandler_->postRecv('ping', $expectedsequenceid, $ex->result);
          return $ex->result;
      }
    } catch (\Exception $ex) {
      $this->eventHandler_->recvError('ping', $expectedsequenceid, $ex);
      throw $ex;
    }
    if ($result->success !== null) {
      $success = $result->success;
      $this->eventHandler_->postRecv('ping', $expectedsequenceid, $success);
      return $success;
    }
    $x = new \TApplicationException("ping failed: unknown result", \TApplicationException::MISSING_RESULT);
    $this->eventHandler_->recvError('ping', $expectedsequenceid, $x);
    throw $x;
  }

}

class TestServiceAsyncClient extends \foo\hack_ns\FooHackServiceAsyncClient implements TestServiceAsyncIf {
  use TestServiceClientBase;

  /**
   * Original thrift definition:-
   * i32
   *   ping(1: string str_arg);
   */
  public async function ping(string $str_arg): Awaitable<int> {
    await $this->asyncHandler_->genBefore("TestService", "ping");
    $currentseqid = $this->sendImpl_ping($str_arg);
    $channel = $this->channel_;
    $out_transport = $this->output_->getTransport();
    $in_transport = $this->input_->getTransport();
    if ($channel !== null && $out_transport is \TMemoryBuffer && $in_transport is \TMemoryBuffer) {
      $msg = $out_transport->getBuffer();
      $out_transport->resetBuffer();
      list($result_msg, $_read_headers) = await $channel->genSendRequestResponse(new \RpcOptions(), $msg);
      $in_transport->resetBuffer();
      $in_transport->write($result_msg);
    } else {
      await $this->asyncHandler_->genWait($currentseqid);
    }
    return $this->recvImpl_ping($currentseqid);
  }

}

class TestServiceClient extends \foo\hack_ns\FooHackServiceClient implements TestServiceClientIf {
  use TestServiceClientBase;

  /**
   * Original thrift definition:-
   * i32
   *   ping(1: string str_arg);
   */
  public async function ping(string $str_arg): Awaitable<int> {
    await $this->asyncHandler_->genBefore("TestService", "ping");
    $currentseqid = $this->sendImpl_ping($str_arg);
    $channel = $this->channel_;
    $out_transport = $this->output_->getTransport();
    $in_transport = $this->input_->getTransport();
    if ($channel !== null && $out_transport is \TMemoryBuffer && $in_transport is \TMemoryBuffer) {
      $msg = $out_transport->getBuffer();
      $out_transport->resetBuffer();
      list($result_msg, $_read_headers) = await $channel->genSendRequestResponse(new \RpcOptions(), $msg);
      $in_transport->resetBuffer();
      $in_transport->write($result_msg);
    } else {
      await $this->asyncHandler_->genWait($currentseqid);
    }
    return $this->recvImpl_ping($currentseqid);
  }

  /* send and recv functions */
  public function send_ping(string $str_arg): int {
    return $this->sendImpl_ping($str_arg);
  }
  public function recv_ping(?int $expectedsequenceid = null): int {
    return $this->recvImpl_ping($expectedsequenceid);
  }
}

class TestServiceAsyncRpcOptionsClient extends \foo\hack_ns\FooHackServiceAsyncRpcOptionsClient implements TestServiceAsyncRpcOptionsIf {
  use TestServiceClientBase;

  /**
   * Original thrift definition:-
   * i32
   *   ping(1: string str_arg);
   */
  public async function ping(\RpcOptions $rpc_options, string $str_arg): Awaitable<int> {
    await $this->asyncHandler_->genBefore("TestService", "ping");
    $currentseqid = $this->sendImpl_ping($str_arg);
    $channel = $this->channel_;
    $out_transport = $this->output_->getTransport();
    $in_transport = $this->input_->getTransport();
    if ($channel !== null && $out_transport is \TMemoryBuffer && $in_transport is \TMemoryBuffer) {
      $msg = $out_transport->getBuffer();
      $out_transport->resetBuffer();
      list($result_msg, $_read_headers) = await $channel->genSendRequestResponse($rpc_options, $msg);
      $in_transport->resetBuffer();
      $in_transport->write($result_msg);
    } else {
      await $this->asyncHandler_->genWait($currentseqid);
    }
    return $this->recvImpl_ping($currentseqid);
  }

}

// HELPER FUNCTIONS AND STRUCTURES

class TestService_ping_args implements \IThriftStruct {
  use \ThriftSerializationTrait;

  const dict<int, this::TFieldSpec> SPEC = dict[
    1 => shape(
      'var' => 'str_arg',
      'type' => \TType::STRING,
    ),
  ];
  const dict<string, int> FIELDMAP = dict[
    'str_arg' => 1,
  ];

  const type TConstructorShape = shape(
    ?'str_arg' => string,
  );

  const int STRUCTURAL_ID = 5542358918184482101;
  public string $str_arg;

  <<__Rx>>
  public function __construct(?string $str_arg = null  ) {
    $this->str_arg = $str_arg ?? '';
  }

  <<__Rx>>
  public static function fromShape(self::TConstructorShape $shape = shape()): this {
    return new static(
      Shapes::idx($shape, 'str_arg'),
    );
  }

  public function getName(): string {
    return 'TestService_ping_args';
  }

  public static function getAllStructuredAnnotations(): \TStructAnnotations {
    return shape(
      'struct' => dict[],
      'fields' => dict[
        'str_arg' => shape(
          'field' => dict[],
          'type' => dict[],
        ),
      ],
    );
  }

  public static function getAnnotations(): darray<string, mixed> {
    return darray[
    ];
  }

}

class TestService_ping_result implements \IThriftStruct {
  use \ThriftSerializationTrait;

  const dict<int, this::TFieldSpec> SPEC = dict[
    0 => shape(
      'var' => 'success',
      'type' => \TType::I32,
    ),
  ];
  const dict<string, int> FIELDMAP = dict[
    'success' => 0,
  ];

  const type TConstructorShape = shape(
    ?'success' => int,
  );

  const int STRUCTURAL_ID = 3865318819874171525;
  public ?int $success;

  <<__Rx>>
  public function __construct(?int $success = null  ) {
  }

  <<__Rx>>
  public static function fromShape(self::TConstructorShape $shape = shape()): this {
    return new static(
      Shapes::idx($shape, 'success'),
    );
  }

  public function getName(): string {
    return 'TestService_ping_result';
  }

  public static function getAllStructuredAnnotations(): \TStructAnnotations {
    return shape(
      'struct' => dict[],
      'fields' => dict[
        'success' => shape(
          'field' => dict[],
          'type' => dict[],
        ),
      ],
    );
  }

  public static function getAnnotations(): darray<string, mixed> {
    return darray[
    ];
  }

}

class TestServiceStaticMetadata implements \IThriftServiceStaticMetadata {
  public static function getAllStructuredAnnotations(): \TServiceAnnotations {
    return shape(
      'service' => dict[],
      'functions' => dict[
        'ping' => dict[],
      ],
    );
  }
}

