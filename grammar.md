# Grammar for Java that we will be using

#### Note:
Donzo     == [X]
Not done  == [ ]

### declare pacakge, import files, then declare types
```
[ ] <program> ::= <package_decl> <import> {<type_decl>}
```

### package, import
```
[x] <package_decl>  ::= [ "package" IDENTIFIER { "." IDENTIFIER } ";" ]
[x] <import>        ::= { "import" [ "static" ] IDENTIFIER { "." IDENTIFIER } [ ".*" ] ";" }
```

### util stuffs that everything uses
```
[x] <annotation>      ::= "@" IDENTIFIER {"." IDENTIFIER} ["(" <skip_parens> ")"]
[x] <modifier>        ::= "public" | "private" | "protected" | "abstract" | "static" | "final" | "strictfp"
[x] <modifiers>       ::= { <modifiers> }
[x] <type>            ::= "void" | <ref_type>
[x] <ref_type>        ::= IDENTIFIER { "." IDENTIFIER } [ "<" <type_arg_lst> ">" ] { "[]" }
[x] <type_arg_list>   ::= <type_arg> { "," <type_arg> }
[x] <type_arg>        ::= <ref_type> | "?" [ ( "extends" | "super" ) <ref_type> ]
[x] <type_params>     ::= "<" <type_param> { "," <type_param> } ">"
[x] <type_param>      ::= IDENTIFIER [ "extends" <ref_type> { "&" <ref_type> } ]
```

#### Note: 
- Type params is the parameter, e.g. `public <T> T at(int i) {...}`
- Type args is the argument, e.g. `ArrayList<int> lst;`

### type: class, enum, interface, annotation
```
[ ] <type_decl>       ::= {<annotation>} <modifiers> ( <enum_decl> | <class_decl> | <interface_decl> | <annotation_decl> )
[ ] <enum_decl>       ::= "enum" IDENTIFIER [ "implements" <ref_type> { "," <ref_type> } ] "{" <enum_body> "}"
[ ] <class_decl>      ::= "class" IDENTIFIER [ "extends" <ref_type> ] 
                      [ "implements" <ref_type> { "," <ref_type> } ] "{" <class_body> "}"
[ ] <interface_decl>  ::= "interface" IDENTIFIER [ "extends" <ref_type> { "," <ref_type> } ] "{" <skip_interface_body> "}"
[ ] <annotation_decl> ::= "@interface" IDENTIFIER "{" <skip_annotation_body> "}"
```

### Body for a class: properties, functions
```
[ ] <class_body>      ::= { <member_decl> }
[ ] <member_decl>     ::= <modifiers> ( <method_decl> | <property_decl> | <type_decl> )
[ ] <method_decl>     ::= [<type_params>] [<type>] IDENTIFIER "(" <arg_list> ")" ["throws" <ref_type> {"," <ref_type>}]
                      "{" <skip_body> "}"
[ ] <property_decl>   ::= <ref_type> IDENTIFIER [ "=" <skip_expr> ] ";"

```

### Body for enum
```
[ ] <enum_body> ::= {<enum_val>} [ ";" [ <class_body> ] ]
[ ] <enum_val>  ::= IDENTIFIER ["(" <skip_paren> ")"] ["{" <skip_brace> "}"]
```

### Note
All `<skip_something>` nonterminals are skipped via some sort of stack counting
