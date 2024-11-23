# shot

## Warning

This repository is `Work In Progress`. Note that breaking change may occur.

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

## Getting Started

### Comment

```shot
# you can use comment
```

### declaration of variable

```shot
let a: int = 1;
let b: string = "hello";
let c: void = none; # none is value for void type
```

You can't declare a variable without a type.

```shot
let a = 1; # error
```

### declaration of function

```shot
let f: fn = (x: int, y: int): int {
  return x + y;
};
```

You can't declare a function without a type. 

```shot
let f = (x, y): int {
  return x + y;
}; # error
```

Also, you can't declare a function without a return type.

```shot
let f: fn = (x: int, y: int) {
  return x + y;
}; # error
let f: fn = (x: int, y: int): int {
  return x + y;
}; # ok
```

Finally, you must declare a function with return statement.

```shot
let f: fn = (x: int, y: int): void {
  x + y;
}; # error
```

### Call of Variable

You can call a variable.

```shot
let a: int = 1;
let b: int = a;
```

### Call of Function

You can call a function.

```shot
let f: fn = (x: int, y: int): int {
  return x + y;
};
let a: int = f(x: 1, y: 2);
```

you must pass the argument with the name of the parameter.

```shot
let f: fn = (x: int, y: int): int {
  return x + y;
};
let a: int = f(1, 2); # error
```

### Binary Operation

Shot supports binary operation.

```shot
let a: int = 1 + 2;
let b: int = 1 - 2;
let c: int = 1 * 2;
let d: int = 1 / 2;
```

### Parenthesis

You can use parenthesis to change the order of operation.

```shot
let a: int = (1 + 2) * 3;
```

### Semicolon (Where should we put semicolon?)

You can put semicolon at the end of the statement.

```shot
# Expression
1 + 2;

# variable declaration
let a: int = 1;

# function declaration
let f: fn = (x: int, y: int): void {
  return x + y;
};

# Return
return 0;
```

Speaking of implementation, what I said "statement" is the following.

```rust
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Expression(ExpressionNode),
    DeclarationOfFunction(Box<FunctionDeclarationNode>),
    DeclarationOfVariable(Box<VariableDeclarationNode>),
    Return(Box<ExpressionNode>),
}
```
