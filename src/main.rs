
mod pols;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    terminal::{self, ClearType},
    ExecutableCommand,
};
use pols::cherries::OrbitalCherries;
use std::{
    io::{stdout, Stdout, Write},
    time::{Duration, Instant},
};

fn print_help() {
    println!("OrbitalCherries Manual Mode");
    println!("Usage: orbital_cherries [OPTIONS]");
    println!();
    println!("Options:");
    println!("  --help          Show this help page");
    println!("  --shell [type]  Print shell integration commands (bash, zsh, fish)");
    println!("                  Example: orbital_cherries --shell bash");
}

fn print_shell_integration(shell: &str) {
    match shell {
        "bash" => println!("alias cherries='orbital_cherries'"),
        "zsh" => println!("alias cherries='orbital_cherries'"),
        "fish" => println!("alias cherries 'orbital_cherries'"),
        _ => eprintln!("Unknown shell: {}", shell),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--help") {
        print_help();
        return;
    }
    if let Some(idx) = args.iter().position(|a| a == "--shell") {
        if let Some(shell) = args.get(idx + 1) {
            print_shell_integration(shell);
        } else {
            eprintln!("Error: --shell requires an argument (bash, zsh, fish)");
        }
        return;
    }

    let mut stdout: Stdout = stdout();

    // Setup terminal for animation
    terminal::enable_raw_mode().unwrap();
    stdout.execute(terminal::EnterAlternateScreen).unwrap();
    stdout.execute(cursor::Hide).unwrap();

    let (w, h) = terminal::size().unwrap_or((80, 24));
    let mut scene = OrbitalCherries::new(w, h);

    // Main animation loop
    loop {
        // Clear and draw scene
        stdout.execute(terminal::Clear(ClearType::All)).unwrap();
        scene.update(0.016);
        scene.draw(&mut stdout);
        stdout.flush().unwrap();

        // Check for keypresses
        if event::poll(Duration::from_millis(16)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                // Quit on 'q' or any key
                running_cleanup(&mut stdout);
                if key_event.code == KeyCode::Char('q') {
                    break;
                } else {
                    break;
                }
            }
        }

        std::thread::sleep(Duration::from_millis(16));
    }

    // Cleanup terminal on exit
    running_cleanup(&mut stdout);
}

fn running_cleanup(stdout: &mut Stdout) {
    stdout.execute(cursor::Show).ok();
    stdout.execute(terminal::LeaveAlternateScreen).ok();
    terminal::disable_raw_mode().ok();
}
