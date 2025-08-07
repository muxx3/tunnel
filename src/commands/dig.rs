use crate::commands::send;
use crate::network::discovery;
use anyhow::Result;
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::{self, Write};

pub async fn handle() -> Result<()> {
    println!("Starting peer discovery...");

    let mut peers = discovery::discover_peers().await?;

    if peers.is_empty() {
        println!("No tunnels found on local network.");
        return Ok(());
    }

    peers.push("Quit".to_string());

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let mut selected = 0;

    loop {
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

        let mut current_y = 0;

        execute!(stdout, MoveTo(0, current_y))?;
        println!("Select a tunnel to bury your file into (j/k or arrows, Enter to confirm):");
        current_y += 2;

        for (i, peer) in peers.iter().enumerate() {
            execute!(stdout, MoveTo(0, current_y))?;
            if i == selected {
                execute!(stdout, SetForegroundColor(Color::Cyan))?;
                print!("> {}", peer);
                execute!(stdout, ResetColor)?;
            } else {
                print!("  {}", peer);
            }
            println!();
            current_y += 1;
        }

        stdout.flush()?;

        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        println!("\nQuitting...");
                        return Ok(());
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        selected = (selected + 1) % peers.len();
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        if selected == 0 {
                            selected = peers.len() - 1;
                        } else {
                            selected -= 1;
                        }
                    }
                    KeyCode::Enter => {
                        let chosen = &peers[selected];
                        if chosen == "Quit" {
                            disable_raw_mode()?;
                            println!("\nQuitting...");
                            return Ok(());
                        }

                        disable_raw_mode()?;
                        println!("\nSelected tunnel: {}", chosen);

                        println!("Enter file paths to send (space-separated):");
                        let mut input = String::new();
                        io::stdin().read_line(&mut input)?;
                        let inputs: Vec<String> =
                            input.split_whitespace().map(|s| s.to_string()).collect();

                        send::handle(inputs, chosen.to_string()).await?;

                        println!("File sent to {}", chosen);
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
    }
}
