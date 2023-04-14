use crate::app::App;
use std::rc::Rc;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs, Row, Table},
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

    draw_state_block(f, status, &parts);

    let rows = app.informations
        .iter()
        .map(|d| {
            Row::new(vec![d.to_owned()])
        });
    let table = Table::new(rows)
        .block(Block::default().title("Devices").borders(Borders::ALL))
        .widths(&[Constraint::Length(200)]);
    f.render_widget(table, parts[1]);
}

fn draw_state_block<B: Backend>(f: &mut Frame<B>, status: &str, parts: &Rc<[Rect]>) {
    let titles = vec![
        Spans::from("Bluetooth status"),
        Spans::from(Span::styled(status, Style::default().fg(Color::Green))),
    ];
    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Gray))
                .title("Blue-rs"),
        )
        .highlight_style(Style::default().fg(Color::Cyan))
        .select(0);
    f.render_widget(tabs, parts[0]);
}
