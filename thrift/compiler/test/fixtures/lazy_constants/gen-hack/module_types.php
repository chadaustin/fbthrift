<?hh // strict
/**
 * Autogenerated by Thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated
 */

namespace test\fixtures\lazy_constants;

/**
 * Original thrift enum:-
 * City
 */
enum City: int {
  NYC = 0;
  MPK = 1;
  SEA = 2;
  LON = 3;
}

/**
 * Original thrift enum:-
 * Company
 */
enum Company: int {
  FACEBOOK = 0;
  WHATSAPP = 1;
  OCULUS = 2;
  INSTAGRAM = 3;
}

/**
 * Original thrift struct:-
 * Internship
 */
class Internship implements \IThriftStruct {
  use \ThriftSerializationTrait;

  const dict<int, this::TFieldSpec> SPEC = dict[
    1 => shape(
      'var' => 'weeks',
      'type' => \TType::I32,
    ),
    2 => shape(
      'var' => 'title',
      'type' => \TType::STRING,
    ),
    3 => shape(
      'var' => 'employer',
      'type' => \TType::I32,
      'enum' => \test\fixtures\lazy_constants\Company::class,
    ),
  ];
  const dict<string, int> FIELDMAP = dict[
    'weeks' => 1,
    'title' => 2,
    'employer' => 3,
  ];

  const type TConstructorShape = shape(
    ?'weeks' => int,
    ?'title' => string,
    ?'employer' => ?\test\fixtures\lazy_constants\Company,
  );

  const int STRUCTURAL_ID = 749038867953722654;
  /**
   * Original thrift field:-
   * 1: i32 weeks
   */
  public int $weeks;
  /**
   * Original thrift field:-
   * 2: string title
   */
  public string $title;
  /**
   * Original thrift field:-
   * 3: enum module.Company employer
   */
  public ?\test\fixtures\lazy_constants\Company $employer;

  <<__Rx>>
  public function __construct(?int $weeks = null, ?string $title = null, ?\test\fixtures\lazy_constants\Company $employer = null  ) {
    $this->weeks = $weeks ?? 0;
    $this->title = $title ?? '';
    $this->employer = $employer;
  }

  <<__Rx>>
  public static function fromShape(self::TConstructorShape $shape = shape()): this {
    return new static(
      Shapes::idx($shape, 'weeks'),
      Shapes::idx($shape, 'title'),
      Shapes::idx($shape, 'employer'),
    );
  }

  public function getName(): string {
    return 'Internship';
  }

  public static function getAllStructuredAnnotations(): \TStructAnnotations {
    return shape(
      'struct' => dict[],
      'fields' => dict[
        'weeks' => shape(
          'field' => dict[],
          'type' => dict[],
        ),
        'title' => shape(
          'field' => dict[],
          'type' => dict[],
        ),
        'employer' => shape(
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

/**
 * Original thrift struct:-
 * Range
 */
class Range implements \IThriftStruct {
  use \ThriftSerializationTrait;

  const dict<int, this::TFieldSpec> SPEC = dict[
    1 => shape(
      'var' => 'min',
      'type' => \TType::I32,
    ),
    2 => shape(
      'var' => 'max',
      'type' => \TType::I32,
    ),
  ];
  const dict<string, int> FIELDMAP = dict[
    'min' => 1,
    'max' => 2,
  ];

  const type TConstructorShape = shape(
    ?'min' => int,
    ?'max' => int,
  );

  const int STRUCTURAL_ID = 6850388386457434767;
  /**
   * Original thrift field:-
   * 1: i32 min
   */
  public int $min;
  /**
   * Original thrift field:-
   * 2: i32 max
   */
  public int $max;

  <<__Rx>>
  public function __construct(?int $min = null, ?int $max = null  ) {
    $this->min = $min ?? 0;
    $this->max = $max ?? 0;
  }

  <<__Rx>>
  public static function fromShape(self::TConstructorShape $shape = shape()): this {
    return new static(
      Shapes::idx($shape, 'min'),
      Shapes::idx($shape, 'max'),
    );
  }

  public function getName(): string {
    return 'Range';
  }

  public static function getAllStructuredAnnotations(): \TStructAnnotations {
    return shape(
      'struct' => dict[],
      'fields' => dict[
        'min' => shape(
          'field' => dict[],
          'type' => dict[],
        ),
        'max' => shape(
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

