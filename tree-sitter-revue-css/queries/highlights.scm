; Tree-sitter highlight queries for Revue CSS

; Comments
(comment) @comment

; Selectors
(universal_selector) @operator
(type_selector (identifier) @type)
(class_selector "." @punctuation.delimiter)
(class_selector (identifier) @attribute)
(id_selector "#" @punctuation.delimiter)
(id_selector (identifier) @label)
(pseudo_class ":" @punctuation.delimiter)
(pseudo_class (identifier) @function.builtin)
(pseudo_element "::" @punctuation.delimiter)
(pseudo_element (identifier) @function.builtin)
(combinator) @operator

; Attribute selectors
(attribute_selector "[" @punctuation.bracket)
(attribute_selector "]" @punctuation.bracket)
(attribute_selector (identifier) @attribute)

; Properties
(declaration (property (identifier)) @property)
(declaration (property (variable_name)) @variable.parameter)

; Values
(declaration (identifier) @constant)
(number) @number
(unit) @type
(string) @string
(hex_color) @constant.numeric
(named_color) @constant.builtin

; CSS Variables
(variable_name) @variable.parameter
(variable_ref "var" @function.builtin)
(variable_ref (variable_name) @variable.parameter)

; Functions
(function_call (identifier) @function)

; Keywords
(important) @keyword

; At-rules
"@keyframes" @keyword
"@media" @keyword
"@import" @keyword
"from" @constant.builtin
"to" @constant.builtin

; Keyframes
(keyframes (identifier) @function)

; Punctuation
"{" @punctuation.bracket
"}" @punctuation.bracket
"(" @punctuation.bracket
")" @punctuation.bracket
":" @punctuation.delimiter
";" @punctuation.delimiter
"," @punctuation.delimiter
