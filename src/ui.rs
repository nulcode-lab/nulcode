use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::{App, Message};

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),  // Title
                Constraint::Min(1),     // Messages
                Constraint::Length(3),  // Input
            ]
            .as_ref(),
        )
        .split(frame.area());

    // Title
    let title = Paragraph::new(Span::styled(
        "NULCODE",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))
    .block(Block::default().borders(Borders::ALL))
    .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(title, chunks[0]);

    // Messages
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

    // Input
    let input = create_input_widget(app);
    frame.render_widget(input, chunks[2]);

    // Set cursor position (always in input mode)
    frame.set_cursor_position((
        chunks[2].x + app.cursor_position as u16 + 1,
        chunks[2].y + 1,
    ));
}

fn create_message_lines(msg: &Message) -> Vec<Line> {
    let (prefix, prefix_style) = match msg {
        Message::User(_) => ("You: ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Message::Agent(_) => ("Agent: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Message::System(_) => ("System: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Message::Error(_) => ("Error: ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
    };

    let text = match msg {
        Message::User(t) | Message::Agent(t) | Message::System(t) | Message::Error(t) => t.clone(),
    };

    let mut lines = Vec::new();
    let full_text = format!("{}{}", prefix, text);

    // Split by newlines first
    for (i, line) in full_text.split('\n').enumerate() {
        if i > 0 {
            lines.push(Line::from(""));
        }
        lines.push(Line::from(Span::styled(line.to_string(), prefix_style)));
    }

    lines
}

fn create_input_widget(app: &App) -> Paragraph {
    let title = "Input - Press Enter to send, Esc to quit";

    let thinking_indicator = if app.thinking {
        " [Thinking...]"
    } else {
        ""
    };

    Paragraph::new(app.input.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("{}{}", title, thinking_indicator)),
        )
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        .wrap(Wrap { trim: true })
}
