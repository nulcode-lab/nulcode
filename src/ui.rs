use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::{App, InputMode, Message};

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
        "NULLCODE",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))
    .block(Block::default().borders(Borders::ALL))
    .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(title, chunks[0]);

    // Messages
    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .map(|msg| create_message_item(msg))
        .collect();

    let messages_list = List::new(messages)
        .block(Block::default().borders(Borders::ALL).title("Messages"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(messages_list, chunks[1]);

    // Input
    let input = create_input_widget(app);
    frame.render_widget(input, chunks[2]);

    // Set cursor position
    if app.input_mode == InputMode::Editing {
        frame.set_cursor_position((
            chunks[2].x + app.cursor_position as u16 + 1,
            chunks[2].y + 1,
        ));
    }
}

fn create_message_item(msg: &Message) -> ListItem {
    match msg {
        Message::User(text) => ListItem::new(Line::from(vec![
            Span::styled("You: ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(text),
        ])),
        Message::Agent(text) => ListItem::new(Line::from(vec![
            Span::styled("Agent: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(text),
        ])),
        Message::System(text) => ListItem::new(Line::from(vec![
            Span::styled("System: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(text),
        ])),
        Message::Error(text) => ListItem::new(Line::from(vec![
            Span::styled("Error: ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(text),
        ])),
    }
}

fn create_input_widget(app: &App) -> Paragraph {
    let input_style = match app.input_mode {
        InputMode::Normal => Style::default().fg(Color::Gray),
        InputMode::Editing => Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
    };

    let mode_indicator = match app.input_mode {
        InputMode::Normal => "NORMAL",
        InputMode::Editing => "EDIT",
    };

    let title = format!("Input [{}] - Press 'e' to edit", mode_indicator);

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
        .style(input_style)
        .wrap(Wrap { trim: true })
}
