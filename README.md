# shot

## Warning

This repository is `Work In Progress`. Note that breaking change may occur.

## WIP

### Parser

- ✅ Declaration of Variable
- ✅ Declaration of Function
- ✅ Primary
- ✅ Binary
- ✅ Parenthesis
- ✅ Call of Variable and Function
- ✅ Type Cast Operation

### Evaluator

- Declaration of Variable
- Declaration of Function
- Primary
- Binary
- Parenthesis
- Call of Variable and Function
- Type Cast Operation

## Usage

```
shot --help

     _______. __    __    ______   .___________.
    /       ||  |  |  |  /  __  \  |           |
   |   (----`|  |__|  | |  |  |  | `---|  |----`
    \   \    |   __   | |  |  |  |     |  |
.----)   |   |  |  |  | |  `--'  |     |  |
|_______/    |__|  |__|  \______/      |__|
                    ##
             #####%%##%%%#####
               %%#%@@%#%##%%%%#####%%########%%%%%
###+#######*-+*###****######***#%%%#%@%
%%%%%%#######** %%%             %%%
%%%###*##   #
 %


kill your task in one shot

USAGE:
    shot [OPTIONS]

Options:
  -i <inline>      read source code inline
  -f <file>        read source code from file
  -d, --debug      Enable debug mode
  -h, --help       Print help
  -V, --version    Print version
```

## Install and Uninstall

We use [Task](https://taskfile.dev/) to manage the installation and uninstallation of the program.

```shell
task install
task uninstall
```

## BNF

This is BNF of Shot Language.

```
<Program> ::= <StatementList>
<StatementList> ::= <Statement> ";" | <Statement> ";" <StatementList>

<Statement> ::= <VariableDeclaration>
              | <FunctionDeclaration>
              | <ReturnStatement>
              | <Expression>

<VariableDeclaration> ::= "let" <Identifier> ":" <Type> "=" <Expression>

<FunctionDeclaration> ::= "let" <Identifier> ":" "fn" "=" "(" <ParameterList> ")" ":" <Type> "{" <StatementList> "return" <Expression> ";"}"

<ReturnStatement> ::= "return" <Expression>

<Expression> ::= <Literal>
               | <Identifier>
               | <FunctionCall>
               | <BinaryOperation>
               | <TypeCast>

<Literal> ::= <IntegerLiteral> | <FloatLiteral> | <StringLiteral> | "none"

<FunctionCall> ::= <Identifier> "(" <ArgumentList> ")"

<BinaryOperation> ::= <Expression> <BinaryOperator> <Expression>

<TypeCast> ::= <Expression> "as" <Type> "->" <Type>

<Type> ::= "int" | "float" | "string" | "void" | "fn"

<ArgumentList> ::= <Expression> | <Expression> "," <ArgumentList>
```
