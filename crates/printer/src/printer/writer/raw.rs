use std::{cell::RefCell, fmt::Display, rc::Rc};

use crossterm::{queue, style::*, terminal::*};

use crate::Result;
#[derive(Debug, Clone)]
pub struct Raw<W> {
    pub raw: Rc<RefCell<W>>,
}
impl<W: std::io::Write> std::io::Write for Raw<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.raw.borrow_mut().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.raw.borrow_mut().flush()
    }
}

impl<W: std::io::Write> Raw<W> {
    pub fn scroll_up(&mut self, n: u16) -> Result<()> {
        queue!(self, ScrollUp(n))?;
        Ok(())
    }

    pub fn clear(&mut self, clear_type: ClearType) -> Result<()> {
        queue!(self, Clear(clear_type))?;
        Ok(())
    }

    pub fn _write<D: Display + Clone>(&mut self, value: D) -> Result<()> {
        queue!(self, Print(value))?;
        Ok(())
    }

    pub fn write<D: Display + Clone>(&mut self, value: D) -> Result<()> {
        self._write(value)
    }

    pub fn write_with_color<D: Display + Clone>(&mut self, value: D, color: Color) -> Result<()> {
        self.set_fg(color)?;
        self.write(value)?;
        self.reset_color()?;
        Ok(())
    }

    pub fn reset_color(&mut self) -> Result<()> {
        queue!(self, ResetColor)?;
        Ok(())
    }

    pub fn set_fg(&mut self, color: Color) -> Result<()> {
        queue!(self, SetForegroundColor(color))?;
        Ok(())
    }

    pub fn set_bg(&mut self, color: Color) -> Result<()> {
        queue!(self, SetBackgroundColor(color))?;
        Ok(())
    }

    pub fn set_title(&mut self, title: &str) -> Result<()> {
        queue!(self, SetTitle(title))?;
        Ok(())
    }
}
