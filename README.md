# Haxlel
Haxlel is supposed to be a joke language while also allowing a program to be interpreted in a single pass. **The grammar may be changed at this stage to allow for the parsing of a source with no peeking to latter lexemes**. The interpreter will not include any standard libraries: the language is supposed to be something stupid and annoying.

## Program examples
### Hello, world!
```haxlel
$main:print"Hello, world\!?
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
.i$fibs n if n==0>0elif n==1>1!>fibs(n-2!+fibs(n-1?
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
.i$fibs n if n==0|n==1>n!>fibs(n-2!+fibs(n-1?
```
Pattern matching
```haxlel
.i
$ fibs 0 > 0
$ fibs 1 > 1
$ fibs n > fibs(n-2! + fibs(n-1!
```
```haxlel
.i$fibs0>0$fibs1>1$fibs n>fibs(n-2!+fibs(n-1!
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
.i$fibs n=0a=1b#n>0=b t=b+a b=t a=n-1 n!>a!
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
$fact n if n==0>1!>n*fact(n-1?
```
Pattern matching
```haxlel
$ fact 0 > 1
$ fact n > n * fact(n-1!
```
```haxlel
$fact0=1$fact n=n*fact(n-1!
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
$fact n=1r#n>1=r*n r=n-1 n!>r!
```
