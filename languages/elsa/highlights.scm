; Keywords
"let" @keyword
"eval" @keyword.control
"conf" @keyword.control

; Operators & punctuation

; Lambda arrow  ->
"->" @keyword.operator

; Lambda backslash  \
"\\" @keyword.operator

; Assignment = (in let)
"=" @operator

; Colon after eval/conf name  :
":" @punctuation.delimiter

; Parentheses
"(" @punctuation.bracket
")" @punctuation.bracket

; Step operators (=b>, =d>, =n*>, =b:w>, …)

(step_operator) @operator

; Definitions

; Name being defined
(let_binding  name: (identifier) @function.definition)
(eval_block   name: (identifier) @function.definition)
(conf_block   name: (identifier) @function.definition)

; Terms

; Lambda parameters  \x y -> ...
(lambda params: (identifier) @variable.parameter)

; All other identifiers are variables
(variable (identifier) @variable)

; Comments
(line_comment)  @comment
(block_comment) @comment
