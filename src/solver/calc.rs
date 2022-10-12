extern crate alloc;
use alloc::vec::IntoIter;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Cell(pub u8);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Row(pub u8);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Col(pub u8);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Block(pub u8);

pub trait House: IntoIterator<Item = Cell, IntoIter = IntoIter<Cell>> {
    fn start(self) -> Cell;
}

impl Cell {
    pub fn row(self) -> Row {
        Row(self.0 / 9)
    }

    pub fn col(self) -> Col {
        Col(self.0 % 9)
    }

    pub fn block(self) -> Block {
        let Row(row) = self.row();
        let Col(col) = self.col();

        Block(row / 3 * 3 + col / 3)
    }
}

impl IntoIterator for Row {
    type Item = Cell;
    type IntoIter = IntoIter<Cell>;

    fn into_iter(self) -> Self::IntoIter {
        let start = self.start().0;
        (start..(start + 9))
            .into_iter()
            .map(Cell)
            .collect::<Vec<Cell>>()
            .into_iter()
    }
}

impl House for Row {
    fn start(self) -> Cell {
        Cell(self.0 * 9)
    }
}

impl IntoIterator for Col {
    type Item = Cell;
    type IntoIter = IntoIter<Cell>;

    fn into_iter(self) -> Self::IntoIter {
        let start = self.start().0;
        (0..9)
            .into_iter()
            .map(|x| Cell(start + x * 9))
            .collect::<Vec<Cell>>()
            .into_iter()
    }
}

impl House for Col {
    fn start(self) -> Cell {
        Cell(self.0)
    }
}

impl IntoIterator for Block {
    type Item = Cell;
    type IntoIter = IntoIter<Cell>;

    fn into_iter(self) -> Self::IntoIter {
        let start = self.start().0;
        (0..9)
            .into_iter()
            .map(|x| {
                let block_row = x / 3;
                let block_col = x % 3;
                Cell(start + block_col + block_row * 9)
            })
            .collect::<Vec<Cell>>()
            .into_iter()
    }
}

impl House for Block {
    fn start(self) -> Cell {
        Cell(self.0 / 3 * 27 + self.0 % 3 * 3)
    }
}

pub fn number_to_mask(num: u8) -> u16 {
    1 << num
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 > 80 {
            write!(f, "Cell[Impossible Position]")
        } else {
            write!(f, "Cell[Col: {}, Row: {}]", self.col().0, self.row().0)
        }
    }
}
