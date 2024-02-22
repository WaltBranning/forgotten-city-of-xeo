use ratatui::{
    prelude::*,
    widgets::{
        Block,
        Borders,
        BorderType,
    }
};

pub fn ui(frame: &mut Frame) {

    frame.render_widget(
        Block::new()
            .title("The Forgotten City Of Xeo")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded),
        frame.size());

    let header = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(5), Constraint::Percentage(100)])
        .split(frame.size());

    let main_window = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(0)])
        .split(header[1]);
    
    // render_window(frame, header[0], "Game header");
    // render_window(frame, main_window[0], "Main");
}

fn render_window(frame: &mut Frame, layout:Rect, title: &str){
    frame.render_widget(
        Block::new().borders(Borders::ALL).title(title),
        layout);
}