/**
 * Autogenerated by Thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated
 */

package test.fixtures.includes.transitive;

import com.facebook.swift.codec.*;
import com.facebook.swift.codec.ThriftField.Requiredness;
import com.facebook.swift.codec.ThriftField.Recursiveness;
import com.google.common.collect.*;
import java.util.*;
import org.apache.thrift.*;
import org.apache.thrift.async.*;
import org.apache.thrift.meta_data.*;
import org.apache.thrift.server.*;
import org.apache.thrift.transport.*;
import org.apache.thrift.protocol.*;
import org.apache.thrift.meta_data.FieldMetaData;
import org.apache.thrift.meta_data.FieldValueMetaData;
import static com.google.common.base.MoreObjects.toStringHelper;
import static com.google.common.base.MoreObjects.ToStringHelper;

@SwiftGenerated
@ThriftStruct(value="Foo", builder=Foo.Builder.class)
public final class Foo {
    @ThriftConstructor
    public Foo(
        @ThriftField(value=1, name="a", requiredness=Requiredness.NONE) final long a
    ) {
        this.a = a;
    }
    
    @ThriftConstructor
    protected Foo() {
      this.a = 0L;
    }
    
    public static class Builder {
        private long a = 2L;
    
        @ThriftField(value=1, name="a", requiredness=Requiredness.NONE)
        public Builder setA(long a) {
            this.a = a;
            return this;
        }
    
        public Builder() { }
        public Builder(Foo other) {
            this.a = other.a;
        }
    
        @ThriftConstructor
        public Foo build() {
            return new Foo (
                this.a
            );
        }
    }
    
    private static final TStruct STRUCT_DESC = new TStruct("Foo");
    private final long a;
    public static final int _A = 1;
    private static final TField A_FIELD_DESC = new TField("a", TType.I64, (short)1);

    
    @ThriftField(value=1, name="a", requiredness=Requiredness.NONE)
    public long getA() { return a; }
    
    @Override
    public String toString() {
        ToStringHelper helper = toStringHelper(this);
        helper.add("a", a);
        return helper.toString();
    }
    
    @Override
    public boolean equals(Object o) {
        if (this == o) {
            return true;
        }
        if (o == null || getClass() != o.getClass()) {
            return false;
        }
    
        Foo other = (Foo)o;
    
        return
            Objects.equals(a, other.a) &&
            true;
    }
    
    @Override
    public int hashCode() {
        return Arrays.deepHashCode(new Object[] {
            a
        });
    }
    
    
      // Currently, the read0 method cannot read metadatamap for JSON styled serialization.
      // Perhaps, it will be implemented in the future!
    public static Foo read0(TProtocol oprot) throws TException {
      TField __field;
      oprot.readStructBegin();
      Foo.Builder builder = new Foo.Builder();
      while (true) {
        __field = oprot.readFieldBegin();
        if (__field.type == TType.STOP) { break; }
        switch (__field.id) {
        case _A:
          if (__field.type == TType.I64) {
            long a = oprot.readI64();
            builder.setA(a);
          } else {
            TProtocolUtil.skip(oprot, __field.type);
          }
          break;
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
      oprot.writeFieldBegin(A_FIELD_DESC);
      oprot.writeI64(this.a);
      oprot.writeFieldEnd();
      oprot.writeFieldStop();
      oprot.writeStructEnd();
    }
    
}
