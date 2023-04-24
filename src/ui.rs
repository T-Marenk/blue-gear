use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::Spans,
    widgets::{Block, Borders, Row, Table, Paragraph},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let constraints: Vec<Constraint> = vec![Constraint::Length(3), Constraint::Min(0)];
    let parts = Layout::default()
        .constraints(constraints.as_ref())
        .split(f.size());

    let status = match app.status {
        true => "On",
        false => "Off",
    };

    draw_state_block(f, status, parts[0], app);

    draw_device_block(f, app, parts[1]);
}

fn draw_state_block<B: Backend>(f: &mut Frame<B>, status: &str, part: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Blue-gear");
    f.render_widget(block, part);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(vec![Constraint::Length(18), Constraint::Percentage(10)].as_ref())
        .split(part);
    
    let toggle_text = Spans::from("Bluetooth status");
    let toggle_color = app.toggle_selected();
    let toggle_paragraph = Paragraph::new(toggle_text.clone())
        .style(Style::default().fg(toggle_color));
    f.render_widget(toggle_paragraph, chunks[0]);
    
    let status_color = app.status_color(status);
    let status_paragraph = Paragraph::new(status)
        .style(Style::default().fg(status_color))
        .alignment(tui::layout::Alignment::Left);
    f.render_widget(status_paragraph, chunks[1])
}

fn draw_device_block<B: Backend>(f: &mut Frame<B>, app: &App, part: Rect) {
    let rows = app.informations
        .iter()
        .map(|d| {
            Row::new(vec![d.to_owned()])
        });

    let table = Table::new(rows)
        .block(Block::default().title("Devices").borders(Borders::ALL))
        .widths(&[Constraint::Length(200)]);
    f.render_widget(table, part);}
