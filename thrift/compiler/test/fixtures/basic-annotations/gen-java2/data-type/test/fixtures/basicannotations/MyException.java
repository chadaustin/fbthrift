/**
 * Autogenerated by Thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated
 */

package test.fixtures.basicannotations;

import com.facebook.swift.codec.*;
import com.facebook.swift.codec.ThriftField.Requiredness;
import com.facebook.swift.codec.ThriftField.Recursiveness;
import java.util.*;
import javax.annotation.Nullable;
import org.apache.thrift.*;
import org.apache.thrift.transport.*;
import org.apache.thrift.protocol.*;

@SwiftGenerated
@com.facebook.swift.codec.ThriftStruct("MyException")
public final class MyException extends org.apache.thrift.TBaseException implements com.facebook.thrift.payload.ThriftSerializable {
    private static final long serialVersionUID = 1L;


    public static final Map<String, Integer> NAMES_TO_IDS = new HashMap();
    public static final Map<Integer, TField> FIELD_METADATA = new HashMap<>();

    private static final TStruct STRUCT_DESC = new TStruct("MyException");

    @ThriftConstructor
    public MyException(
    ) {
    }
    
    
    
    public static class Builder {
    
    
    public Builder() { }
        public Builder(MyException other) {
        }
    
        @ThriftConstructor
        public MyException build() {
            MyException result = new MyException (
            );
            return result;
        }
    }
    

    
    public static com.facebook.thrift.payload.Reader<MyException> asReader() {
      return MyException::read0;
    }
    
    public static MyException read0(TProtocol oprot) throws TException {
      TField __field;
      oprot.readStructBegin(MyException.NAMES_TO_IDS, MyException.FIELD_METADATA);
      MyException.Builder builder = new MyException.Builder();
      while (true) {
        __field = oprot.readFieldBegin();
        if (__field.type == TType.STOP) { break; }
        switch (__field.id) {
        default:
          TProtocolUtil.skip(oprot, __field.type);
          break;
        }
        oprot.readFieldEnd();
      }
      oprot.readStructEnd();
      return builder.build();
    }
    
    public void write0(TProtocol oprot) throws TException {
      oprot.writeStructBegin(STRUCT_DESC);
      oprot.writeFieldStop();
      oprot.writeStructEnd();
    }
    
}
