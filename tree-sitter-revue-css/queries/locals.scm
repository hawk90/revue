; Local variable tracking for Revue CSS

; CSS variables are scoped to their rule set
(rule_set) @local.scope

; Variable definitions
(declaration
  (property (variable_name) @local.definition))

; Variable references
(variable_ref
  (variable_name) @local.reference)
