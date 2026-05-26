pub const TS_HIGHLIGHTS: &str = r#"
(comment) @comment
(string) @string
(template_string) @string
(number) @number
(true) @constant.builtin
(false) @constant.builtin
(null) @constant.builtin
(undefined) @constant.builtin
(regex) @string.special

["const" "let" "var" "function" "class" "interface" "type" "enum" "namespace" "module" "declare" "implements" "extends" "public" "private" "protected" "readonly" "static" "abstract" "async" "await" "return" "if" "else" "for" "while" "do" "switch" "case" "default" "break" "continue" "try" "catch" "finally" "throw" "new" "delete" "typeof" "instanceof" "in" "of" "as" "is" "import" "export" "from" "default" "void"] @keyword

(type_identifier) @type
(predefined_type) @type.builtin

(function_declaration name: (identifier) @function)
(method_definition name: (property_identifier) @function.method)
(call_expression function: (identifier) @function)
(call_expression function: (member_expression property: (property_identifier) @function.method))
(arrow_function) @function

(property_identifier) @property
(shorthand_property_identifier) @property
(shorthand_property_identifier_pattern) @property

["(" ")" "[" "]" "{" "}"] @punctuation.bracket
["." "," ";" ":"] @punctuation.delimiter
"#;

pub const TSX_HIGHLIGHTS: &str = r#"
(comment) @comment
(string) @string
(template_string) @string
(number) @number
(true) @constant.builtin
(false) @constant.builtin
(null) @constant.builtin
(undefined) @constant.builtin
(regex) @string.special

["const" "let" "var" "function" "class" "interface" "type" "enum" "namespace" "module" "declare" "implements" "extends" "public" "private" "protected" "readonly" "static" "abstract" "async" "await" "return" "if" "else" "for" "while" "do" "switch" "case" "default" "break" "continue" "try" "catch" "finally" "throw" "new" "delete" "typeof" "instanceof" "in" "of" "as" "is" "import" "export" "from" "default" "void"] @keyword

(type_identifier) @type
(predefined_type) @type.builtin

(function_declaration name: (identifier) @function)
(method_definition name: (property_identifier) @function.method)
(call_expression function: (identifier) @function)
(call_expression function: (member_expression property: (property_identifier) @function.method))
(arrow_function) @function

(property_identifier) @property
(shorthand_property_identifier) @property
(shorthand_property_identifier_pattern) @property

(jsx_element open_tag: (jsx_opening_element name: (identifier) @tag))
(jsx_element close_tag: (jsx_closing_element name: (identifier) @tag))
(jsx_self_closing_element name: (identifier) @tag)
(jsx_attribute (property_identifier) @attribute)

["(" ")" "[" "]" "{" "}"] @punctuation.bracket
["." "," ";" ":"] @punctuation.delimiter
"#;

pub const JS_HIGHLIGHTS: &str = r#"
(comment) @comment
(string) @string
(template_string) @string
(number) @number
(true) @constant.builtin
(false) @constant.builtin
(null) @constant.builtin
(undefined) @constant.builtin
(regex) @string.special

["const" "let" "var" "function" "class" "extends" "async" "await" "return" "if" "else" "for" "while" "do" "switch" "case" "default" "break" "continue" "try" "catch" "finally" "throw" "new" "delete" "typeof" "instanceof" "in" "of" "import" "export" "from" "default" "void"] @keyword

(function_declaration name: (identifier) @function)
(method_definition name: (property_identifier) @function.method)
(call_expression function: (identifier) @function)
(call_expression function: (member_expression property: (property_identifier) @function.method))
(arrow_function) @function

(property_identifier) @property
(shorthand_property_identifier) @property

(jsx_element open_tag: (jsx_opening_element name: (identifier) @tag))
(jsx_element close_tag: (jsx_closing_element name: (identifier) @tag))
(jsx_self_closing_element name: (identifier) @tag)
(jsx_attribute (property_identifier) @attribute)

["(" ")" "[" "]" "{" "}"] @punctuation.bracket
["." "," ";" ":"] @punctuation.delimiter
"#;

pub const RUST_HIGHLIGHTS: &str = r#"
; Comments
; Regular comments (line_comment and block_comment capture the entire comment)
(line_comment) @comment
(block_comment) @comment
; Doc comment parts need explicit captures to prevent operator conflicts
; The "/" in "///" and "!" in "//!" would otherwise match operator patterns
(outer_doc_comment_marker) @comment
(inner_doc_comment_marker) @comment
(doc_comment) @comment

; Strings and literals
(string_literal) @string
(raw_string_literal) @string
(char_literal) @string
(integer_literal) @number
(float_literal) @number
(boolean_literal) @constant.builtin

; Types
(type_identifier) @type
(primitive_type) @type.builtin

; Functions
(function_item (identifier) @function)
(function_signature_item (identifier) @function)
(call_expression function: (identifier) @function)
(call_expression function: (field_expression field: (field_identifier) @function.method))
(call_expression function: (scoped_identifier name: (identifier) @function))
(generic_function function: (identifier) @function)
(generic_function function: (scoped_identifier name: (identifier) @function))

; Macros
(macro_invocation macro: (identifier) @function.macro "!" @function.macro)
(macro_definition "macro_rules!" @function.macro)

; Fields and properties
(field_identifier) @variable.member
(shorthand_field_identifier) @variable.member

; Labels and lifetimes
(lifetime (identifier) @label)

; Parameters
(parameter (identifier) @variable.parameter)

; Modules
(mod_item name: (identifier) @module)
(scoped_identifier path: (identifier) @module)

; Self, crate, and special
(self) @variable.builtin
(crate) @keyword
(super) @keyword
(mutable_specifier) @keyword

; Keywords
"as" @keyword
"async" @keyword
"await" @keyword
"break" @keyword
"const" @keyword
"continue" @keyword
"dyn" @keyword
"else" @keyword
"enum" @keyword
"extern" @keyword
"fn" @keyword
"for" @keyword
"if" @keyword
"impl" @keyword
"in" @keyword
"let" @keyword
"loop" @keyword
"match" @keyword
"mod" @keyword
"move" @keyword
"pub" @keyword
"ref" @keyword
"return" @keyword
"static" @keyword
"struct" @keyword
"trait" @keyword
"type" @keyword
"unsafe" @keyword
"use" @keyword
"where" @keyword
"while" @keyword

; Operators
; Note: "/" and "!" are not matched globally to avoid conflicts with doc comments
; They are highlighted via binary_expression and unary_expression patterns below
"*" @operator
"&" @operator
"=" @operator
"+" @operator
"-" @operator
"%" @operator
"<" @operator
">" @operator
"==" @operator
"!=" @operator
"<=" @operator
">=" @operator
"&&" @operator
"||" @operator
"+=" @operator
"-=" @operator
"*=" @operator
"/=" @operator
".." @operator
"..=" @operator
"=>" @operator
"->" @operator
"?" @operator

; Division and negation operators in specific contexts
(binary_expression "/" @operator)
(unary_expression "!" @operator)

; Punctuation
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"::" @punctuation.delimiter
":" @punctuation.delimiter
"#;

pub const RUBY_HIGHLIGHTS: &str = r#"
; Comments
(comment) @comment

; Strings and symbols
(string) @string
(bare_string) @string
(subshell) @string
(heredoc_body) @string
(heredoc_beginning) @string
(simple_symbol) @string.special
(delimited_symbol) @string.special
(hash_key_symbol) @string.special
(bare_symbol) @string.special
(regex) @string.special

; Literals
(integer) @number
(float) @number
(nil) @constant.builtin
(true) @constant.builtin
(false) @constant.builtin

; Constants
(constant) @type

; Variables
(instance_variable) @property
(class_variable) @property
(global_variable) @variable.builtin
(self) @variable.builtin
(super) @variable.builtin

; Parameters
(block_parameter (identifier) @variable.parameter)
(block_parameters (identifier) @variable.parameter)
(method_parameters (identifier) @variable.parameter)
(keyword_parameter name: (identifier) @variable.parameter)
(optional_parameter name: (identifier) @variable.parameter)
(splat_parameter (identifier) @variable.parameter)
(hash_splat_parameter (identifier) @variable.parameter)

; Functions and methods
(method name: (identifier) @function)
(method name: (constant) @function)
(singleton_method name: (identifier) @function)
(call method: (identifier) @function.method)
(call method: (constant) @function.method)

; Keywords
"alias" @keyword
"and" @keyword
"begin" @keyword
"break" @keyword
"case" @keyword
"class" @keyword
"def" @keyword
"do" @keyword
"else" @keyword
"elsif" @keyword
"end" @keyword
"ensure" @keyword
"for" @keyword
"if" @keyword
"in" @keyword
"module" @keyword
"next" @keyword
"or" @keyword
"rescue" @keyword
"retry" @keyword
"return" @keyword
"then" @keyword
"unless" @keyword
"until" @keyword
"when" @keyword
"while" @keyword
"yield" @keyword
"not" @keyword
"defined?" @keyword

; Operators
"=" @operator
"=>" @operator
"->" @operator
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"%" @operator
"**" @operator
"==" @operator
"!=" @operator
"<" @operator
">" @operator
"<=" @operator
">=" @operator
"<=>" @operator
"&&" @operator
"||" @operator
"!" @operator
"&" @operator
"|" @operator
"^" @operator
"~" @operator
"<<" @operator
">>" @operator
".." @operator
"..." @operator

; Punctuation
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"," @punctuation.delimiter
";" @punctuation.delimiter
"." @punctuation.delimiter
":" @punctuation.delimiter
"::" @punctuation.delimiter
"#;

pub const JSON_HIGHLIGHTS: &str = r#"
(string) @string
(number) @number
(true) @constant.builtin
(false) @constant.builtin
(null) @constant.builtin
(pair key: (string) @property)

"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
":" @punctuation.delimiter
"," @punctuation.delimiter
"#;

pub const PYTHON_HIGHLIGHTS: &str = r#"
; Comments and strings
(comment) @comment
(string) @string
(escape_sequence) @string.special

; Literals
(integer) @number
(float) @number
(none) @constant.builtin
(true) @constant.builtin
(false) @constant.builtin

; Types and attributes
(type (identifier) @type)
(attribute attribute: (identifier) @property)

; Functions
(function_definition name: (identifier) @function)
(call function: (identifier) @function)
(call function: (attribute attribute: (identifier) @function.method))
(decorator) @function
(decorator (identifier) @function)

; Keywords
"as" @keyword
"assert" @keyword
"async" @keyword
"await" @keyword
"break" @keyword
"class" @keyword
"continue" @keyword
"def" @keyword
"del" @keyword
"elif" @keyword
"else" @keyword
"except" @keyword
"finally" @keyword
"for" @keyword
"from" @keyword
"global" @keyword
"if" @keyword
"import" @keyword
"lambda" @keyword
"nonlocal" @keyword
"pass" @keyword
"raise" @keyword
"return" @keyword
"try" @keyword
"while" @keyword
"with" @keyword
"yield" @keyword
"match" @keyword
"case" @keyword
"and" @operator
"or" @operator
"not" @operator
"in" @operator
"is" @operator

"#;

pub const GO_HIGHLIGHTS: &str = r#"
; Comments and strings
(comment) @comment
(interpreted_string_literal) @string
(raw_string_literal) @string
(rune_literal) @string

; Literals
(int_literal) @number
(float_literal) @number
(true) @constant.builtin
(false) @constant.builtin
(nil) @constant.builtin

; Types
(type_identifier) @type
(type_spec name: (type_identifier) @type)

; Functions
(function_declaration name: (identifier) @function)
(method_declaration name: (field_identifier) @function.method)
(call_expression function: (identifier) @function)
(call_expression function: (selector_expression field: (field_identifier) @function.method))

; Fields
(field_identifier) @property

; Package
(package_identifier) @module

; Keywords
"break" @keyword
"case" @keyword
"chan" @keyword
"const" @keyword
"continue" @keyword
"default" @keyword
"defer" @keyword
"else" @keyword
"fallthrough" @keyword
"for" @keyword
"func" @keyword
"go" @keyword
"goto" @keyword
"if" @keyword
"import" @keyword
"interface" @keyword
"map" @keyword
"package" @keyword
"range" @keyword
"return" @keyword
"select" @keyword
"struct" @keyword
"switch" @keyword
"type" @keyword
"var" @keyword

; Operators
"=" @operator
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"%" @operator
"!" @operator
"<" @operator
">" @operator
"&" @operator
"|" @operator
"^" @operator
":=" @operator
"==" @operator
"!=" @operator
"<=" @operator
">=" @operator
"&&" @operator
"||" @operator
"++" @operator
"--" @operator
"+=" @operator
"-=" @operator
"<-" @operator

; Punctuation
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"." @punctuation.delimiter
"," @punctuation.delimiter
";" @punctuation.delimiter
":" @punctuation.delimiter
"#;

pub const CSS_HIGHLIGHTS: &str = r#"
(comment) @comment
(string_value) @string
(integer_value) @number
(float_value) @number
(color_value) @constant
(property_name) @property
(tag_name) @tag
(class_name) @type
(id_name) @constant
(at_keyword) @keyword
"#;

pub const HTML_HIGHLIGHTS: &str = r#"
(comment) @comment
(quoted_attribute_value) @string
(tag_name) @tag
(attribute_name) @attribute
"#;

pub const TOML_HIGHLIGHTS: &str = r#"
(comment) @comment
(string) @string
(integer) @number
(float) @number
(boolean) @constant.builtin
(bare_key) @property
(dotted_key) @property
"#;

pub const BASH_HIGHLIGHTS: &str = r#"
(comment) @comment
(string) @string
(raw_string) @string
(number) @number
(command_name) @function
(variable_name) @variable
"#;

pub const MD_HIGHLIGHTS: &str = r#"
(atx_heading) @keyword
(setext_heading) @keyword
(thematic_break) @punctuation.delimiter
(fenced_code_block) @string
(indented_code_block) @string
(block_quote) @comment
(list_marker_plus) @punctuation
(list_marker_minus) @punctuation
(list_marker_star) @punctuation
(list_marker_dot) @punctuation
(list_marker_parenthesis) @punctuation
(link_destination) @string
(link_title) @string
"#;

pub const CSHARP_HIGHLIGHTS: &str = r#"
; Comments
(comment) @comment

; Strings and literals
(string_literal) @string
(verbatim_string_literal) @string
(interpolated_string_expression) @string
(character_literal) @string
(integer_literal) @number
(real_literal) @number
(boolean_literal) @constant.builtin
(null_literal) @constant.builtin

; Types - C# doesn't have type_identifier, types are represented by identifier in context
; or predefined_type for built-in types
(predefined_type) @type.builtin

; Namespaces and usings
(namespace_declaration name: (qualified_name) @module)
(namespace_declaration name: (identifier) @module)
(using_directive (identifier) @module)
(using_directive (qualified_name) @module)

; Classes, structs, interfaces, enums
(class_declaration name: (identifier) @type)
(struct_declaration name: (identifier) @type)
(interface_declaration name: (identifier) @type)
(enum_declaration name: (identifier) @type)
(record_declaration name: (identifier) @type)

; Methods and functions
(method_declaration name: (identifier) @function)
(local_function_statement name: (identifier) @function)
(constructor_declaration name: (identifier) @function)
(destructor_declaration name: (identifier) @function)
(invocation_expression function: (identifier) @function)
(invocation_expression function: (member_access_expression name: (identifier) @function.method))

; Properties and fields
(property_declaration name: (identifier) @property)
(field_declaration (variable_declaration (variable_declarator (identifier) @variable.member)))

; Parameters
(parameter name: (identifier) @variable.parameter)

; Attributes
(attribute) @attribute
(attribute_list) @attribute

; Keywords
"abstract" @keyword
"as" @keyword
"async" @keyword
"await" @keyword
"base" @keyword
"break" @keyword
"case" @keyword
"catch" @keyword
"checked" @keyword
"class" @keyword
"const" @keyword
"continue" @keyword
"default" @keyword
"delegate" @keyword
"do" @keyword
"else" @keyword
"enum" @keyword
"event" @keyword
"explicit" @keyword
"extern" @keyword
"finally" @keyword
"fixed" @keyword
"for" @keyword
"foreach" @keyword
"goto" @keyword
"if" @keyword
"implicit" @keyword
"in" @keyword
"interface" @keyword
"internal" @keyword
"is" @keyword
"lock" @keyword
"namespace" @keyword
"new" @keyword
"operator" @keyword
"out" @keyword
"override" @keyword
"params" @keyword
"private" @keyword
"protected" @keyword
"public" @keyword
"readonly" @keyword
"record" @keyword
"ref" @keyword
"return" @keyword
"sealed" @keyword
"sizeof" @keyword
"stackalloc" @keyword
"static" @keyword
"struct" @keyword
"switch" @keyword
"this" @keyword
"throw" @keyword
"try" @keyword
"typeof" @keyword
"unchecked" @keyword
"unsafe" @keyword
"using" @keyword
"var" @keyword
"virtual" @keyword
"volatile" @keyword
"when" @keyword
"where" @keyword
"while" @keyword
"yield" @keyword
"get" @keyword
"set" @keyword
"init" @keyword
"add" @keyword
"remove" @keyword
"partial" @keyword
"global" @keyword
"required" @keyword
"file" @keyword
"scoped" @keyword

; Operators
"=" @operator
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"%" @operator
"!" @operator
"<" @operator
">" @operator
"&" @operator
"|" @operator
"^" @operator
"~" @operator
"?" @operator
"==" @operator
"!=" @operator
"<=" @operator
">=" @operator
"&&" @operator
"||" @operator
"+=" @operator
"-=" @operator
"*=" @operator
"/=" @operator
"??" @operator
"??=" @operator
"=>" @operator
"->" @operator
"++" @operator
"--" @operator

; Punctuation
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"." @punctuation.delimiter
"," @punctuation.delimiter
";" @punctuation.delimiter
":" @punctuation.delimiter
"#;

pub const JAVA_HIGHLIGHTS: &str = r#"
; Variables
(identifier) @variable

; Comments
(line_comment) @comment
(block_comment) @comment

; Strings and literals
(string_literal) @string
(character_literal) @string
(escape_sequence) @string.special
(hex_integer_literal) @number
(decimal_integer_literal) @number
(octal_integer_literal) @number
(binary_integer_literal) @number
(decimal_floating_point_literal) @number
(hex_floating_point_literal) @number
(true) @constant.builtin
(false) @constant.builtin
(null_literal) @constant.builtin

; Types
(type_identifier) @type
(class_declaration name: (identifier) @type)
(interface_declaration name: (identifier) @type)
(enum_declaration name: (identifier) @type)
(record_declaration name: (identifier) @type)
(annotation_type_declaration name: (identifier) @type)
(constructor_declaration name: (identifier) @constructor)

[
  (boolean_type)
  (integral_type)
  (floating_point_type)
  (void_type)
] @type.builtin

; Constants and members
((identifier) @constant
  (#match? @constant "^_*[A-Z][A-Z\\d_]+$"))
(field_declaration declarator: (variable_declarator name: (identifier) @variable.member))
(field_access field: (identifier) @variable.member)

; Methods
(method_declaration name: (identifier) @function.method)
(method_invocation name: (identifier) @function.method)
(method_reference (identifier) @function.method)
(super) @function.builtin

; Parameters
(formal_parameter name: (identifier) @variable.parameter)
(spread_parameter (variable_declarator name: (identifier) @variable.parameter))
(catch_formal_parameter name: (identifier) @variable.parameter)

; Annotations and modules
(annotation name: (identifier) @attribute)
(marker_annotation name: (identifier) @attribute)
(package_declaration (scoped_identifier) @module)
(import_declaration (scoped_identifier) @module)

; Builtins
(this) @variable.builtin

; Keywords
"abstract" @keyword
"assert" @keyword
"break" @keyword
"case" @keyword
"catch" @keyword
"class" @keyword
"continue" @keyword
"default" @keyword
"do" @keyword
"else" @keyword
"enum" @keyword
"exports" @keyword
"extends" @keyword
"final" @keyword
"finally" @keyword
"for" @keyword
"if" @keyword
"implements" @keyword
"import" @keyword
"instanceof" @keyword
"interface" @keyword
"module" @keyword
"native" @keyword
"new" @keyword
"non-sealed" @keyword
"open" @keyword
"opens" @keyword
"package" @keyword
"permits" @keyword
"private" @keyword
"protected" @keyword
"provides" @keyword
"public" @keyword
"requires" @keyword
"record" @keyword
"return" @keyword
"sealed" @keyword
"static" @keyword
"strictfp" @keyword
"switch" @keyword
"synchronized" @keyword
"throw" @keyword
"throws" @keyword
"to" @keyword
"transient" @keyword
"transitive" @keyword
"try" @keyword
"uses" @keyword
"volatile" @keyword
"when" @keyword
"while" @keyword
"with" @keyword
"yield" @keyword

; Operators
"@" @operator
"=" @operator
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"%" @operator
"!" @operator
"<" @operator
">" @operator
"&" @operator
"|" @operator
"^" @operator
"~" @operator
"?" @operator
"==" @operator
"!=" @operator
"<=" @operator
">=" @operator
"&&" @operator
"||" @operator
"+=" @operator
"-=" @operator
"*=" @operator
"/=" @operator
"%=" @operator
"&=" @operator
"|=" @operator
"^=" @operator
"<<" @operator
">>" @operator
">>>" @operator
"->" @operator
"::" @operator
"++" @operator
"--" @operator

; Punctuation
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"." @punctuation.delimiter
"," @punctuation.delimiter
";" @punctuation.delimiter
":" @punctuation.delimiter
"..." @punctuation.delimiter
"::" @punctuation.delimiter
"#;

/*
Portions of `ZIG_HIGHLIGHTS` are adapted from tree-sitter-zig 1.1.2
`queries/highlights.scm` and remapped to Divergent's supported capture names.

Upstream project: https://github.com/tree-sitter-grammars/tree-sitter-zig

MIT License

Copyright (c) 2024 Amaan Qureshi <amaanq12@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
pub const ZIG_HIGHLIGHTS: &str = r#"
; Variables
(identifier) @variable

; Parameters
(parameter
  name: (identifier) @variable.parameter)

((payload
  (identifier) @variable.parameter)
  (#set! "priority" 110))

; Types
(parameter
  type: (identifier) @type)

((identifier) @type
  (#match? @type "^[A-Z_][a-zA-Z0-9_]*$"))

(variable_declaration
  (identifier) @type
  "="
  [
    (struct_declaration)
    (enum_declaration)
    (union_declaration)
    (opaque_declaration)
  ])

[
  (builtin_type)
  "anyframe"
] @type.builtin

; Constants
((identifier) @constant
  (#match? @constant "^[A-Z][A-Z_0-9]+$"))

[
  "null"
  "unreachable"
  "undefined"
] @constant.builtin

(field_expression
  .
  member: (identifier) @constant)

(enum_declaration
  (container_field
    type: (identifier) @constant))

; Labels
(block_label (identifier) @label)

(break_label (identifier) @label)

; Fields
(field_initializer
  .
  (identifier) @variable.member)

(field_expression
  (_)
  member: (identifier) @variable.member)

(container_field
  name: (identifier) @variable.member)

(initializer_list
  (assignment_expression
      left: (field_expression
              .
              member: (identifier) @variable.member)))

; Functions
(builtin_identifier) @function.builtin

(call_expression
  function: (identifier) @function)

(call_expression
  function: (field_expression
    member: (identifier) @function.method))

(function_declaration
  name: (identifier) @function)

; Modules
(variable_declaration
  (identifier) @module
  (builtin_function
    (builtin_identifier) @keyword
    (#any-of? @keyword "@import" "@cImport")))

; Builtins
[
  "c"
  "..."
] @variable.builtin

((identifier) @variable.builtin
  (#eq? @variable.builtin "_"))

(calling_convention
  (identifier) @variable.builtin)

; Keywords
[
  "asm"
  "defer"
  "errdefer"
  "test"
  "error"
  "const"
  "var"
  "struct"
  "union"
  "enum"
  "opaque"
  "async"
  "await"
  "suspend"
  "nosuspend"
  "resume"
  "fn"
  "and"
  "or"
  "orelse"
  "return"
  "if"
  "else"
  "switch"
  "for"
  "while"
  "break"
  "continue"
  "usingnamespace"
  "export"
  "try"
  "catch"
  "volatile"
  "allowzero"
  "noalias"
  "addrspace"
  "align"
  "callconv"
  "linksection"
  "pub"
  "inline"
  "noinline"
  "extern"
  "comptime"
  "packed"
  "threadlocal"
] @keyword

; Operators
[
  "="
  "*="
  "*%="
  "*|="
  "/="
  "%="
  "+="
  "+%="
  "+|="
  "-="
  "-%="
  "-|="
  "<<="
  "<<|="
  ">>="
  "&="
  "^="
  "|="
  "!"
  "~"
  "-"
  "-%"
  "&"
  "=="
  "!="
  ">"
  ">="
  "<="
  "<"
  "^"
  "|"
  "<<"
  ">>"
  "<<|"
  "+"
  "++"
  "+%"
  "+|"
  "*"
  "/"
  "%"
  "**"
  "*%"
  "*|"
  "||"
  ".*"
  ".?"
  "?"
  ".."
] @operator

; Literals
(character) @string.special

([
  (string)
  (multiline_string)
] @string
  (#set! "priority" 95))

(integer) @number

(float) @number

(boolean) @constant.builtin

(escape_sequence) @string.special

; Punctuation
[
  "["
  "]"
  "("
  ")"
  "{"
  "}"
] @punctuation.bracket

[
  ";"
  "."
  ","
  ":"
  "=>"
  "->"
] @punctuation.delimiter

(payload "|" @punctuation.bracket)

; Comments
(comment) @comment
"#;

/*
Portions of `POSTGRES_HIGHLIGHTS` are adapted from tree-sitter-postgres
`postgres/queries/highlights.scm` and remapped to Divergent's supported
capture names.

Upstream project: https://github.com/gmr/tree-sitter-postgres

BSD 3-Clause License

Copyright (c) 2023, PostgreSQL Global Development Group
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
pub const POSTGRES_HIGHLIGHTS: &str = r#"
; Comments
(comment) @comment

; Literals
(integer_literal) @number
(float_literal) @number
(string_literal) @string
(bit_string_literal) @string
(hex_string_literal) @string

(kw_true) @constant.builtin
(kw_false) @constant.builtin
(kw_null) @constant.builtin

(param) @variable.parameter

; Identifiers
(identifier) @variable

(columnref
  (ColId) @variable)

; Types
[
  (Numeric)
  (Bit)
  (ConstBit)
  (Character)
  (ConstCharacter)
  (ConstDatetime)
  (ConstInterval)
  (JsonType)
] @type.builtin

(Typename
  (SimpleTypename
    (GenericType
      (type_function_name
        (identifier) @type) .)))

(Typename
  (SimpleTypename
    (GenericType
      (type_function_name
        (identifier) @type)
      (opt_type_modifiers))))

(Typename
  (SimpleTypename
    (GenericType
      (attrs
        (attr_name
          (ColLabel
            (identifier) @type))))))

(DefineStmt
  (kw_create)
  (kw_type)
  (any_name
    (attrs
      (attr_name
        (ColLabel
          (identifier) @type)))))

(DefineStmt
  (kw_create)
  (kw_type)
  (any_name
    (ColId
      (identifier) @type) .))

; Functions
(func_application
  (func_name) @function)

(func_expr_common_subexpr) @function

; Operators
(operator) @operator

[
  "+"
  "-"
  "*"
  "/"
  "%"
  "^"
  "<"
  ">"
  "="
] @operator

; Punctuation
["(" ")"] @punctuation.bracket
["[" "]"] @punctuation.bracket
"," @punctuation.delimiter
"." @punctuation.delimiter
";" @punctuation.delimiter

; Statement keywords
[
  (kw_select)
  (kw_from)
  (kw_where)
  (kw_insert)
  (kw_into)
  (kw_update)
  (kw_delete)
  (kw_create)
  (kw_alter)
  (kw_drop)
  (kw_table)
  (kw_index)
  (kw_view)
  (kw_with)
  (kw_as)
  (kw_set)
  (kw_values)
  (kw_returning)
  (kw_explain)
  (kw_analyze)
  (kw_vacuum)
  (kw_truncate)
  (kw_copy)
  (kw_grant)
  (kw_revoke)
] @keyword

; Clause keywords
[
  (kw_distinct)
  (kw_all)
  (kw_group)
  (kw_order)
  (kw_by)
  (kw_having)
  (kw_limit)
  (kw_offset)
  (kw_fetch)
  (kw_for)
  (kw_on)
  (kw_using)
  (kw_asc)
  (kw_desc)
  (kw_nulls)
  (kw_first)
  (kw_last)
  (kw_only)
  (kw_recursive)
  (kw_cascade)
  (kw_restrict)
  (kw_if)
  (kw_exists)
] @keyword

; Join keywords
[
  (kw_join)
  (kw_inner)
  (kw_left)
  (kw_right)
  (kw_full)
  (kw_cross)
  (kw_natural)
  (kw_lateral)
] @keyword

; Logical / boolean keywords
[
  (kw_and)
  (kw_or)
  (kw_not)
  (kw_in)
  (kw_between)
  (kw_like)
  (kw_ilike)
  (kw_similar)
  (kw_is)
  (kw_isnull)
  (kw_notnull)
  (kw_escape)
] @keyword

; Set operation keywords
[
  (kw_union)
  (kw_intersect)
  (kw_except)
] @keyword

; Conditional keywords
[
  (kw_case)
  (kw_when)
  (kw_then)
  (kw_else)
  (kw_end)
] @keyword

; Transaction keywords
[
  (kw_begin)
  (kw_commit)
  (kw_rollback)
  (kw_savepoint)
  (kw_release)
  (kw_abort)
  (kw_start)
] @keyword

; Type keywords
[
  (kw_int)
  (kw_integer)
  (kw_smallint)
  (kw_bigint)
  (kw_decimal)
  (kw_numeric)
  (kw_float)
  (kw_real)
  (kw_double)
  (kw_char)
  (kw_character)
  (kw_varchar)
  (kw_text)
  (kw_boolean)
  (kw_bit)
  (kw_time)
  (kw_timestamp)
  (kw_interval)
  (kw_array)
  (kw_json)
  (kw_xml)
] @type.builtin

((Typename
  (SimpleTypename
    (GenericType
      (type_function_name
        (identifier) @type.builtin))))
  (#match? @type.builtin "(?i)^(bigserial|bool|inet|jsonb|timestamptz|uuid|void)$"))

; Constraint / DDL keywords
[
  (kw_primary)
  (kw_key)
  (kw_unique)
  (kw_check)
  (kw_foreign)
  (kw_references)
  (kw_constraint)
  (kw_default)
  (kw_collate)
  (kw_not)
] @keyword

; Aggregate / window keywords
[
  (kw_over)
  (kw_partition)
  (kw_rows)
  (kw_range)
  (kw_groups)
  (kw_preceding)
  (kw_following)
  (kw_unbounded)
  (kw_current)
] @keyword

; Other common keywords
[
  (kw_to)
  (kw_of)
  (kw_cast)
  (kw_do)
  (kw_function)
  (kw_procedure)
  (kw_trigger)
  (kw_temporary)
  (kw_temp)
  (kw_unlogged)
  (kw_materialized)
  (kw_schema)
  (kw_database)
  (kw_extension)
  (kw_sequence)
  (kw_domain)
  (kw_type)
  (kw_role)
  (kw_user)
  (kw_owner)
  (kw_language)
  (kw_replace)
  (kw_returns)
  (kw_security)
  (kw_row)
  (kw_column)
  (kw_add)
  (kw_rename)
  (kw_no)
  (kw_cycle)
  (kw_increment)
  (kw_maxvalue)
  (kw_minvalue)
  (kw_cache)
  (kw_owned)
  (kw_local)
  (kw_global)
  (kw_execute)
  (kw_prepare)
  (kw_deallocate)
  (kw_listen)
  (kw_notify)
  (kw_load)
  (kw_lock)
  (kw_move)
  (kw_cluster)
  (kw_reindex)
  (kw_reset)
  (kw_show)
  (kw_enable)
  (kw_disable)
  (kw_refresh)
  (kw_concurrently)
  (kw_import)
  (kw_policy)
  (kw_publication)
  (kw_subscription)
] @keyword
"#;
