use ratatui::{
    prelude::*,
    widgets::{
        Block,
        Borders,
        BorderType,
    }
};

pub fn ui(frame: &mut Frame) {;

    let main_window_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100), Constraint::Percentage(15)])
        .split(frame.size());

    // let main_window = Layout::default()
    //     .direction(Direction::Vertical)
    //     .constraints([Constraint::Percentage(90), Constraint::Percentage(0)])
    //     .split(controls[1]);

    frame.render_widget(
        Block::new()
            .title(" The Forgotten City Of Xeo ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded),
        main_window_split[0]);

    frame.render_widget(
            Block::new()
                .title(" Controls ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Yellow))
                .border_type(BorderType::Rounded),
            main_window_split[1]);
    
    // render_window(frame, header[0], "Game header");
    // render_window(frame, main_window[0], "Main");
}

fn render_window(frame: &mut Frame, layout:Rect, title: &str){
    frame.render_widget(
        Block::new().borders(Borders::ALL).title(title),
        layout);
}