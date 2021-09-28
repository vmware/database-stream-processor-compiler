# Syntax

```ebnf
Root = Item*

Item =
    FunctionDef
    | StructDef
    | EnumDef
    | ConstDef
    | UseDef
    // TODO: Relations & Clauses

FunctionDef = Attribute* Modifier* keyword:'fn' name:FunctionName Generics args:FunctionArgs ret:FunctionReturn? body:Block
FunctionName = 'ident'
FunctionArgs = '(' args:FunctionArg* ')'
FunctionArg = Attribute* binding:Pattern ':' ty:Type ','*
FunctionReturn = '->' return_ty:Type

StructDef = Attribute* Modifier* keyword:'struct' name:StructName fields:StructFields
StructName = 'ident'
StructFields = '{' fields:StructField* '}'
StructField = name:StructFieldName ':' ty:Type ','*
StructFieldName = 'ident'

EnumDef = Attribute* Modifier* keyword:'enum' name:EnumName variants:EnumVariants
EnumName = 'ident'
EnumVariants = '{' variants:EnumVariant* '}'
EnumVariant = EnumVariantName (VariantTuple | VariantStruct)
EnumVariantName = 'ident'
VariantTuple = '(' VariantTupleElem* ')'
VariantTupleElem = Type ','*
VariantStruct = '{' fields:VariantStructField* '}'
VariantStructField = name:VariantStructFieldName ':' ty:Type ','*
VariantStructFieldName = 'ident'

ConstDef = Attribute* Modifier* keyword:'const' name:ConstName '=' ConstValue ';'
// TODO: Could be a pattern
ConstName = 'ident'
ConstValue = value:Expr

UseDef = Attribute* Modifier* keyword:'use' UseTree ';'
UseTree = Path UseBranch?
UseBranch = '{' (UseTree ','*)* '}'

Attribute = '#[' AttrPair* ']'
AttrPair = AttrName ('=' Expr ','*)?
AttrName = 'ident'

Modifier = 'pub' // 'input' | 'output'

Type = GenericType | TupleType | FunctionType
Path = '::'? 'ident' ('::' 'ident')*

GenericType = Path Generics?
Generics = '<' generics:GenericArg* '>'
GenericArg = Type ','*

TupleType = '(' elements:TupleTypeElem* ')'
TupleTypeElem = Type ','*

FunctionType = 'fn' args:FunctionTypeArgs ret:FunctionReturnType?
FunctionTypeArgs = '(' args:FunctionTypeArg* ')'
FunctionTypeArg = Type ','*
FunctionReturnType = '->' Type

Block = '{' statements:Stmt* '}'
Stmt =
    ExprStmt
    | VarDecl
    | IfStmt

ExprStmt = Expr ';'
VarDecl = 'let' binding:Pattern '=' value:Expr ';'

IfStmt = IfBlock* ElseBlock?
IfBlock = leading_else:'else'? 'if' cond:Expr Block
ElseBlock = 'else' Block

Pattern = VarRef | TuplePattern
TuplePattern = '(' elements:TuplePatternElem* ')'
TuplePatternElem = Pattern ','*

Expr =
    Literal
    | VarRef
    | Assign
    | ParenExpr
    | BinExpr
    | IfStmt
    | RetExpr
    | UnaryExpr
    | Block
    | WhileExpr
    | ForExpr
    | LoopExpr
    | MatchExpr

VarRef = 'ident'

WhileExpr = 'while' cond:Expr Block

ForExpr = 'for' binding:Pattern 'in' iter:Expr Block

LoopExpr = 'loop' Block

MatchExpr = 'match' scrutinee:Expr '{' MatchArm* '}'
MatchArm = binding:Pattern '=>' body:Expr ','* 

Assign = binding:Pattern AssignOp value:Expr
AssignOp =
    '='
    | '+='
    | '-='
    | '/='
    | '*='
    | '%='
    | '&='
    | '|='
    | '^='
    | '<<='
    | '>>='

ParenExpr = '(' inner:Expr ')'

// TODO: Floats
Literal = Bool | Number | String
Bool = 'true' | 'false'
Number = 'number'
String = 'string'

RetExpr = 'return' expr:Expr

UnaryExpr = op:UnaryOp expr:Expr
UnaryOp = '!' | '-'

BinExpr = lhs:Expr op:BinOp rhs:Expr
BinOp =
    '+'
    | '-'
    | '*'
    | '/' 
    | '%'
    | '|'
    | '^'
    | '&'
    | '<<'
    | '>>'
    | 'and'
    | 'or'
    | '=='
    | '!='
    | '>'
    | '>='
    | '<'
    | '<='
```
