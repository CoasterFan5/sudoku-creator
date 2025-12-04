# Human solver

Weight is placed based on the difficulty of a human to use a particular technique.
Score is the sum of every technique used. The algorithm attempts to use easier techniques first.
Values are adopted from [HoDoKu's score system](https://sourceforge.net/p/hodoku/code/HEAD/tree/HoDoKu/trunk/src/sudoku/Options.java#l75)

Extra notes:
While HoDoKu considers naked singles and last digits / full houses to be distinct solving tecniques, I consider them to be the same. Therefore a naked single encompases both of these edge cases as it will lead to a cell with a single candidate left.

## Each tecnique explained in my own way: 
### Naked Single 
A single candidate is left for a cell

### Hidden Single 
A cell has multiple candidates but contains the only instance of a candidate in it's row, column, or house.

Ex: 0 0 0 0 0 0 0 0 0 0 3 0 0 4 9 8 0 0 8 6 0 0 0 0 9 0 2 0 1 4 3 0 0 0 7 6 0 0 0 6 0 2 0 0 0 3 0 0 0 9 0 0 0 0 0 0 0 0 6 0 0 8 4 0 0 0 2 0 0 0 5 0 0 0 7 4 0 3 0 0 0

r4c1

## Difficulty Levels

```
Easy <= 200,
Medium <= 300,
Hard <= 400,
Very Hard <= 500,
Extreme >= 1000

## Scoring Values
```
Naked Single: 1
Hidden Single: 3
Locked Candidates: 10
```
