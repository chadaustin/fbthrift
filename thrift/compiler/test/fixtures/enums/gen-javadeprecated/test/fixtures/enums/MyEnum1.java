/**
 * Autogenerated by Thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated
 */
package test.fixtures.enums;


import com.facebook.thrift.IntRangeSet;
import java.util.Map;
import java.util.HashMap;

@SuppressWarnings({ "unused" })
public enum MyEnum1 implements com.facebook.thrift.TEnum {
  ME1_1(1),
  ME1_2(2),
  ME1_3(3),
  ME1_5(5),
  ME1_6(6),
  ME1_0(0);

  private final int value;

  private MyEnum1(int value) {
    this.value = value;
  }

  /**
   * Get the integer value of this enum value, as defined in the Thrift IDL.
   */
  public int getValue() {
    return value;
  }

  /**
   * Find a the enum type by its integer value, as defined in the Thrift IDL.
   * @return null if the value is not found.
   */
  public static MyEnum1 findByValue(int value) { 
    switch (value) {
      case 1:
        return ME1_1;
      case 2:
        return ME1_2;
      case 3:
        return ME1_3;
      case 5:
        return ME1_5;
      case 6:
        return ME1_6;
      case 0:
        return ME1_0;
      default:
        return null;
    }
  }
}