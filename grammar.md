# Grammar for Java that we will be using

### declare pacakge, import files, then declare types
```
<program> ::= [<package_decl>] {<import>} {<type_decl>}
```

### package com.file.etc;
```
<package_decl> ::= "package" IDENTIFIER { "." IDENTIFIER } ";"
```

### util stuffs that everything uses
```
<modifier>        ::= "public" | "private" | "protected" | "abstract" | "static" | "final" | "strictfp"
<type>            ::= "void" | <ref_type>
<ref_type>        ::= IDENTIFIER { "." IDENTIFIER } [ "<" <type_arg_lst> ">" ] { "[]" }
<type_arg_list>   ::= <type_arg> { "," <type_arg> }
<type_arg>        ::= <ref_type> | "?" [ ( "extends" | "super" ) <ref_type> ]
<type_params>     ::= "<" <type_param> { "," <type_param } ">"
<type_param>      ::= IDENTIFIER [ "extends" <ref_type> { "&" <ref_type> } ]
```
#### Note: 
- Type params is the parameter, e.g. `public <T> T at(int i) {...}`
- Type args is the argument, e.g. `ArrayList<int> lst;`

### type: class, enum, interface, annotation
```
<type_decl>       ::= { <modifier> } ( <enum_decl> | <class_decl> | <interface_decl> | <annotation_decl> )
<enum_decl>       ::= "enum" IDENTIFIER [ "implements" <ref_type> { "," <ref_type> } ] "{" <enum_body> "}"
<class_decl>      ::= "class" IDENTIFIER [ "extends" <ref_type> ] 
                      [ "implements" <ref_type> { "," <ref_type> } ] "{" <class_body> "}"
<interface_decl>  ::= "interface" IDENTIFIER [ "extends" <ref_type> { "," <ref_type> } ] "{" <interface_body> "}"
<annotation_decl> ::= "@interface" IDENTIFIER "{" <annotation_body> "}"
```

### Body for a class: properties, functions
```
<class_body>      ::= { { <modifier> }( <method_decl> | <property_decl> ) }
<method_decl>     ::= [ <type_params> ] [ <type> ] IDENTIFIER "(" <arg_list> ")" "{" <method_body> "}"
<property_decl>   ::= IDENTIFIER IDENTIFIER [ "=" <expr> ] ";"

```

