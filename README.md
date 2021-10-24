# ku

ku is a commandline sudoku tool (and future game) written in rust

## Usage

ku supports a few sub commands:

### solve

Solves a given sudoku

```
ku solve 900800000000000500000000000020010003010000060000400070708600000000030100400000200
972853614146279538583146729624718953817395462359462871798621345265934187431587296
```

### generate

(Not implemented yet)

Takes a difficulty level and generates a new puzzle

### encode

(Not implemented yet)

Encodes a given puzzle in a compressed format for easier exchange with others

### decode

(Not implemented yet)

Decodes a given puzzle from a compressed format to a simple series of numbers

### play

(Not implemented yet)

Allows you to play a game of sudoku in the console. You can either give it an existing puzzle (compressed or
uncompressed) or a difficulty, so it can generate a puzzle for you.
