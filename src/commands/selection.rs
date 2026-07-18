use std::mem::swap;

use crate::error::{PatchwiseError, Result};
use crate::notify;
use nvim_oxi as oxi;
use oxi::api::types::CommandArgs;

pub fn run(_args: CommandArgs) -> Result<()> {
    let selection = read_visual_selection()?;
    notify::info(&selection.text);
    Ok(())
}

pub struct Selection {
    pub start_row: usize,
    pub start_col: usize,
    pub end_row: usize,
    pub end_col: usize,
    pub text: String,
}

mod marks {
    pub const VISUAL_START_MARK: char = '<';
    pub const VISUAL_END_MARK: char = '>';
}

fn read_visual_selection() -> Result<Selection> {
    let buffer = oxi::api::get_current_buf();

    let (mut start_row, start_col) = buffer
        .get_mark(marks::VISUAL_START_MARK)
        .map_err(PatchwiseError::SelectionRead)?;

    let (mut end_row, mut end_col) = buffer
        .get_mark(marks::VISUAL_END_MARK)
        .map_err(PatchwiseError::SelectionRead)?;

    if start_row == 0 || end_row == 0 {
        Err(PatchwiseError::NoSelection)?;
    }

    start_row -= 1;
    end_row -= 1;

    let mut start = (start_row, start_col);
    let mut end = (end_row, end_col);

    if end < start {
        swap(&mut start, &mut end);
    }

    end_col += 1;

    let opts = Default::default();
    let lines = buffer
        .get_text(start_row..=end_row, start_col, end_col, &opts)
        .map_err(PatchwiseError::SelectionRead)?;

    let text = lines
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    Ok(Selection {
        start_row,
        start_col,
        end_row,
        end_col,
        text,
    })
}
