# Syntax

```ebnf
Root = Item*

Item =
    FunctionDef
    | StructDef
    | EnumDef
    | ConstDef
    | UseDef
    | ImplBlock
    | TypeAlias
    // TODO: Relations & Clauses

FunctionDef = Attribute* Modifier* keyword:'fn' name:FunctionName Generics args:FunctionArgs ret:FunctionReturn? body:Block
FunctionName = 'ident'
FunctionArgs = '(' args:FunctionArg* ')'
FunctionArg = Attribute* binding:Pattern ':' ty:Type ','*
FunctionReturn = '->' return_ty:Type

StructDef = Attribute* Modifier* keyword:'struct' name:StructName fields:StructFields
StructName = 'ident'
StructFields = BrackedStructFields | TupleStructFields
BrackedStructFields = '{' fields:BrackedStructField* '}'
BrackedStructField = name:StructFieldName ':' ty:Type ','*
StructFieldName = 'ident'
TupleStructFields = '(' TupleStructField* ')'
TupleStructField = Type ','*

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
UseTree = Path (UseBranch | 'as' UseAlias)?
UseAlias = 'ident'
UseBranch = '{' (UseTree ','*)* '}'

// TODO: `impl Trait for Type { ... }`
ImplBlock = Attribute* Modifier* keyword:'impl' Type contents:ImplBlockContents
ImplBlockContents = '{' Item* '}'

TypeAlias =  Attribute* Modifier* keyword:'type' alias:Type '=' original:Type ';'*

Attribute = '#[' AttrPair* ']'
AttrPair = AttrName ('=' Expr ','*)?
AttrName = 'ident'

Modifier = 'pub' // 'input' | 'output'

Pattern = VarRef | TuplePattern | StructPattern
TuplePattern = '(' elements:TuplePatternElem* ')'
TuplePatternElem = Pattern ','*
StructPattern = '{' fields:StructPatternField* '}'
StructPatternField = name:StructPatternFieldName (':' alias:Pattern)? ','*
StructPatternFieldName = 'ident'

Type = GenericType | TupleType | FunctionType
Path = '::'? PathSegment ('::' PathSegment)*
PathSegment = 'ident'

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

ExprStmt = Expr ';'*
VarDecl = 'let' binding:Pattern (':' Type)? '=' value:Expr ';'*

Expr =
    Literal
    | VarRef
    | QualifiedRef
    | Assign
    | ParenExpr
    | BinExpr
    | IfExpr
    | RetExpr
    | BreakExpr
    | ContinueExpr
    | UnaryExpr
    | Block
    | WhileExpr
    | ForExpr
    | LoopExpr
    | MatchExpr
    | ClosureExpr
    | FieldAccess
    | ArrayAccess
    | FunctionCall
    | StructInitExpr
    | TupleInitExpr
    // TODO: `if let` and `while let` expressions

VarRef = 'ident'
QualifiedRef = Path

WhileExpr = 'while' cond:Expr Block
ForExpr = 'for' binding:Pattern 'in' iter:Expr Block
LoopExpr = 'loop' Block

IfExpr = IfBlock* ElseBlock?
IfBlock = leading_else:'else'? 'if' cond:Expr Block
ElseBlock = 'else' Block

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

ClosureExpr = '|' args:ClosureArg* '|' body:Expr
ClosureArg = binding:Pattern (':' Type)? ','*

FieldAccess = Expr '.' ('ident' | Number)
ArrayAccess = Expr '[' index:Expr ']'

FunctionCall = func:Expr '(' args:FunctionCallArg* ')'
// Future Possibility: Named arguments
FunctionCallArg = arg:Expr ','*

StructInitExpr = ty:Path '{' fields:StructInitField* '}'
StructInitField = field:'ident' (':' value:Expr)? ','*

TupleInitExpr = '(' elems:TupleExprElem* ')'
TupleExprElem = Expr ','*

// TODO: Floats
Literal = Bool | Number | String
Bool = 'true' | 'false'
Number = 'number'
String = 'string'

RetExpr = 'return' expr:Expr
BreakExpr = 'break' expr:Expr
ContinueExpr = 'continue'

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
