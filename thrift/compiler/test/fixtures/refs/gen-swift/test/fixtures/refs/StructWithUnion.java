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
@ThriftStruct(value="StructWithUnion", builder=StructWithUnion.Builder.class)
public final class StructWithUnion {
    @ThriftConstructor
    public StructWithUnion(
        @ThriftField(value=1, name="u", requiredness=Requiredness.NONE) final test.fixtures.refs.MyUnion u,
        @ThriftField(value=2, name="aDouble", requiredness=Requiredness.NONE) final double aDouble,
        @ThriftField(value=3, name="f", requiredness=Requiredness.NONE) final test.fixtures.refs.MyField f
    ) {
        this.u = u;
        this.aDouble = aDouble;
        this.f = f;
    }
    
    @ThriftConstructor
    protected StructWithUnion() {
      this.u = null;
      this.aDouble = 0.;
      this.f = null;
    }
    
    public static class Builder {
        private test.fixtures.refs.MyUnion u = null;
        private double aDouble = 0.;
        private test.fixtures.refs.MyField f = null;
    
        @ThriftField(value=1, name="u", requiredness=Requiredness.NONE)
        public Builder setU(test.fixtures.refs.MyUnion u) {
            this.u = u;
            return this;
        }
        @ThriftField(value=2, name="aDouble", requiredness=Requiredness.NONE)
        public Builder setADouble(double aDouble) {
            this.aDouble = aDouble;
            return this;
        }
        @ThriftField(value=3, name="f", requiredness=Requiredness.NONE)
        public Builder setF(test.fixtures.refs.MyField f) {
            this.f = f;
            return this;
        }
    
        public Builder() { }
        public Builder(StructWithUnion other) {
            this.u = other.u;
            this.aDouble = other.aDouble;
            this.f = other.f;
        }
    
        @ThriftConstructor
        public StructWithUnion build() {
            return new StructWithUnion (
                this.u,
                this.aDouble,
                this.f
            );
        }
    }
    
    private static final TStruct STRUCT_DESC = new TStruct("StructWithUnion");
    private final test.fixtures.refs.MyUnion u;
    public static final int _U = 1;
    private static final TField U_FIELD_DESC = new TField("u", TType.STRUCT, (short)1);
    private final double aDouble;
    public static final int _ADOUBLE = 2;
    private static final TField A_DOUBLE_FIELD_DESC = new TField("aDouble", TType.DOUBLE, (short)2);
    private final test.fixtures.refs.MyField f;
    public static final int _F = 3;
    private static final TField F_FIELD_DESC = new TField("f", TType.STRUCT, (short)3);

    
    @ThriftField(value=1, name="u", requiredness=Requiredness.NONE)
    public test.fixtures.refs.MyUnion getU() { return u; }
        
    @ThriftField(value=2, name="aDouble", requiredness=Requiredness.NONE)
    public double getADouble() { return aDouble; }
        
    @ThriftField(value=3, name="f", requiredness=Requiredness.NONE)
    public test.fixtures.refs.MyField getF() { return f; }
    
    @Override
    public String toString() {
        ToStringHelper helper = toStringHelper(this);
        helper.add("u", u);
        helper.add("aDouble", aDouble);
        helper.add("f", f);
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
    
        StructWithUnion other = (StructWithUnion)o;
    
        return
            Objects.equals(u, other.u) &&
            Objects.equals(aDouble, other.aDouble) &&
            Objects.equals(f, other.f) &&
            true;
    }
    
    @Override
    public int hashCode() {
        return Arrays.deepHashCode(new Object[] {
            u,
            aDouble,
            f
        });
    }
    
    
      // Currently, the read0 method cannot read metadatamap for JSON styled serialization.
      // Perhaps, it will be implemented in the future!
    public static StructWithUnion read0(TProtocol oprot) throws TException {
      TField __field;
      oprot.readStructBegin();
      StructWithUnion.Builder builder = new StructWithUnion.Builder();
      while (true) {
        __field = oprot.readFieldBegin();
        if (__field.type == TType.STOP) { break; }
        switch (__field.id) {
        case _U:
          if (__field.type == TType.STRUCT) {
            test.fixtures.refs.MyUnion u = test.fixtures.refs.MyUnion.read0(oprot);
            builder.setU(u);
          } else {
            TProtocolUtil.skip(oprot, __field.type);
          }
          break;
        case _ADOUBLE:
          if (__field.type == TType.DOUBLE) {
            double aDouble = oprot.readDouble();
            builder.setADouble(aDouble);
          } else {
            TProtocolUtil.skip(oprot, __field.type);
          }
          break;
        case _F:
          if (__field.type == TType.STRUCT) {
            test.fixtures.refs.MyField f = test.fixtures.refs.MyField.read0(oprot);
            builder.setF(f);
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
      if (this.u != null) {
        oprot.writeFieldBegin(U_FIELD_DESC);
        this.u.write0(oprot);
        oprot.writeFieldEnd();
      }
      oprot.writeFieldBegin(A_DOUBLE_FIELD_DESC);
      oprot.writeDouble(this.aDouble);
      oprot.writeFieldEnd();
      if (this.f != null) {
        oprot.writeFieldBegin(F_FIELD_DESC);
        this.f.write0(oprot);
        oprot.writeFieldEnd();
      }
      oprot.writeFieldStop();
      oprot.writeStructEnd();
    }
    
}
