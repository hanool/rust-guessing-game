use crossterm::{
    cursor::{Hide, MoveTo},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    style::Stylize,
    terminal,
};
use pad::{Alignment, PadStr};
use std::io;

fn run<W>(w: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    // Initialize app
    execute!(w, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    execute!(w, Hide, MoveTo(0, 0))?;

    display_title()?;

    loop {
        match read_char()? {
            'q' => {
                println!("Goobye!");
                break;
            }
            _ => {}
        }
    }

    // Clean up app, be a good ciitzen
    terminal::disable_raw_mode()?;
    execute!(w, terminal::LeaveAlternateScreen)
}

fn display_title() -> io::Result<()> {
    let size = terminal::size()?;
    let width: usize = size.0.into();
    let title: &str = include_str!("../assets/title.txt");
    let title = title.split("\n");

    for row in title {
        println!(
            "{}",
            row.pad_to_width_with_alignment(width, Alignment::Middle)
                .stylize()
                .bold()
        );
    }

    println!(
        "{}",
        "Press Enter to start!"
            .pad_to_width_with_alignment(width, Alignment::Middle)
            .slow_blink()
    );

    Ok(())
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
    let mut stdout = io::stdout();
    run(&mut stdout)
}
