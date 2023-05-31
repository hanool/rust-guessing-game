use crossterm::{
    cursor::{Hide, MoveTo, MoveToColumn, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    style::Stylize,
    terminal::{self, Clear, ClearType},
};
use pad::{Alignment, PadStr};
use rand::{thread_rng, Rng};
use std::io;
use std::thread;
use std::{cmp::Ordering, time::Duration};

fn run<W>(w: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    // Initialize app
    execute!(w, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    execute!(w, Hide, MoveTo(0, 0))?;

    let mut is_game_over = false;

    while !is_game_over {
        display_title()?;

        loop {
            match process_keypress()? {
                KeyCode::Char(c) => match c {
                    'q' => {
                        println!("Goodbye!");
                        is_game_over = true;
                        break;
                    }
                    _ => {}
                },
                KeyCode::Enter => {
                    match process_game(w) {
                        Ok(_) => is_game_over = false,
                        Err(_) => is_game_over = true,
                    }
                    break;
                }
                _ => {}
            }
        }
    }

    // Clean up app, be a good ciitzen
    terminal::disable_raw_mode()?;
    execute!(w, Show)?;
    execute!(w, terminal::LeaveAlternateScreen)
}

fn display_title() -> io::Result<()> {
    let width: usize = terminal::size().unwrap().0.into();
    let title: &str = include_str!("../assets/title.txt");
    let title = title.split("\n");

    for row in title {
        let styled_line = row
            .pad_to_width_with_alignment(width, Alignment::Middle)
            .stylize()
            .bold();

        if row.ends_with("\r") {
            println!("{}", styled_line);
        } else {
            print!("{}", styled_line);
        }
    }

    println!(
        "{}",
        "Press Enter to start"
            .pad_to_width_with_alignment(width, Alignment::Middle)
            .slow_blink()
    );

    println!(
        "{}",
        "or 'q' to quit".pad_to_width_with_alignment(width, Alignment::Middle)
    );
    Ok(())
}

fn process_keypress() -> io::Result<KeyCode> {
    loop {
        match event::read() {
            Ok(e) => match e {
                Event::Key(KeyEvent {
                    code: k,
                    kind: KeyEventKind::Press,
                    modifiers: _,
                    state: _,
                }) => {
                    return Ok(k);
                }
                _ => {}
            },
            Err(_) => {}
        }
    }
}

fn process_game<W>(w: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    execute!(w, Clear(ClearType::All), MoveTo(0, 0))?;

    let mut rng = thread_rng();
    let answer: i32 = rng.gen_range(0..101);

    println!("the number is {}", answer);
    execute!(w, MoveToColumn(0))?;

    loop {
        println!(
            "{}",
            "Guess the Number!"
                .pad_to_width_with_alignment(terminal::size().unwrap().0.into(), Alignment::Middle)
        );
        execute!(w, MoveToColumn(0))?;
        let mut guessed_number = String::new();

        loop {
            match process_keypress() {
                Ok(KeyCode::Char(c)) => {
                    print!("{}", c);
                    w.flush()?;

                    match c.to_digit(10) {
                        Some(_) => {
                            guessed_number.push(c);
                        }
                        None => {}
                    }
                }
                Ok(KeyCode::Enter) => {
                    print!("\n");
                    w.flush()?;
                    execute!(w, MoveToColumn(0))?;
                    break;
                }
                _ => {}
            }
        }

        let guessed_number = match guessed_number.parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please input valid number".red());
                execute!(w, MoveToColumn(0))?;
                continue;
            }
        };

        match guessed_number.cmp(&answer) {
            Ordering::Less => println!("{}", "Too Small!".yellow().bold()),
            Ordering::Greater => println!("{}", "Too Big!".yellow().bold()),
            Ordering::Equal => {
                println!("{}", "You Win!".green().bold());
                thread::sleep(Duration::from_secs(1));
                break;
            }
        }
        execute!(w, MoveToColumn(0))?;
    }
    execute!(w, Clear(ClearType::All), MoveTo(0, 0))?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut stdout = io::stdout();
    run(&mut stdout)
}
