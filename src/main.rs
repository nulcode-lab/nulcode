use std::io;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
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
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            if app.input_mode != InputMode::Editing {
                                return Ok(());
                            }
                        }
                        KeyCode::Char('e') => {
                            if app.input_mode == InputMode::Normal {
                                app.input_mode = InputMode::Editing;
                            }
                        }
                        KeyCode::Enter => {
                            if app.input_mode == InputMode::Editing && !app.input.is_empty() {
                                app.send_command();
                            }
                        }
                        KeyCode::Backspace => {
                            if app.input_mode == InputMode::Editing {
                                app.input.pop();
                            }
                        }
                        KeyCode::Char(c) => {
                            if app.input_mode == InputMode::Editing {
                                app.input.push(c);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        // Process agent messages
        app.process_messages();
    }
}

use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug, Clone)]
enum Message {
    User(String),
    Agent(String),
    System(String),
    Error(String),
}

struct App {
    input: String,
    input_mode: InputMode,
    messages: Vec<Message>,
    agent: Agent,
    cursor_position: usize,
    thinking: bool,
}

impl App {
    fn new() -> Self {
        let agent = Agent::new();
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: vec![
                Message::System("Welcome to NULLCODE! Press 'e' to edit, 'q' to quit.".to_string()),
                Message::System("Type your command and press Enter to send.".to_string()),
            ],
            agent,
            cursor_position: 0,
            thinking: false,
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
        while let Ok(msg) = self.agent.receiver.try_recv() {
            match msg {
                AgentMessage::Response(response) => {
                    self.messages.push(Message::Agent(response));
                    self.thinking = false;
                }
                AgentMessage::Error(error) => {
                    self.messages.push(Message::Error(error));
                    self.thinking = false;
                }
                AgentMessage::Status(status) => {
                    self.messages.push(Message::System(status));
                }
            }
        }
    }
}
