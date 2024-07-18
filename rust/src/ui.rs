use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Line},
        Block, BorderType, Borders, List, ListItem, Padding, Paragraph,
    },
    Frame,
};

use crate::app::{App, CurrentlyEditing};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let area = frame.size();
    let [title_area, main_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .areas(area);

    let constraints = if app.show_contestants {
        vec![Constraint::Ratio(1, 4), Constraint::Min(0)]
    } else {
        vec![Constraint::Min(0)]
    };
    let left_and_main = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(main_area);

    let (main_area, contestant_area) = if left_and_main.len() == 1 {
        (left_and_main[0], None)
    } else {
        (left_and_main[1], Some(left_and_main[0]))
    };

    let [wheel_area, sub_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Ratio(1, 4)])
        .areas(main_area);

    let [left_area, middle_area, right_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(Constraint::from_ratios([(1, 3), (1, 3), (1, 3)]))
        .areas(sub_area);

    render_title(frame, title_area);
    render_wheel(app, frame, wheel_area);
    render_status(app, frame, left_area);
    render_help(frame, right_area);
    if let Some(contestant_area) = contestant_area {
        render_names(app, frame, contestant_area);
    }
    render_winners(app, frame, middle_area);

    if let Some(editing) = &app.currently_editing {
        match editing {
            CurrentlyEditing::Name => {
                render_editing_name(frame, app);
            }
        }
    }
}

fn render_editing_name(frame: &mut Frame, app: &mut App) {
    let popup_block = Block::new().title("Enter a new name").bg(Color::DarkGray);
    let area = centerd_rect(15, 10, frame.size());
    frame.render_widget(popup_block, area);

    let popup_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .margin(1)
        .split(area);

    let name_block = Block::default().title("Name").borders(Borders::ALL);

    let name_text = Paragraph::new(app.name_input.clone()).block(name_block);
    frame.render_widget(name_text, popup_chunks[0]);
}

fn render_title(frame: &mut Frame, area: Rect) {
    let title = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let title = Paragraph::new("Wheel of Names")
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(title);
    frame.render_widget(title, area);
}

fn render_wheel(app: &mut App, frame: &mut Frame, area: Rect) {
    let wheel = Block::default()
        .title("Wheel")
        .title_alignment(Alignment::Center);

    let middle = (0.0, 0.0);
    let radius = 10.0;

    let canvas = Canvas::default()
        .block(wheel)
        .marker(Marker::HalfBlock)
        .paint(|ctx| {
            let lines = app.all_participants.items.len() * 2;
            for i in 0..lines {
                let dangle = 360.0 * i as f64 / lines as f64;
                if i % 2 == 1 {
                    let dx = radius / 1.25 * (dangle + app.wheel.angle).to_radians().sin();
                    let dy = radius / 1.25 * (dangle + app.wheel.angle).to_radians().cos();
                    ctx.print(
                        middle.0 + dx,
                        middle.1 + dy,
                        app.all_participants.items[app.all_participants.items.len() - 1 - (i / 2)] // reverse the list
                            .clone()
                            .yellow(),
                    );
                } else {
                    let dx = radius * (dangle + app.wheel.angle).to_radians().sin();
                    let dy = radius * (dangle + app.wheel.angle).to_radians().cos();
                    ctx.draw(&Line::new(
                        middle.0,
                        middle.1,
                        middle.0 + dx,
                        middle.1 + dy,
                        match i % 2 {
                            0 => Color::Yellow,
                            1 => Color::Green,
                            2 => Color::Blue,
                            3 => Color::Red,
                            _ => Color::White,
                        },
                    ));
                }
                ctx.draw(&Line::new(0.0, 10.0, 0.0, 9.0, Color::Green));
            }
        })
        .x_bounds([-10.0 * 1.55, 10.0 * 1.55])
        .y_bounds([-10.0, 10.0]);

    frame.render_widget(canvas, area);
}

fn render_names(app: &mut App, frame: &mut Frame, area: Rect) {
    let names = Block::default()
        .borders(Borders::ALL)
        .padding(Padding::new(1, 1, 0, 0))
        .title("Contestants")
        .title_alignment(Alignment::Left)
        .border_type(BorderType::Rounded);

    let names_items: Vec<ListItem> = app
        .all_participants
        .items
        .iter()
        .map(|name| ListItem::new(name.clone()).style(Style::default().fg(Color::Yellow)))
        .collect();
    let names = List::new(names_items)
        .block(names)
        .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black));
    frame.render_stateful_widget(names, area, &mut app.all_participants.state);
}

fn render_help(frame: &mut Frame, area: Rect) {
    let help = Block::default()
        .title("Help")
        .title_alignment(Alignment::Left)
        .padding(Padding::new(1, 1, 0, 0))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let help = Paragraph::new(vec![
        "Esc/q - Quit".into(),
        "Enter/s - Start spin".into(),
        "Backspace/r - Reset spin".into(),
        "j/down - Select next".into(),
        "k/up - Select previous".into(),
        "Del/x - Remove selected".into(),
        "Tab - Show/hide contestants".into(),
    ])
    .block(help);
    frame.render_widget(help, area);
}

fn render_status(app: &mut App, frame: &mut Frame, area: Rect) {
    let status = Block::default()
        .title("Status")
        .title_alignment(Alignment::Left)
        .padding(Padding::new(1, 1, 0, 0))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let text = vec![
        format!("Spins remaining: {}", app.wheel.spin_count).into(),
        format!(
            "Number of contestants: {}",
            app.all_participants.items.len()
        )
        .into(),
        format!(
            "Chance of winning: {:.1}%",
            100.0
                / (if app.all_participants.items.is_empty() {
                    1.0
                } else {
                    app.all_participants.items.len() as f64
                })
        )
        .into(),
    ];
    let status = Paragraph::new(text).block(status);
    frame.render_widget(status, area);
}

fn render_winners(app: &mut App, frame: &mut Frame, area: Rect) {
    let winners = Block::default()
        .title("Winners")
        .title_alignment(Alignment::Left)
        .padding(Padding::new(1, 1, 0, 0))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let winners_items: Vec<ListItem> = app
        .all_winners
        .iter()
        .map(|name| {
            ListItem::new(format!("🎁 {}", name.clone())).style(Style::default().fg(Color::Yellow))
        })
        .collect();
    let winners = List::new(winners_items)
        .block(winners)
        .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black));
    frame.render_widget(winners, area);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centerd_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
