use std::io;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

mod agent;
mod ui;
mod command;

use agent::{Agent, AgentMessage};

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();

    // Run the app
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui::draw(frame, app))?;

        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Esc => {
                                return Ok(());
                            }
                            KeyCode::Enter => {
                                if !app.input.is_empty() {
                                    // Check for /exit command
                                    if app.input.trim() == "/exit" {
                                        return Ok(());
                                    }
                                    app.send_command();
                                }
                            }
                            KeyCode::Backspace => {
                                app.input.pop();
                            }
                            KeyCode::Char(c) => {
                                app.input.push(c);
                            }
                            _ => {}
                        }
                    }
                }
                Event::Mouse(mouse_event) => {
                    match mouse_event.kind {
                        MouseEventKind::ScrollUp => {
                            // Scroll up: decrease scroll_offset
                            if app.scroll_offset > 0 {
                                app.scroll_offset -= 1;
                            }
                        }
                        MouseEventKind::ScrollDown => {
                            // Scroll down: increase scroll_offset
                            let max_scroll = app.messages.len().saturating_sub(1) as u16;
                            if app.scroll_offset < max_scroll {
                                app.scroll_offset += 1;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // Process agent messages
        app.process_messages();
    }
}

use std::time::Duration;

#[derive(Debug, Clone)]
enum Message {
    User(String),
    Agent(String),
    System(String),
    Error(String),
}

struct App {
    input: String,
    messages: Vec<Message>,
    agent: Agent,
    cursor_position: usize,
    thinking: bool,
    scroll_offset: u16,
}

impl App {
    fn new() -> Self {
        let agent = Agent::new();
        Self {
            input: String::new(),
            messages: vec![
                Message::System("Welcome to NULCODE! Type your command and press Enter.".to_string()),
                Message::System("Type '/exit' or press Esc to quit.".to_string()),
            ],
            agent,
            cursor_position: 0,
            thinking: false,
            scroll_offset: 0,
        }
    }

    fn send_command(&mut self) {
        let command = self.input.clone();
        self.messages.push(Message::User(command.clone()));
        self.input.clear();
        self.cursor_position = 0;
        self.thinking = true;

        // Send command to agent
        self.agent.execute_command(command);
    }

    fn process_messages(&mut self) {
        let mut has_new = false;
        while let Ok(msg) = self.agent.receiver.try_recv() {
            match msg {
                AgentMessage::Response(response) => {
                    self.messages.push(Message::Agent(response));
                    self.thinking = false;
                    has_new = true;
                }
                AgentMessage::Error(error) => {
                    self.messages.push(Message::Error(error));
                    self.thinking = false;
                    has_new = true;
                }
                AgentMessage::Status(status) => {
                    self.messages.push(Message::System(status));
                    has_new = true;
                }
            }
        }
        // Auto-scroll to bottom when new messages arrive
        if has_new {
            self.scroll_offset = self.messages.len().saturating_sub(1) as u16;
        }
    }
}
