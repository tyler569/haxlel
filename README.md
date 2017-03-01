# Haxlel
Haxlel is supposed to be a joke language while also allowing a program to be interpreted in a single pass. **The grammar may be changed at this stage to allow for the parsing of a source with no peeking to latter lexemes**. The interpreter will not include any standard libraries: the language is supposed to be something stupid and annoying.

Objectives:
* make a certain individual hate me for this.
* allow it to be interpreted in a single-pass with optimizations (generate AST while running it so it can be later optimized if the AST ever gets traversed in the future).
* allow it to be parsed with no tokens of look ahead.

## Problems
* ebnf-like grammar cannot represent simple things like a global terminator `?` without making things complicated.

## Program examples
### Hello, world!
```haxlel
.i
print"Hello, world\!?
```
Or use functions
```haxlel
.i
$ helloworld > print("Hello, world\!?

helloworld(!
```
```
.i$helloworld>print("Hello, world\!?helloWorld(!
```
Maybe less characters, eh?
```haxlel
.i
$ helloworld
  :print"Hello, world\!
?

helloworld(!
```
```haxlel
.i$helloworld:print"Hello, world\!?helloWorld(!
```

### Fibonacci
```haxlel
.i
$ fibs n
  if n == 0
    > 0
  elif n == 1
    > 1
  !
  
  > fibs(n-2! + fibs(n-1
?
```
```haxlel
.i$fibsNifN==0>0elifN==1>1!>fibs(N-2!+fibs(N-1?
```
Boolean operator
```haxlel
.i
$ fibs n
  if n == 0 | n == 1
    > n
  !
  
  >fibs(n-2! + fibs(n-1!
!
```
```haxlel
.i$fibsNifN==0|N==1>N!>fibs(N-2!+fibs(N-1?
```
Pattern matching
```haxlel
.i
$ fibs 0 > 0
$ fibs 1 > 1
$ fibs n > fibs(n-2! + fibs(n-1!
```
```haxlel
.i$fibs0>0$fibs1>1$fibsN>fibs(N-2!+fibs(N-1!
```
Non-recursive
```haxlel
.i
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
.i$fibsN=0a=1b#N>0=b t=b+a b=t a=N-1 N!>a!
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
$factNifN==0>1!>N*fact(N-1?
```
Pattern matching
```haxlel
$ fact 0 > 1
$ fact n > n * fact(n-1!
```
```haxlel
$fact0=1$factN=N*fact(N-1!
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
$fact N=1r#N>1=r*Nr=N-1N!>r!
```
