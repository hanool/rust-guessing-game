use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute, style, terminal,
};
use std::io;

fn run<W>(w: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    execute!(w, terminal::EnterAlternateScreen)?;

    terminal::enable_raw_mode()?;

    let title: &str = include_str!("../assets/title.txt");

    println!("{}", title);
    println!("Press 'q' to quit!");

    loop {
        match read_char()? {
            'q' => {
                println!("Goobye!");
                break;
            }
            _ => {}
        }
    }

    terminal::disable_raw_mode()
}

fn read_char() -> io::Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            kind: KeyEventKind::Press,
            modifiers: _,
            state: _,
        })) = event::read()
        {
            return Ok(c);
        }
    }
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let mut stdout = io::stdout();
    run(&mut stdout)
}
