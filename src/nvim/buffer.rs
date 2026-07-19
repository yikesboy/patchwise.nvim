use std::path::PathBuf;

use nvim_oxi::api;
use nvim_oxi::api::Buffer;
use nvim_oxi::api::opts::OptionOpts;

use crate::error::PatchwiseError;
use crate::error::Result;
use nvim_oxi as oxi;

use crate::nvim::selection::TextRange;

pub struct PatchwiseBuffer {
    inner: Buffer,
}

impl PatchwiseBuffer {
    pub fn current() -> Self {
        Self {
            inner: oxi::api::get_current_buf(),
        }
    }

    pub fn get_mark(&self, mark: char) -> Result<(usize, usize)> {
        self.inner
            .get_mark(mark)
            .map_err(PatchwiseError::SelectionRead)
    }

    pub fn get_line(&self, row: usize) -> Result<String> {
        self.inner
            .get_lines(row..=row, false)
            .map_err(PatchwiseError::SelectionRead)?
            .next()
            .map(|line| line.to_string())
            .ok_or(PatchwiseError::NoSelection)
    }

    pub fn replace(&mut self, range: TextRange, replacement: &str) -> Result<()> {
        let replacement_lines: Vec<String> = replacement.split('\n').map(String::from).collect();

        self.inner
            .set_text(
                range.start.row..range.end.row,
                range.start.col,
                range.end.col,
                replacement_lines,
            )
            .map_err(PatchwiseError::BufferEdit)
    }

    pub fn read(&self, range: TextRange) -> Result<String> {
        let opts = Default::default();
        let lines = self
            .inner
            .get_text(
                range.start.row..range.end.row,
                range.start.col,
                range.end.col,
                &opts,
            )
            .map_err(PatchwiseError::SelectionRead)?;

        Ok(lines
            .map(|line| line.to_string())
            .collect::<Vec<_>>()
            .join("\n"))
    }

    pub fn text(&self) -> Result<String> {
        let lines = self
            .inner
            .get_lines(.., false)
            .map_err(PatchwiseError::BufferRead)?;

        Ok(lines
            .map(|line| line.to_string())
            .collect::<Vec<String>>()
            .join("\n"))
    }

    pub fn file_path(&self) -> Result<PathBuf> {
        Ok(self.inner.get_name().map_err(PatchwiseError::BufferRead)?)
    }

    pub fn file_type(&self) -> Result<String> {
        let opts = OptionOpts::builder().buffer(self.inner.clone()).build();
        api::get_option_value("filetype", &opts).map_err(PatchwiseError::BufferRead)
    }
}
