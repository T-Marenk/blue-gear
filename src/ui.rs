use tui::{
    backend::Backend,
    Frame,
    layout::{Layout, Constraint, Rect},
    widgets::{Block, Borders, Tabs},
    style::{Style, Color},
    text::{Span, Spans}
};

pub fn draw<B: Backend>(f: &mut Frame<B>, status: bool) {
    let constraints: Vec<Constraint> = vec![
        Constraint::Length(3),
        Constraint::Min(0)
    ];
    let parts = Layout::default()
        .constraints(constraints.as_ref())
        .split(f.size());
    
    let status = match status {
        true => "On",
        false => "Off"
    };

    draw_state_block(f, status, &parts);
}

fn draw_state_block<B: Backend>(f: &mut Frame<B>, status: &str, parts: &Vec<Rect>) {
    let titles = vec![
        Spans::from("Bluetooth status"),
        Spans::from(Span::styled(status, Style::default().fg(Color::Green)))];
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Gray)).title("Blue-rs"))
        .highlight_style(Style::default().fg(Color::Cyan))
        .select(0);
    f.render_widget(tabs, parts[0]);
}
