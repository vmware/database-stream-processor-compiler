# Syntax

The root of a ddlog file will contain any number of item declarations,
an empty ddlog file is a valid program.

```ebnf
Root := Item*

Ident := [A-Za-z_][A-Za-z0-9_]*
Path := (Ident)'.'+
Pattern := Ident | Literal | TuplePattern | RecordPattern
TuplePattern := '(' (Pattern)','* ','? ')'
RecordDecl := '{' (Ident (':' Pattern)?)','* ','? '}'
Attribute := '#[' (AttrPair)','* ','? ']'
AttrPair := Ident ('=' Expr)?
Modifier := 'export'
```

## Items

Items are any of the top-level declarations contained within a ddlog file

### Imports

Imports allow using other modules and the types contained within them

#### Grammar

```ebnf
Import := 'import' Path ('exposing' (Path)','+ ','?)?
```

#### Example

```
import graphs
import graphs exposing SCC
import graphs exposing SCC, SCCUnsafe
```

### Relations

#### Grammar

```ebnf
Relation := Attribute* Modifier* ('relation' | 'stream' | 'multiset') Ident '=' (Type | RecordDecl)
```

### Functions

#### Grammar

```ebnf
Function := Attribute* Modifier* 'function' Ident '(' (FunctionArgument)','* ','? ')' ('->' Type)? Block
FunctionArgument := Pattern ':' Type
```

#### Example

```
function foo() -> Bar {
    Bar.new()
}
```

### Type Declarations

#### Grammar

```ebnf
TypeDecl := Attribute* Modifier* 'type' Ident '=' (Type | RecordDecl)
RecordDecl := '{' (Ident ':' Type)','* ','? '}'
```

#### Example

```
type Foo = Bar
type Foo = (u32, u32, u32)
type Foo = { bar: u32, baz: Box[Foo] }
```

## Statements

```ebnf
Stmt := (VarDecl | Expr) ';'*
VarDecl := 'let' 'mut'? Pattern '=' Expr
```

## Expressions

```ebnf
Expr :=
    Literal
    | VarRef
    | Block
    | If
    | Match
    | For
    | While
    | Loop
    | Assign
    | BinOp
    | Return
    | Break
    | Continue
    | '(' Expr ')'

Literal := Bool | String | Number | Float
Bool := 'true' | 'false'
String := '"' .* '"'
Number := [0-9]+
Float := [0-9]+ '.' [0-9]+
VarRef := Ident
Block := '{' (Stmt)';'* '}'
If := 'if' Expr Block ('else' 'if' Expr Block)* ('else' Block)?
Match := 'match' Expr '{' (Pattern '=>' Expr)','* ','? '}'
For := 'for' Pattern 'in' Expr Block
While := 'while' Expr Block
Loop := 'loop' Block
Assign := Pattern AssignModifier? '=' Expr
AssignModifier :=
    '+' | '-' | '*' | '/'
    | '%' | '&' | '|' | '^'
BinOp := Expr BinaryOperator Expr
BinaryOperator :=
    AssignModifier | 'and' | 'or'
    | '<' | '>' | '<=' | '>='
    | '==' | '!='
Return = 'return' Expr
Break = 'break' Expr
Continue = 'continue'
```
