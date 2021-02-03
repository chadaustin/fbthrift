<?hh // strict
/**
 * Autogenerated by Thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated
 */

/**
 * Original thrift struct:-
 * Foo
 */
class Foo implements \IThriftStruct {
  use \ThriftSerializationTrait;

  const dict<int, this::TFieldSpec> SPEC = dict[
    1 => shape(
      'var' => 'a',
      'type' => \TType::I64,
    ),
    2 => shape(
      'var' => 'b',
      'type' => \TType::STRING,
    ),
  ];
  const dict<string, int> FIELDMAP = dict[
    'a' => 1,
    'b' => 2,
  ];

  const type TConstructorShape = shape(
    ?'a' => ?int,
    ?'b' => ?string,
  );

  const int STRUCTURAL_ID = 5526046231048010857;
  /**
   * Original thrift field:-
   * 1: i64 a
   */
  public int $a;
  /**
   * Original thrift field:-
   * 2: string b
   */
  public string $b;

  <<__Rx>>
  public function __construct(?int $a = null, ?string $b = null  )[] {
    $this->a = $a ?? 0;
    $this->b = $b ?? '';
  }

  <<__Rx, __MutableReturn>>
  public static function withDefaultValues()[]: this {
    return new static();
  }

  <<__Rx, __MutableReturn>>
  public static function fromShape(self::TConstructorShape $shape)[]: this {
    return new static(
      Shapes::idx($shape, 'a'),
      Shapes::idx($shape, 'b'),
    );
  }

  <<__Rx, __MutableReturn>>
  public static function fromMap_DEPRECATED(<<__Soft>> KeyedContainer<string, mixed> $map)[]: this {
    return new static(
      /* HH_FIXME[4110] For backwards compatibility with map's mixed values. */
      idx($map, 'a'),
      /* HH_FIXME[4110] For backwards compatibility with map's mixed values. */
      idx($map, 'b'),
    );
  }

  public function getName(): string {
    return 'Foo';
  }

  public static function getAllStructuredAnnotations(): \TStructAnnotations {
    return shape(
      'struct' => dict[],
      'fields' => dict[
      ],
    );
  }

}

