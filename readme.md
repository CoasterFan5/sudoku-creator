# Sudoku Creator
Originally designed to create sudoku puzzles, this project is now working towards creating sudoku puzzles and rating them.

## Human solve log syntax system
```
r{n}c{n2} is the row and column
={n} means that n was placed in that cell
<{n}> means that the candidate n was removed from that cell

Examples:
r1c1=6 means that the very top left square is a six
r9c9<1> means 1 was removed as a candidate from the bottom right square
```

## A note on comments
This code is very over-commented and under self-explaining. While making this project I am also learning Rust. Sorry :^)
