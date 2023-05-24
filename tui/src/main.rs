use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute, terminal,
};
use std::io;

fn run<W>(w: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    execute!(w, terminal::EnterAlternateScreen)?;

    terminal::enable_raw_mode()?;

    println!("Hello, world!");
    println!("Press 'q' to quit!");

    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            kind: KeyEventKind::Press,
            modifiers: _,
            state: _,
        })) = event::read()
        {
            if c == 'q' {
                println!("Goodbye, world!");
                break;
            }
        }
    }

    terminal::disable_raw_mode()
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let mut stdout = io::stdout();
    run(&mut stdout)
}
