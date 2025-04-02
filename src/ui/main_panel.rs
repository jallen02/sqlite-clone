use std::{fs, io};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
    style::Style,
    widgets::Widget,
};
use thiserror::Error;

use crate::database::{Database, DatabaseReadError, header::FileFormatVersion};

use super::cli::Args;

pub struct MainPanel {
    database: Database,
}

impl MainPanel {
    fn new(database: Database) -> Self {
        Self { database }
    }
}

impl Widget for &MainPanel {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        buf.set_string(
            area.x,
            area.y,
            format!(
                "there are {} pages",
                self.database.header.database_size_in_pages
            ),
            Style::default(),
        );
        buf.set_string(
            area.x,
            area.y + 1,
            format!("each page is {} bytes", self.database.header.page_size),
            Style::default(),
        );
        buf.set_string(
            area.x,
            area.y + 2,
            format!(
                "database is in {:?} write mode - options are {:?} and {:?}",
                self.database.header.file_format_write_version,
                FileFormatVersion::Wal,
                FileFormatVersion::Legacy
            ),
            Style::default(),
        );
        buf.set_string(
            area.x,
            area.y + 3,
            format!(
                "database is in {:?} read mode - options are {:?} and {:?}",
                self.database.header.file_format_read_version,
                FileFormatVersion::Wal,
                FileFormatVersion::Legacy
            ),
            Style::default(),
        );
    }
}

pub fn start_ui(args: Args) -> Result<(), UiError> {
    let terminal = ratatui::init();
    let result = run(terminal, args);
    ratatui::restore();
    result
}

pub fn run(mut terminal: DefaultTerminal, args: Args) -> Result<(), UiError> {
    let db_bytes = fs::read(args.filepath.clone()).map_err(UiError::IoError)?;
    let database = Database::from_bytes(db_bytes).map_err(UiError::DatabaseReadError)?;
    let panel = MainPanel::new(database);
    loop {
        terminal
            .draw(|frame| render(frame, &panel))
            .map_err(UiError::IoError)?;
        if matches!(event::read().map_err(UiError::IoError)?, Event::Key(_)) {
            break Ok(());
        }
    }
}

pub fn render(frame: &mut Frame, panel: &MainPanel) {
    frame.render_widget(panel, frame.area());
}

#[derive(Error, Debug)]
pub enum UiError {
    #[error("Encountered error reading the database: {0}")]
    DatabaseReadError(DatabaseReadError),
    #[error("Encountered an IO Error: {0}")]
    IoError(io::Error),
}
