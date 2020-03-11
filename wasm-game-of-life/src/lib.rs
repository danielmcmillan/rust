extern crate fixedbitset;
mod utils;

use fixedbitset::FixedBitSet;
use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
    next_cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();
        let width = 128;
        let height = 128;
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            if i % 2 == 0 || i % 7 == 0 {
                cells.set(i, true);
            }
        }

        Universe {
            width,
            height,
            cells,
            next_cells: FixedBitSet::with_capacity(size),
        }
    }

    pub fn tick(&mut self) {
        self.next_cells.clone_from(&self.cells);
        {
            for row in 0..self.height {
                for col in 0..self.width {
                    let index = self.get_index(row, col);
                    let cell = self.cells[index];
                    let neighbour_count = self.live_neighbour_count(row, col);
                    self.next_cells.set(
                        index,
                        match (cell, neighbour_count) {
                            (true, 2..=3) => true,
                            (false, 3) => true,
                            _ => false,
                        },
                    );
                }
            }
        }
        std::mem::swap(&mut self.cells, &mut self.next_cells);
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn toggle(&mut self, row: u32, col: u32) {
        self.cells.toggle(self.get_index(row, col));
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }

    pub fn glider(&mut self, row: u32, col: u32) {
        self.cells
            .set(self.get_index(row + self.height() - 1, col), true);
        self.cells.set(self.get_index(row, col + 1), true);
        self.cells
            .set(self.get_index(row + 1, col + self.width() - 1), true);
        self.cells.set(self.get_index(row + 1, col), true);
        self.cells.set(self.get_index(row + 1, col + 1), true);
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        let row = match row {
            r if r < self.height() => r,
            r if r == self.height() => 0,
            r => r - self.height(),
        };
        let col = match col {
            c if c < self.width() => c,
            c if c == self.width() => 0,
            c => c - self.width(),
        };

        (row * self.width + col) as usize
    }

    fn live_neighbour_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for row_offset in [self.height - 1, 0, 1].iter().cloned() {
            for col_offset in [self.width - 1, 0, 1].iter().cloned() {
                if row_offset != 0 || col_offset != 0 {
                    let index = self.get_index(row + row_offset, col + col_offset);
                    count += self.cells[index] as u8;
                }
            }
        }
        count
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let symbol = if self.cells[self.get_index(row, col)] {
                    '◻'
                } else {
                    '◼'
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
