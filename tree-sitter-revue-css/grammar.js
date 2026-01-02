/// Tree-sitter grammar for Revue CSS
/// Supports standard CSS + Revue-specific extensions

module.exports = grammar({
  name: 'revue_css',

  extras: $ => [
    /\s/,
    $.comment,
  ],

  rules: {
    // Entry point
    stylesheet: $ => repeat($._rule),

    _rule: $ => choice(
      $.rule_set,
      $.at_rule,
    ),

    // Comments
    comment: $ => token(choice(
      seq('/*', /[^*]*\*+([^/*][^*]*\*+)*/, '/'),
      seq('//', /[^\n]*/),
    )),

    // Rule set: selector { declarations }
    rule_set: $ => seq(
      $.selectors,
      $.block,
    ),

    selectors: $ => sep1($.selector, ','),

    selector: $ => repeat1($._selector_part),

    _selector_part: $ => choice(
      $.universal_selector,
      $.type_selector,
      $.class_selector,
      $.id_selector,
      $.attribute_selector,
      $.pseudo_class,
      $.pseudo_element,
      $.combinator,
    ),

    universal_selector: $ => '*',

    type_selector: $ => $.identifier,

    class_selector: $ => seq('.', $.identifier),

    id_selector: $ => seq('#', $.identifier),

    attribute_selector: $ => seq(
      '[',
      $.identifier,
      optional(seq(
        choice('=', '~=', '|=', '^=', '$=', '*='),
        choice($.string, $.identifier),
      )),
      ']',
    ),

    pseudo_class: $ => seq(
      ':',
      $.identifier,
      optional(seq('(', $._value, ')')),
    ),

    pseudo_element: $ => seq('::', $.identifier),

    combinator: $ => choice('>', '+', '~'),

    // Block: { declarations }
    block: $ => seq(
      '{',
      repeat($.declaration),
      '}',
    ),

    // Declaration: property: value;
    declaration: $ => seq(
      $.property,
      ':',
      $._value,
      optional($.important),
      ';',
    ),

    property: $ => choice(
      $.identifier,
      $.variable_name,
    ),

    // CSS variable name: --name
    variable_name: $ => /--[a-zA-Z_][a-zA-Z0-9_-]*/,

    _value: $ => repeat1($._value_part),

    _value_part: $ => choice(
      $.identifier,
      $.number,
      $.string,
      $.color,
      $.variable_ref,
      $.function_call,
      $.operator,
    ),

    // var(--name) or var(--name, fallback)
    variable_ref: $ => seq(
      'var',
      '(',
      $.variable_name,
      optional(seq(',', $._value)),
      ')',
    ),

    // Function call: name(args)
    function_call: $ => seq(
      $.identifier,
      '(',
      optional(sep1($._value, ',')),
      ')',
    ),

    // Color values
    color: $ => choice(
      $.hex_color,
      $.named_color,
    ),

    hex_color: $ => /#[0-9a-fA-F]{3,8}/,

    named_color: $ => choice(
      'transparent', 'currentColor',
      // Basic colors
      'black', 'white', 'red', 'green', 'blue', 'yellow',
      'cyan', 'magenta', 'orange', 'purple', 'pink', 'gray', 'grey',
    ),

    // Numbers with optional units
    number: $ => seq(
      /[+-]?[0-9]*\.?[0-9]+/,
      optional($.unit),
    ),

    unit: $ => choice(
      // Length
      'px', 'em', 'rem', '%', 'vh', 'vw', 'vmin', 'vmax',
      'ch', 'ex', 'cm', 'mm', 'in', 'pt', 'pc',
      // Time
      's', 'ms',
      // Angle
      'deg', 'rad', 'turn',
    ),

    operator: $ => choice('/', ','),

    important: $ => '!important',

    // At-rules
    at_rule: $ => choice(
      $.keyframes,
      $.media_query,
      $.import_rule,
    ),

    keyframes: $ => seq(
      '@keyframes',
      $.identifier,
      '{',
      repeat($.keyframe),
      '}',
    ),

    keyframe: $ => seq(
      $.keyframe_selector,
      $.block,
    ),

    keyframe_selector: $ => choice(
      'from',
      'to',
      seq($.number, '%'),
    ),

    media_query: $ => seq(
      '@media',
      $.media_condition,
      $.block,
    ),

    media_condition: $ => /[^{]+/,

    import_rule: $ => seq(
      '@import',
      $.string,
      ';',
    ),

    // Primitives
    identifier: $ => /[a-zA-Z_][a-zA-Z0-9_-]*/,

    string: $ => choice(
      seq('"', /[^"]*/, '"'),
      seq("'", /[^']*/, "'"),
    ),
  },
});

// Helper: separated by
function sep1(rule, separator) {
  return seq(rule, repeat(seq(separator, rule)));
}
