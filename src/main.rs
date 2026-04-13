use std::io;
use std::time::Duration;

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

mod agent;
mod command;
mod ui;

use agent::{Agent, AgentMessage};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    let res = run_app(&mut terminal, &mut app);

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

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui::draw(frame, app))?;

        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        if app.show_menu {
                            match key.code {
                                KeyCode::Up => {
                                    if app.menu_selection > 0 {
                                        app.menu_selection -= 1;
                                    }
                                }
                                KeyCode::Down => {
                                    if app.menu_selection < 1 {
                                        app.menu_selection += 1;
                                    }
                                }
                                KeyCode::Enter => match app.menu_selection {
                                    0 => {
                                        app.show_menu = false;
                                        app.input = "/model ".to_string();
                                        app.cursor_position = app.input.len();
                                    }
                                    1 => {
                                        return Ok(());
                                    }
                                    _ => {}
                                },
                                KeyCode::Esc => {
                                    app.show_menu = false;
                                }
                                _ => {}
                            }
                        } else {
                            match key.code {
                                KeyCode::Char('/') if app.input.is_empty() => {
                                    app.show_menu = true;
                                    app.menu_selection = 0;
                                }
                                KeyCode::Enter => {
                                    if !app.input.is_empty() {
                                        if app.input.trim() == "/exit" {
                                            return Ok(());
                                        }
                                        app.send_command();
                                    }
                                }
                                KeyCode::Backspace => {
                                    app.input.pop();
                                    if app.cursor_position > 0 {
                                        app.cursor_position -= 1;
                                    }
                                }
                                KeyCode::Left => {
                                    if app.cursor_position > 0 {
                                        app.cursor_position -= 1;
                                    }
                                }
                                KeyCode::Right => {
                                    if app.cursor_position < app.input.len() {
                                        app.cursor_position += 1;
                                    }
                                }
                                KeyCode::Char(c) => {
                                    app.input.push(c);
                                    app.cursor_position += 1;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Event::Mouse(mouse_event) => match mouse_event.kind {
                    MouseEventKind::ScrollUp => {
                        if app.scroll_offset > 0 {
                            app.scroll_offset -= 1;
                        }
                    }
                    MouseEventKind::ScrollDown => {
                        let max_scroll = app.messages.len().saturating_sub(1) as u16;
                        if app.scroll_offset < max_scroll {
                            app.scroll_offset += 1;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        app.process_messages();
    }
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
    messages: Vec<Message>,
    agent: Agent,
    cursor_position: usize,
    thinking: bool,
    scroll_offset: u16,
    show_menu: bool,
    menu_selection: usize,
}

impl App {
    fn new() -> Self {
        let agent = Agent::new();
        Self {
            input: String::new(),
            messages: vec![],
            agent,
            cursor_position: 0,
            thinking: false,
            scroll_offset: 0,
            show_menu: false,
            menu_selection: 0,
        }
    }

    fn send_command(&mut self) {
        let command = self.input.clone();
        self.messages.push(Message::User(command.clone()));
        self.input.clear();
        self.cursor_position = 0;
        self.thinking = true;

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
        if has_new {
            self.scroll_offset = self.messages.len().saturating_sub(1) as u16;
        }
    }
}
