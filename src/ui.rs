use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style,
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};

pub fn ui_setup() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let _ = terminal.draw(|f| {
        let size_screen = f.size();

        let size = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(size_screen);

        // Shortcut
        let block = Block::default().title("Shortcut").borders(Borders::ALL);
        f.render_widget(block, size[0]);

        // Shortcut list
        let list = List::new(vec![
            ListItem::new("[N] New File"),
            ListItem::new("[D] Delete file"),
        ]);
        f.render_widget(list, size[0]);

        //Files and folders
        let size = Rect::new(0, 3, f.size().width, size_screen.height - 4);
        let block = Block::default().title("Explorer").borders(Borders::ALL);
        f.render_widget(block, size);
    });

    thread::sleep(Duration::from_millis(10000));

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
