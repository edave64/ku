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

Encodes a given puzzle in a compressed base64 format for easier exchange with others.
This format has a variable length, and is a lot more efficient for unsolved sudokus, but still
always smaller than the traditional 81 character strings.

```
> ku encode 900800000000000500000000000020010003010000060000400070708600000000030100400000200
mo_F_SobMB1sS3BwhvowGk0q
> ku encode 972853614146279538583146729624718953817395462359462871798621345265934187431587296
lyhTYUFGJ5U4WDFGcpYkcYlTgXOVRiNZRihxeYYhNFJlk0GHQxWHKWA
```

### decode

Decodes a given puzzle from a compressed base64 format to a simple series of numbers

```
> ku decode mo_F_SobMB1sS3BwhvowGk0q
900800000000000500000000000020010003010000060000400070708600000000030100400000200
> ku decode lyhTYUFGJ5U4WDFGcpYkcYlTgXOVRiNZRihxeYYhNFJlk0GHQxWHKWA
972853614146279538583146729624718953817395462359462871798621345265934187431587296
```

### play

(Not implemented yet)

Allows you to play a game of sudoku in the console. You can either give it an existing puzzle (compressed or
uncompressed) or a difficulty, so it can generate a puzzle for you.
