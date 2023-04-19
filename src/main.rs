use std::{io, thread, time::Duration};

use crossterm::{
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Terminal,
};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();

        let vert_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
            .split(size);

        let horiz_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Max(3)].as_ref())
            .split(vert_chunks[0]);

        let block1 = Block::default().title("1").borders(Borders::ALL);
        f.render_widget(block1, horiz_chunks[0]);

        let block2 = Block::default().title("2").borders(Borders::ALL);
        f.render_widget(block2, horiz_chunks[1]);

        let block3 = Block::default().title("3").borders(Borders::ALL);
        f.render_widget(block3, vert_chunks[1]);
    })?;

    thread::sleep(Duration::from_secs(10));

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
