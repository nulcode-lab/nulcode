use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::{App, Message};

pub fn draw(frame: &mut Frame, app: &App) {
    if app.messages.is_empty() {
        draw_splash(frame);
    } else {
        draw_main(frame, app);
    }

    let input_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
        .split(frame.area());

    let input = create_input_widget(app);
    frame.render_widget(input, input_chunks[1]);
    frame.set_cursor_position((
        input_chunks[1].x + app.cursor_position as u16 + 1,
        input_chunks[1].y + 1,
    ));
}

fn draw_splash(frame: &mut Frame) {
    let area = frame.area();

    let splash_width = 76;
    let splash_height = 13;
    let x = (area.width - splash_width) / 2;
    let y = (area.height - splash_height) / 2;

    let splash_area = ratatui::layout::Rect {
        x,
        y,
        width: splash_width,
        height: splash_height,
    };

    let colors: Vec<Color> = vec![
        Color::Rgb(255, 80, 80),
        Color::Rgb(255, 140, 0),
        Color::Rgb(255, 220, 0),
        Color::Rgb(80, 220, 100),
        Color::Rgb(80, 160, 255),
        Color::Rgb(140, 100, 255),
        Color::Rgb(200, 100, 255),
    ];

    let logo_patterns: [[&str; 7]; 8] = [
        [
            "██    ██    ",
            "██   █    ",
            "██        ",
            "██████    ",
            "██████    ",
            "█████      ",
            "██████    ",
        ],
        [
            "███   ██    ",
            "██   █    ",
            "██        ",
            "██████    ",
            "██████    ",
            "██  ██     ",
            "██████    ",
        ],
        [
            "████  ██    ",
            "██   █    ",
            "██        ",
            "██        ",
            "██   █    ",
            "██   ██    ",
            "██        ",
        ],
        [
            "██ ██ ██    ",
            "██   █    ",
            "██        ",
            "██        ",
            "██   █    ",
            "██    ██   ",
            "██        ",
        ],
        [
            "██  ████    ",
            "██   █    ",
            "██        ",
            "██        ",
            "██   █    ",
            "██    ██   ",
            "████      ",
        ],
        [
            "██   ███    ",
            "██   █    ",
            "██        ",
            "██        ",
            "██   █    ",
            "██   ██    ",
            "██        ",
        ],
        [
            "██    ██    ",
            "██████    ",
            "██████    ",
            "██████    ",
            "██████    ",
            "██  ██     ",
            "██████    ",
        ],
        [
            "██    ██    ",
            "██████    ",
            "██████    ",
            "██████    ",
            "██████    ",
            "█████      ",
            "██████    ",
        ],
    ];

    let logo_text: Vec<Line> = logo_patterns
        .iter()
        .map(|row| {
            let spans: Vec<Span> = row
                .iter()
                .enumerate()
                .flat_map(|(letter_idx, pattern)| {
                    let letter_spans: Vec<Span> = pattern
                        .chars()
                        .map(|c| {
                            if c == '█' {
                                Span::styled(
                                    "█",
                                    Style::default()
                                        .fg(colors[letter_idx])
                                        .bg(colors[letter_idx]),
                                )
                            } else {
                                Span::raw(" ")
                            }
                        })
                        .collect();
                    letter_spans
                })
                .collect();
            Line::from(spans)
        })
        .chain(std::iter::once(Line::from("")))
        .chain(std::iter::once(Line::from(vec![Span::styled(
            "  AI Code Assistant",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )])))
        .chain(std::iter::once(Line::from(vec![Span::styled(
            "  Type a message and press Enter to start",
            Style::default().fg(Color::DarkGray),
        )])))
        .collect();

    let splash = Paragraph::new(Text::from(logo_text))
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);

    frame.render_widget(splash, splash_area);
}

fn draw_main(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(frame.area());

    let title = Paragraph::new(Span::styled(
        "NULCODE",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))
    .block(Block::default().borders(Borders::ALL))
    .alignment(Alignment::Center);
    frame.render_widget(title, chunks[0]);

    let messages: Vec<Line> = app
        .messages
        .iter()
        .flat_map(|msg| create_message_lines(msg))
        .collect();

    let messages_text = Text::from(messages);
    let messages_widget = Paragraph::new(messages_text)
        .block(Block::default().borders(Borders::ALL).title("Messages"))
        .wrap(Wrap { trim: true })
        .scroll((app.scroll_offset, 0));
    frame.render_widget(messages_widget, chunks[1]);
}

fn create_message_lines(msg: &Message) -> Vec<Line<'_>> {
    let (prefix, prefix_style) = match msg {
        Message::User(_) => (
            "You: ",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Message::Agent(_) => (
            "Agent: ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Message::System(_) => (
            "System: ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Message::Error(_) => (
            "Error: ",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
    };

    let text = match msg {
        Message::User(t) | Message::Agent(t) | Message::System(t) | Message::Error(t) => t.clone(),
    };

    let mut lines = Vec::new();
    let full_text = format!("{}{}", prefix, text);

    for (i, line) in full_text.split('\n').enumerate() {
        if i > 0 {
            lines.push(Line::from(""));
        }
        lines.push(Line::from(Span::styled(line.to_string(), prefix_style)));
    }

    lines
}

fn create_input_widget(app: &App) -> Paragraph<'_> {
    let title = "Input - Press Enter to send, Esc to quit";

    let thinking_indicator = if app.thinking { " [Thinking...]" } else { "" };

    Paragraph::new(app.input.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("{}{}", title, thinking_indicator)),
        )
        .style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .wrap(Wrap { trim: true })
}
