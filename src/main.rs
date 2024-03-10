use ratatui::widgets::ListState;
mod chatbuild;
mod ui;
mod tui;
mod app;
mod game;
use crate::app::*;
use crate::ui::ui;

#[tokio::main]
async fn main() -> Result<()> {
    let result = run().await;
    Ok(())
}

async fn run() -> Result<()> {
    let mut tui = tui::Tui::new()?
        .tick_rate(4.0)
        .frame_rate(80.0);
    
    tui.enter()?;
    // println!("Checking After Enter");
    let default_control_state = ControlState{ state:ListState::default(), commands:None, current_selected:None};

    let mut app = App { 
        should_quit: false, 
        text: Some(String::new()), 
        text_buffer: None, 
        current_location: None, 
        load_location: format!("introduction").into(), 
        location_label: None,
        commands:None,
        control_state: default_control_state,
    };
    // app.set_load_location(format!("token_input"));
    // println!("{:?}", app);
    loop {
        tui.draw(|frame| ui(app.clone(), frame))?;
        
        if let Some(text) = app.text_buffer.as_mut()  {
            let new_val = text.pop_back();
            if new_val.is_some() {
                let old_val = app.text.clone().unwrap_or("".to_string());
                let new_text = format!("{}{}",old_val,  &new_val.unwrap_or(' ').to_string());
                app.text.replace(new_text);
            }
        }
        if let Some(event) = tui.next().await {
            app.update(event).await?;
        };
                
        if app.should_quit {
            break;
        };

    }
    tui.exit()?;
    Ok(())
}