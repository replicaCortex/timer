use std::{sync::mpsc, thread};

use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyEvent},
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Tabs, Widget},
    *,
};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

#[derive(Default, PartialEq, Eq)]
enum TimerState {
    #[default]
    Stop,
    Running,
}

#[derive(Display, Default, Debug, EnumIter, FromRepr, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    #[strum(to_string = " NORMAL ")]
    Normal,
    #[strum(to_string = " INSERT ")]
    Insert,
    Quit,
}

#[derive(Display, Default, Debug, EnumIter, FromRepr, Clone, Copy)]
enum Mode {
    #[default]
    Timer,
    Alarm,
}

#[derive(Default)]
struct App {
    current_time: chrono::TimeDelta,
    timer_state: TimerState,

    app_state: AppState,

    mode: Mode,
}

enum AppEvent {
    Key(KeyEvent),
    Pause,
    Unpause,
    Tick,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let app = App {
        current_time: chrono::Duration::seconds(10),
        ..Default::default()
    };

    let (key_tx, rc) = mpsc::channel::<AppEvent>();

    thread::spawn(move || {
        loop {
            if let Event::Key(key) = event::read().unwrap()
                && key_tx.send(AppEvent::Key(key)).is_err()
            {
                break;
            }
        }
    });

    let terminal = ratatui::init();
    app.run(terminal, rc)?;

    ratatui::restore();
    Ok(())
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal, rc: mpsc::Receiver<AppEvent>) -> Result<()> {
        while self.is_running() {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;

            match rc.recv()? {
                AppEvent::Key(key) => self.handle_key_event(key),
                AppEvent::Tick => self.timer_update(),
                _ => (),
            }
        }

        Ok(())
    }

    fn is_running(&self) -> bool {
        self.app_state == AppState::Normal || self.app_state == AppState::Insert
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            event::KeyCode::Char(' ') => {
                if self.timer_state == TimerState::Running {
                    self.timer_state = TimerState::Stop;
                } else {
                    self.timer_state = TimerState::Running;
                }
            }

            event::KeyCode::Char('i') => self.app_state = AppState::Insert,
            event::KeyCode::Char('ш') => self.app_state = AppState::Insert,

            event::KeyCode::Char('q') => self.quit(),
            event::KeyCode::Char('й') => self.quit(),
            event::KeyCode::Esc => self.quit(),

            event::KeyCode::Char('k') => {
                self.mode = self.mode.previous();
            }
            event::KeyCode::Char('j') => {
                self.mode = self.mode.next();
            }
            _ => (),
        }
    }

    fn timer_update(&mut self) {
        if self.current_time > chrono::Duration::zero() {
            self.current_time -= chrono::Duration::seconds(1);
        } else {
            self.timer_state = TimerState::Stop;
        }
    }

    fn quit(&mut self) {
        if self.app_state == AppState::Insert {
            self.app_state = AppState::Normal;
        } else {
            self.app_state = AppState::Quit;
        }
    }
}

impl Widget for &App {
    fn render(self, area: prelude::Rect, buf: &mut prelude::Buffer)
    where
        Self: Sized,
    {
        let [top_area, bottom_area] =
            Layout::horizontal([Constraint::Percentage(20), Constraint::Fill(1)]).areas(area);

        self.render_title(top_area, buf);
        self.render_mode(bottom_area, buf);
        self.app_state.render(area, buf);
    }
}

impl App {
    fn render_title(&self, area: Rect, buf: &mut Buffer) {
        let titles = Mode::iter().map(|mode| mode.to_string());
        let tip = Block::new().title("Use j k to change mode ");
        let selected_index = self.mode as usize;

        Tabs::new(titles)
            .block(tip)
            .select(selected_index)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }

    fn render_mode(&self, area: Rect, buf: &mut Buffer) {
        match self.mode {
            Mode::Timer => self.render_timer(area, buf),
            Mode::Alarm => self.render_alarm(area, buf),
        }
    }

    fn render_timer(&self, area: Rect, buf: &mut Buffer) {
        let total_seconds = self.current_time.num_seconds();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        let style = Style {
            fg: match self.timer_state {
                TimerState::Stop => Some(Color::Gray),
                TimerState::Running => Some(Color::LightGreen),
            },
            ..Default::default()
        };
        let span = Span::styled(format!("{:02}:{:02}:{:02}", hours, minutes, seconds), style);

        Line::from(span).render(area, buf);
    }

    fn render_alarm(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Alarm").render(area, buf);
    }
}

impl Mode {
    fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }

    fn previous(self) -> Self {
        let current_index = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }
}

impl Widget for &AppState {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = self.to_string();
        let style = Style {
            fg: Some(Color::Black),
            bg: match self {
                AppState::Normal => Some(Color::LightBlue),
                AppState::Insert => Some(Color::Magenta),
                _ => Some(Color::Black),
            },
            add_modifier: Modifier::BOLD,
            ..Default::default()
        };
        let span = Span::styled(&title, style);

        let area = Rect::new(0, area.height - 1, title.len() as u16, 2);

        Line::from(span).render(area, buf);
    }
}
