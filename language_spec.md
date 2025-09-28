# lipu lawa pi sitelen ilo
This is the specification and reference for *sitelen ilo*. You will need a UCSUR compatible font to read this properly.

## Goals
Be a grammatically correct programming language, in the vein of COBOL, but using *sitelen pona*. Maybe not the easiest to read, nor the best to program in, but certainly usable.

## Definitions
- *sitelen pona* may be abbreviated to SP.
- *toki pona* may be abbreviated to TP.

## File info
- *sitelen ilo* programs take the file extension `.lipu`, meaning "document" in TP.
- Programs may start with a shebang line, detected as the first line starting with `#!`. This is ignored by the runtime.
- For simplicity and legibility, combining characters are not allowed.

## General syntax
- Statements are delineated by line breaks, as is common practice in SP.
- Comments are started with the word *len* (󱤥), meaning "hidden" in TP. Comments are themselves statements and cannot occur inline with a statement.
```
󱤥　󱥁󱤧󱥠󱤎󱤀
󱤎󱥄󱥬󱤉󱥬「󱥬󱤀　󱤰󱤄󱥄」
󱤎󱥄󱥬󱤉󱥬「󱥁󱤧󱥠󱤎」
```

A statement generally looks like this:
```
(target)󱥄[action](arguments)
```
The *target* is an object that can have actions done using them. The *action* is at least one word and may be a preposition (as they function as content words). Arguments are always preceded by *e* (󱤉) or a preposition (*tawa*, *tan*, *kepeken*, *sama*) (󱥩　󱥧　󱤙　󱥖); arguments by with *e* must come before those using a preposition.

Multiple actions can be stated in a single line by repeating *o* like this:
```
(target)󱥄[action1](arguments1)󱥄[action2](arguments2)
```
However, if both actions use *ni* (see below), this may 

Intrinsic operations do not take a target. See that section for more details.

## Variables, literals and data types
There are 3 data types in *sitelen ilo*:
- *nanpa* (󱤽), 64-bit signed integer
- *lon* (󱤬), boolean value
- *toki* (󱥬), a UTF-8 string

*Literals* use the type name followed by the value in CJK corner brackets:
```
󱥬「󱥬󱥔」
󱤽「󱤼󱤼󱤼󱤭󱥮󱥮」
󱤬「󱤬󱤂」
```

*Variables* use the type name followed by the name, in a cartouche. Pronunciation does not matter, the runtime only cares about the spelling.
```
󱥬󱦐󱥲󱤌󱤬󱤋󱦑
󱤽󱦐󱥤󱦜󱤧󱦜󱦑
󱤬󱦐󱥡󱦝󱦑
```

To assign a variable, simply use `[variable]󱥄[value]`.
```
󱤽󱦐󱥣󱦝󱦑󱥄󱤽「󱥳󱤄󱤄󱤄」
```

Variables must always be declared before first use using *o sin* (󱥄󱥝, be new), however, they can also have be assigned immediately after declaration:
```

󱤥　󱤩󱥁
󱤽󱦐󱥣󱦝󱦑󱥄󱥝󱥄󱤽「󱥳󱤄󱤄󱤄」
󱤥　󱤧󱥖󱥁
󱤽󱦐󱥣󱦝󱦑󱥄󱥝
󱤽󱦐󱥣󱦝󱦑󱥄󱤽「󱥳󱤄󱤄󱤄」
```

### *nanpa* (numbers)
Number literals are specified in [nasin nanpa pona](https://sona.pona.la/wiki/nasin_nanpa_pona). As an extension, adding *weka* (󱥶) to the end creates a negative number.
```
󱤥 -2048
󱤽「󱤼󱤄󱤼󱤼󱤭󱥮󱥳󱥶」
󱤥 -15
󱤽「󱤭󱤭󱤭󱥶」
󱤥 0
󱤽「󱤂」
󱤥 15
󱤽「󱤭󱤭󱤭」
󱤥 2048
󱤽「󱤼󱤄󱤼󱤼󱤭󱥮󱥳」
```

### *lon* (booleans)
Boolean literals are either *lon* (󱤬, true) or *ala* (󱤂, false).

### *toki* (strings)
String literals can be any UTF-8 string. To escape corner brackets, type them twice.
```
󱥬「󱤴󱥬󱤉󱥁「「󱥠󱤎󱤧󱥵󱥣󱤀」」」
```

## *ni* and operations
*ni* (󱥁, this/that) is a special dynamically-typed variable containing the last result of an operation, if one is present. However, all values need a type, so it must be prefixed with the desired type (so: 󱤽󱥁　󱥬󱥁). An error is raised if *ni* is not the correct type when accessed.

*Operations* are special functions that do not use a target as they operate on simple data types.

### Manipulating *ni*
```
󱤥 Set ni to a variable or literal.
󱥄󱥡󱤉[value]
```

### Arithmetic
All arithmetic operations save their result to *ni*.
```
󱤥 Addition. ni = n1 + n2 + n3 + ...
󱥄󱥳󱤉[n1]󱤉[n2]󱤉[n3]

󱤥 Subtraction. ni = k - n1 - n2 - ...
󱥄󱥶󱤉[n1]󱤉[n2]󱥧[k]

󱤥 Multiplication. ni = n1 * n2 * n3 * ...
󱥄󱤼󱤉[n1]󱤉[n2]󱤉[n3]

󱤥 Division. ni = k / n1 / n2 / ...
󱥄󱥻󱤉[k]󱥩[n1]󱥩[n2]

󱤥 Remainder. ni = k % n1
󱥄󱥌󱥻󱤉[k]󱤙[n1]
```

### String manipulation
All string operations save their results to *ni*. Indices are zero-indexed like real programmers always do.
```
󱤥 Concatenation. ni = "".join([s1, s2, s3, ...])
󱥄󱥳󱤩󱤉[s1]󱤉[s2]󱤉[s3]

󱤥 Extract one character. ni = s[i]
󱥄󱤓󱥂󱤉[i]󱥧[s]

󱤥 Extract a substring. ni = s[i:j]
󱥄󱤓󱤩󱤉[i]󱥩[j]󱥧[s]

󱤥 Convert a character to its Unicode codepoint.
󱥄󱤽󱥂󱤉[c]

󱤥 Convert a Unicode codepoint to its character.
󱥄󱥂󱤽󱤉[i]
```

### Conversion
- String conversions use the same logic as *sitelen pona* literals.
- Non-zero integers are converted to true and zero to false.
- Boolean true is converted to 1 and false to 0.
- Conversion operations save their result to *ni*.
```
󱤥 Convert to integer. ni = int(s)
󱥄󱤆󱤽󱤉[s]

󱤥 Convert to boolean. ni = bool(s)
󱥄󱤆󱤬󱤉[s]

󱤥 Convert to string. ni = str(s)
󱥄󱤆󱥬󱤉[s]
```

## Conditions
Conditions are simply sentences that may be true or untrue. Here is a list of them.
```
󱤥 a == b (types must match)
[a]󱤧[b]

󱤥 a > b (numbers only)
[a]󱤧󱥣󱥩[b]
󱤥 a < b (numbers only)
[a]󱤧󱤨󱥩[b]

󱤥 p1 & p2 & p3 == q (booleans only, arbitrarily many arguments on the left)
[p1]󱤊[p2]󱤊[p3]󱤧[q]

󱤥 p1 | p2 | p3 == q (booleans only, arbitrarily many arguments on the left)
[p1]󱤇[p2]󱤇[p3]󱤧[q]
```

While a direct equivalent to `NOT` doesn't exist, you can instead use equivalence to *ala* as a proxy for that.

*ken la* followed by a condition evaluates the condition to *ni* as a boolean.
```
󱤘󱤡󱤬󱦐󱤌󱦝󱦑󱤧󱤬「󱤂」
󱤬󱦐󱤌󱦝󱦑󱥄󱤬󱥁
```

## Flow control
Scoped blocks can be initiated using *o pali* (󱥄󱥉) and closed using *pini* (󱥐).
```
󱥄󱥉
    󱤥 do stuff here...
󱥐
```

These behave as a single statement for the purpose of the below.

### Conditional statements
Preceding any statement, including scoped blocks, with a condition followed by *la* (󱤡) creates an if-statement.
```
[cond1]󱤡󱥄󱥉
    󱤥 cond1 is true...
󱥐
```
*pini* may not be preceded by a condition.

### Chaining conditions
The phrase *ala la* (if not) can be used to create else-if and else statements.
```
[cond1]󱤡󱥄󱥉
    󱤥 cond1 is true...
󱤂󱤡[cond2]󱤡󱥄󱥉
    󱤥 cond2 is true...
󱤂󱤡󱥄󱥉
󱥐
```

### Loops
The phrase *o sike* (󱥄󱥜) begins a loop block. Like conditional blocks, they are closed with *pini*. Without any condition, this loops forever.
```
󱥄󱥜
    󱤥 this runs forever
󱥐
```

To create a conditional loop, you apply a condition before *o sike*. This is checked at the start of each iteration, like a while-loop in C.
```
[cond1]󱤡󱥄󱥜
    󱤥 this runs until cond1 is false
󱥐
```

To break a loop early, you can use the special statement *sike o pini* (󱥜󱥄󱥐), and to jump to the top of the loop, you can use *sike o sin* (󱥜󱥄󱥝).

## *ilo* and system functions
*ilo* (󱤎; tool, device, machine) is a special global target, representing the system, or outside world beyond the runtime.

### Console I/O
```
󱤥 Concatenate and print arguments. This does not affect ni.
󱤎󱥄󱥬󱤉[s1]󱤉[s2]󱤉[s3]

󱤥 Print a line separator. This does not affect ni.
󱤎󱥄󱥐󱤩

󱤥 Read a line of input to ni.
󱤎󱥄󱥷󱤩
```