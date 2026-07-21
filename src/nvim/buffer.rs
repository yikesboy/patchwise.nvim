use std::path::PathBuf;

use nvim_oxi::api;
use nvim_oxi::api::Buffer;
use nvim_oxi::api::opts::OptionOpts;
use nvim_oxi::api::opts::SetExtmarkOpts;

use crate::error::PatchwiseError;
use crate::error::Result;
use crate::nvim::selection::BufferPosition;
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

    pub fn from_handle(handle: i32) -> Self {
        Self {
            inner: Buffer::from(handle),
        }
    }

    pub fn handle(&self) -> i32 {
        self.inner.handle()
    }

    pub fn is_valid(&self) -> bool {
        self.inner.is_valid()
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
        self.inner.get_name().map_err(PatchwiseError::BufferRead)
    }

    pub fn file_type(&self) -> Result<String> {
        let opts = OptionOpts::builder().buffer(self.inner.clone()).build();
        api::get_option_value("filetype", &opts).map_err(PatchwiseError::BufferRead)
    }

    pub fn create_extmark(
        &mut self,
        namespace: u32,
        position: BufferPosition,
        right_gravity: bool,
    ) -> Result<u32> {
        let opts = SetExtmarkOpts::builder()
            .right_gravity(right_gravity)
            .build();

        self.inner
            .set_extmark(namespace, position.row, position.col, &opts)
            .map_err(PatchwiseError::SelectionTracking)
    }

    pub fn extmark_position(&self, namespace: u32, extmark: u32) -> Result<BufferPosition> {
        let (row, col, _) = self
            .inner
            .get_extmark_by_id(namespace, extmark, &Default::default())
            .map_err(PatchwiseError::SelectionTracking)?;

        Ok(BufferPosition { row, col })
    }

    pub fn delete_extmark(&mut self, namespace: u32, extmark: u32) -> Result<()> {
        self.inner
            .del_extmark(namespace, extmark)
            .map_err(PatchwiseError::SelectionTracking)
    }
}
