/**
 * Autogenerated by Thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated
 */

package test.fixtures.refs;

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
@ThriftStruct(value="StructWithRefAndAnnotCppNoexceptMoveCtor", builder=StructWithRefAndAnnotCppNoexceptMoveCtor.Builder.class)
public final class StructWithRefAndAnnotCppNoexceptMoveCtor {
    @ThriftConstructor
    public StructWithRefAndAnnotCppNoexceptMoveCtor(
        @ThriftField(value=1, name="def_field", requiredness=Requiredness.NONE) final test.fixtures.refs.Empty defField
    ) {
        this.defField = defField;
    }
    
    @ThriftConstructor
    protected StructWithRefAndAnnotCppNoexceptMoveCtor() {
      this.defField = null;
    }
    
    public static class Builder {
        private test.fixtures.refs.Empty defField = null;
    
        @ThriftField(value=1, name="def_field", requiredness=Requiredness.NONE)
        public Builder setDefField(test.fixtures.refs.Empty defField) {
            this.defField = defField;
            return this;
        }
    
        public Builder() { }
        public Builder(StructWithRefAndAnnotCppNoexceptMoveCtor other) {
            this.defField = other.defField;
        }
    
        @ThriftConstructor
        public StructWithRefAndAnnotCppNoexceptMoveCtor build() {
            return new StructWithRefAndAnnotCppNoexceptMoveCtor (
                this.defField
            );
        }
    }
    
    private static final TStruct STRUCT_DESC = new TStruct("StructWithRefAndAnnotCppNoexceptMoveCtor");
    private final test.fixtures.refs.Empty defField;
    public static final int _DEF_FIELD = 1;
    private static final TField DEF_FIELD_FIELD_DESC = new TField("defField", TType.STRUCT, (short)1);

    
    @ThriftField(value=1, name="def_field", requiredness=Requiredness.NONE)
    public test.fixtures.refs.Empty getDefField() { return defField; }
    
    @Override
    public String toString() {
        ToStringHelper helper = toStringHelper(this);
        helper.add("defField", defField);
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
    
        StructWithRefAndAnnotCppNoexceptMoveCtor other = (StructWithRefAndAnnotCppNoexceptMoveCtor)o;
    
        return
            Objects.equals(defField, other.defField) &&
            true;
    }
    
    @Override
    public int hashCode() {
        return Arrays.deepHashCode(new Object[] {
            defField
        });
    }
    
    
      // Currently, the read0 method cannot read metadatamap for JSON styled serialization.
      // Perhaps, it will be implemented in the future!
    public static StructWithRefAndAnnotCppNoexceptMoveCtor read0(TProtocol oprot) throws TException {
      TField __field;
      oprot.readStructBegin();
      StructWithRefAndAnnotCppNoexceptMoveCtor.Builder builder = new StructWithRefAndAnnotCppNoexceptMoveCtor.Builder();
      while (true) {
        __field = oprot.readFieldBegin();
        if (__field.type == TType.STOP) { break; }
        switch (__field.id) {
        case _DEF_FIELD:
          if (__field.type == TType.STRUCT) {
            test.fixtures.refs.Empty defField = test.fixtures.refs.Empty.read0(oprot);
            builder.setDefField(defField);
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
      if (this.defField != null) {
        oprot.writeFieldBegin(DEF_FIELD_FIELD_DESC);
        this.defField.write0(oprot);
        oprot.writeFieldEnd();
      }
      oprot.writeFieldStop();
      oprot.writeStructEnd();
    }
    
}
