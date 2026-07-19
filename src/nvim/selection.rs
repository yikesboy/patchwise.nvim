use std::mem::swap;

use crate::error::{PatchwiseError, Result};
use crate::nvim::buffer::PatchwiseBuffer;

const VISUAL_START_MARK: char = '<';
const VISUAL_END_MARK: char = '>';

#[derive(Debug)]
pub struct Selection {
    pub range: TextRange,
    pub text: String,
}

impl Selection {
    pub fn current(buffer: &PatchwiseBuffer) -> Result<Self> {
        let range = TextRange::from_visual_marks(buffer)?;
        let text = buffer.read(range)?;
        Ok(Self { range, text })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TextRange {
    pub start: BufferPosition,
    pub end: BufferPosition,
}

impl TextRange {
    pub fn from_visual_marks(buffer: &PatchwiseBuffer) -> Result<Self> {
        let mut start = read_mark(buffer, VISUAL_START_MARK)?;
        let mut end = read_mark(buffer, VISUAL_END_MARK)?;

        if end < start {
            swap(&mut start, &mut end);
        }

        end.col = exclusive_end_column(buffer, end)?;
        Ok(Self { start, end })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BufferPosition {
    pub row: usize,
    pub col: usize,
}

impl BufferPosition {
    fn from_mark((row, col): (usize, usize)) -> Result<Self> {
        if row == 0 {
            return Err(PatchwiseError::NoSelection);
        }
        Ok(Self { row: row - 1, col })
    }
}

fn read_mark(buffer: &PatchwiseBuffer, mark: char) -> Result<BufferPosition> {
    let mark = buffer.get_mark(mark)?;
    BufferPosition::from_mark(mark)
}

fn exclusive_end_column(buffer: &PatchwiseBuffer, position: BufferPosition) -> Result<usize> {
    let line = buffer.get_line(position.row)?;

    if position.col >= line.len() {
        return Ok(line.len());
    }

    let selected_character = line
        .get(position.col..)
        .and_then(|remaining| remaining.chars().next())
        .ok_or(PatchwiseError::InvalidSelectionPosition {
            row: position.row,
            col: position.col,
        })?;

    Ok(position.col + selected_character.len_utf8())
}
