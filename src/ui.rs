use ratatui::{
    prelude::*,
    widgets::{
        Block, BorderType, Borders, Paragraph, Wrap
    }
};

use crate::app::App;

pub fn ui(app: &App, frame: &mut Frame) {

    let main_window_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100), Constraint::Percentage(15)])
        .split(frame.size());

    let main_window = Block::new()
        .title(" The Forgotten City Of Xeo ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow))
        .border_type(BorderType::Rounded);

    let control_window = Block::new()
        .title(" Controls ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow))
        // .style(Style::default().bg(Color::Rgb(255, 162, 0)))
        .border_type(BorderType::Rounded);

    // let token_loader = "token_loader".to_string();
    // if let token_loader = &app.current_screen {}

    // let control_output = Block

    let main_output = Paragraph::new(app.text.clone().unwrap())
        .wrap(Wrap {trim:true})
        .block(main_window);

    frame.render_widget(
        main_output,
        main_window_split[0]);

    frame.render_widget(
            control_window,
            main_window_split[1]);
    
    // render_window(frame, header[0], "Game header");
    // render_window(frame, main_window[0], "Main");
}

fn render_window(frame: &mut Frame, layout:Rect, title: &str){
    frame.render_widget(
        Block::new().borders(Borders::ALL).title(title),
        layout);
}