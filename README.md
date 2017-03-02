# Haxlel
## Description
Haxlel is supposed to be a joke language while also allowing a program to be interpreted in a single pass. **The grammar may be changed at this stage to allow for the parsing of a source with no peeking to latter lexemes**. The interpreter will not include any standard libraries: the language is supposed to be something stupid and annoying.

## Paradigms
* Imperative
* Procedural
* Structured
* Functional

## Objectives
* make a certain individual hate me for this.
* allow it to be interpreted in a single-pass with optimizations (generate AST while running it so it can be later optimized if the AST ever gets traversed in the future).
* allow it to be parsed with no tokens of look ahead.

## Problems
* ebnf-like grammar cannot represent simple things like a global terminator `?` without making things complicated.

## Program examples
### Hello, world!
```haxlel
:println"Hello, world\!?
```
Or use functions
```haxlel
$ helloworld > :println"Hello, world\!?

:helloworld!
```
```
$helloworld>:println"Hello, world\!?:helloWorld!
```
Maybe less characters, eh?
```haxlel
$ helloworld
  :println"Hello, world\!
?

:helloworld!
```
```haxlel
$helloworld:println"Hello, world\!?:helloWorld!
```

### Fibonacci
```haxlel
$ fibs n
  if n == 0
    > 0
  elif n == 1
    > 1
  !
  
  > :fibs n-2! + :fibs n-1
?
```
```haxlel
$fibsNifN==0>0elifN==1>1!>:fibsN-2!+:fibsN-1?
```
Boolean operator
```haxlel
$ fibs n
  if n == 0 | n == 1
    > n
  !
  
  >:fibs n-2! + :fibs n-1
?
```
```haxlel
$fibsNifN==0|N==1>N!>:fibsN-2!+:fibsN-1?
```
Pattern matching
```haxlel
$ fibs 0 > 0
$ fibs 1 > 1
$ fibs n > :fibs n-2! + :fibs n-1!
```
```haxlel
$fibs0>0$fibs1>1$fibsN>:fibsN-2!+:fibsN-1!
```
Non-recursive
```haxlel
$ fibs n
  = 0 a
  = 1 b
  
  # n > 0
    = b temp
    
    = b + a b
    = temp a
    
    = n - 1 n
  !
  
  > a
!
```
```haxlel
$fibsN=0a=1b#n>0=bT=b+aB=tA=n-1n!>a!
```

### Factorial
```haxlel
$ fact n
  if n == 0
    > 1
  !
  
  > n * fact(n-1
?
```
```haxlel
$factNifN==0>1!>n*:factN-1?
```
Pattern matching
```haxlel
$ fact 0 > 1
$ fact n > n * fact(n-1!
```
```haxlel
$fact0>1$factN>n*:factN-1!
```
Non-recursive
```haxlel
$ fact n
  = 1 result
  
  # n > 1
    = result * n result
    = n - 1 n
  !
  
  > result
!
```
```haxlel
$factN=1r#n>1=r*nR=n-1n!>r!
```
