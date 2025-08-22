### Scilex

This is a learning project intended to understand **Pratt Parsers** and how to build a simple language from scratch.

---

###  What is this?
Scilex is a toy programming language project.  
At its core it currently:
- Takes mathematical expressions as input (+, -, *, /, ==, !=, >, <, >=, <=)
- Uses a Pratt Parser to generate an Abstract Syntax Tree (AST)
- Evaluates expressions to produce results

Example:
```bash
4 - 5 - (2 ^ 3 * 2) / 4
```

Produces AST: 
```bash
(- (- 4 5) (/ (group (* (^ 2 3) 2)) 4))
```

And evaluates to:
```bash
-4
```
