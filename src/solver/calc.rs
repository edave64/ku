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

impl IntoIterator for Row {
    type Item = Cell;
    type IntoIter = IntoIter<Cell>;

    fn into_iter(self) -> Self::IntoIter {
        let start = start_of_row(self).0;
        (start..(start + 9)).into_iter().map(|x| Cell(x)).collect::<Vec<Cell>>().into_iter()
    }
}

impl IntoIterator for Col {
    type Item = Cell;
    type IntoIter = IntoIter<Cell>;

    fn into_iter(self) -> Self::IntoIter {
        let start = start_of_col(self).0;
        (0..9).into_iter().map(|x| Cell(start + x * 9)).collect::<Vec<Cell>>().into_iter()
    }
}

impl IntoIterator for Block {
    type Item = Cell;
    type IntoIter = IntoIter<Cell>;

    fn into_iter(self) -> Self::IntoIter {
        let start = start_of_block(self).0;
        (0..9).into_iter().map(|x| {
            let block_row = x / 3;
            let block_col = x % 3;
            Cell(start + block_col + block_row * 9)
        }).collect::<Vec<Cell>>().into_iter()
    }
}

pub fn row_of (cell: Cell) -> Row {
    Row(cell.0 / 9)
}

pub fn start_of_row (row: Row) -> Cell {
    Cell(row.0 * 9)
}

pub fn col_of (cell: Cell) -> Col {
    Col(cell.0 % 9)
}

pub fn start_of_col (col: Col) -> Cell {
    Cell(col.0)
}

pub fn block_of (cell: Cell) -> Block {
    Block(row_of(cell).0 / 3 * 3 + col_of(cell).0 / 3)
}

pub fn start_of_block (block: Block) -> Cell {
    Cell(block.0 / 3 * 27 + block.0 % 3 * 3)
}

pub fn number_to_mask (num: u8) -> u16 {
    1 << num
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 > 80 {
            write!(f, "Cell[Impossible Position]")
        } else {
            write!(f, "Cell[Col: {}, Row: {}]", col_of(self.clone()).0, row_of(self.clone()).0)
        }
    }
}