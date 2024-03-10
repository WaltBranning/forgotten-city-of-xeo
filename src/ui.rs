use ratatui::{
    prelude::*,
    widgets::{
        Block, BorderType, Borders, List, ListDirection, Padding, Paragraph, Wrap
    }
};

use crate::app::App;

pub fn ui(mut app: App, frame: &mut Frame) {
    main_game_window(app, frame);
}

fn main_game_window(mut app: App, frame: &mut Frame) {
        // println!("{:?}", app.control_state.state.selected());
        let main_window_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100), Constraint::Percentage(25)])
        .split(frame.size());

    // let token_loader = "token_loader".to_string();
    // if let token_loader = &app.current_location {}

    let control_output = |color:Color| -> Block<'_> { Block::new()
        .title(" Controls ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .style(Style::default().fg(color))
        .border_type(BorderType::Rounded)
        .padding(Padding{left:3, right:1, top:1, bottom: 1})};

    let mut command_list: Vec<Span> = Vec::new();
    if let Some(commands) = app.commands.as_ref() {
        let mut command_count: usize = 0;
        command_list = commands.into_iter().map(
            |command| {
                let mut command_color = Color::Yellow;
                // let mut bg_color = Color::Black;
                if command_count == app.control_state.state.selected().unwrap_or(0) {
                    command_color = Color::LightYellow;
                }
                command_count += 1;
                Span::styled(command.label.clone(),
            Style::default().fg(command_color))
            }
        ).collect();
    }

    let control_panel = List::new(command_list)
        .block(control_output(Color::Yellow))
        .style(Style::default().fg(Color::Yellow))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol("--> ")
        .direction(ListDirection::TopToBottom);

    let text_block = Paragraph::new(app.text.clone().unwrap())
        .wrap(Wrap {trim:true});

    fn render_main_text_area(paragraph: &Paragraph, frame: &mut Frame, area: Rect, app: &App) {
        let main_window = Block::new()
            .title(" The Forgotten City Of Xeo ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded)
            .padding(Padding::uniform(1));
    
        let main_output = Block::new()
            .title(app.location_label.clone().unwrap_or("".to_string()))
            .border_type(BorderType::Double)
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .padding(Padding::uniform(1));

        let inner = main_window.inner(area);
        frame.render_widget(main_window, area);
        frame.render_widget(paragraph.clone().block(main_output), inner);
    }

    render_main_text_area(&text_block, frame, main_window_split[0], &app);

    frame.render_stateful_widget(
        control_panel,
        main_window_split[1],
        &mut app.control_state.state);
}