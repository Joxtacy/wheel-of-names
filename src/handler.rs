use crate::app::{App, AppResult, CurrentlyEditing};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.currently_editing {
        Some(CurrentlyEditing::Name) => match key_event.code {
            KeyCode::Enter => {
                app.currently_editing = None;
                app.save_name();
            }
            KeyCode::Char(value) => {
                app.name_input.push(value);
            }
            KeyCode::Backspace => {
                app.name_input.pop();
            }
            KeyCode::Esc => {
                app.currently_editing = None;
                app.name_input.clear();
            }
            // Other handlers you could add here.
            _ => {}
        },
        None => {
            match key_event.code {
                // Exit application on `ESC` or `q`
                KeyCode::Esc | KeyCode::Char('q') => {
                    app.quit();
                }
                // Exit application on `Ctrl-C`
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.quit();
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if !app.spinning {
                        app.all_participants.previous();
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if !app.spinning {
                        app.all_participants.next();
                    }
                }
                KeyCode::Enter | KeyCode::Char('s') => {
                    if !app.spinning {
                        app.start_spin();
                    }
                }
                KeyCode::Backspace | KeyCode::Char('r') => {
                    if app.spinning {
                        app.reset_spin();
                    }
                }
                KeyCode::Delete | KeyCode::Char('x') => {
                    if !app.spinning {
                        if let Some(i) = app.all_participants.state.selected() {
                            app.all_participants.items.remove(i);
                        }
                    }
                }
                KeyCode::Char('a') => {
                    if !app.spinning {
                        app.currently_editing = Some(CurrentlyEditing::Name);
                    }
                }
                KeyCode::Tab => {
                    app.toggle_contestants();
                }
                // Other handlers you could add here.
                _ => {}
            }
        }
    }
    Ok(())
}
