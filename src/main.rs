use std::io;

use crossterm::{
    execute,
    terminal::{
        self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
    },
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let baslik = Paragraph::new("HELLO CHEF")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length((3)), Constraint::Min((10))])
                .split(f.size());

            f.render_widget(baslik, chunks[0]);
            let mesaj = Paragraph::new("select a recipe pls")
                .block(Block::default().title("mesaj").borders(Borders::ALL));

            f.render_widget(mesaj, chunks[1])
        })?;
        std::thread::sleep(std::time::Duration::from_secs(1));
        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen)?;
    }

    Ok(())
}
