use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{prelude::*, widgets::Paragraph};
use std::time::Duration;

#[derive(Debug, Default)]
pub struct App {
    counter: i32,
    app_state: AppState,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Done,
}

impl App {
    pub fn run(
        &mut self,
        terminal: &mut Terminal<impl Backend>,
    ) -> color_eyre::Result<()> {
        while self.app_state != AppState::Done {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.update()?;
        }
        Ok(())
    }

    fn update(&mut self) -> color_eyre::Result<()> {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(keyevent) => self.handle_key_event(keyevent),
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, keyevent: KeyEvent) {
        if keyevent.kind == KeyEventKind::Press {
            match keyevent.code {
                KeyCode::Char('q') => self.app_state = AppState::Done,
                KeyCode::Char('j') | KeyCode::Down => self.counter -= 1,
                KeyCode::Char('k') | KeyCode::Up => self.counter += 1,
                _ => {}
            }
        }
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(
            Paragraph::new(format!("Counter: {}", self.counter)),
            frame.size(),
        );
    }
}

#[cfg(test)]
mod tests {
    use ratatui::backend::TestBackend;

    use super::*;

    #[test]
    fn handle_key_event() {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('j').into());
        assert_eq!(app.counter, -1, "j");

        let mut app = App::default();
        app.handle_key_event(KeyCode::Down.into());
        assert_eq!(app.counter, -1, "Down");

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('k').into());
        assert_eq!(app.counter, 1, "k");

        let mut app = App::default();
        app.handle_key_event(KeyCode::Up.into());
        assert_eq!(app.counter, 1, "Up");

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert_eq!(app.app_state, AppState::Done);
    }

    #[test]
    fn render_frame() {
        let app = App::default();
        let backend = TestBackend::new(30, 3);
        let mut terminal = Terminal::new(backend).expect("terminal");

        let completed_frame = terminal
            .draw(|frame| app.render_frame(frame))
            .expect("draw");

        assert_eq!(
            *completed_frame.buffer,
            Buffer::with_lines(vec![
                "Counter: 0                    ",
                "                              ",
                "                              ",
            ])
        );
    }
}
