use notify_rust::Notification;

use std::time::{Duration, Instant};
use std::{sync::mpsc, thread};

use color_eyre::eyre::{Ok, Result};
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::style::Modifier;
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyEvent},
    layout::Rect,
    style::{Color, Style},
    widgets::{Paragraph, Widget},
    *,
};

use clap::Parser;
use tui_widgets::big_text::{BigText, PixelSize};

#[derive(Parser, Debug)]
#[command(version="0.0.1", author="replicaCortex",about ="Simple timer", long_about = None)]
struct Cli {
    /// Set the timer in minutes
    #[arg(short = 'd', long, default_value_t = 5)]
    duration: u16,

    /// Notification Title
    #[arg(short = 's', long, default_value_t = String::from(""))]
    summary: String,

    /// Main text of the notification
    #[arg(short = 'b', long, default_value_t = String::from(""))]
    body: String,
}

#[derive(Default, PartialEq, Eq)]
enum TimerState {
    Stop,
    #[default]
    Running,
}

#[derive(Default, Debug, PartialEq)]
enum AppState {
    #[default]
    Normal,
    Quit,
}

#[derive(Default, Debug)]
enum Mode {
    #[default]
    Timer,
    Alarm,
}

#[derive(Default)]
struct App {
    current_time: chrono::TimeDelta,
    startet_time: chrono::TimeDelta,
    timer_state: TimerState,

    app_state: AppState,

    mode: Mode,

    send: bool,
    notification_enable: bool,
    send_notification: bool,

    summary: String,
    body: String,
}

enum AppEvent {
    Key(KeyEvent),
    Tick,
}

fn main() -> Result<()> {
    let (app, terminal) = init_app_and_terminal();
    let rc = init_thread();

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
                AppEvent::Tick => self.update_timer(),
            }
        }

        Ok(())
    }

    fn is_running(&self) -> bool {
        self.app_state == AppState::Normal
    }

    fn update_timer(&mut self) {
        if self.timer_state != TimerState::Stop {
            if self.current_time - chrono::Duration::seconds(1) >= chrono::Duration::zero() {
                self.current_time -= chrono::Duration::seconds(1);
            } else {
                self.notification();
            }
        }
    }
}

impl App {
    fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            event::KeyCode::Char(' ') => {
                if self.current_time == chrono::Duration::zero() {
                    self.current_time = self.startet_time;
                } else if self.timer_state == TimerState::Running {
                    self.timer_state = TimerState::Stop;
                } else {
                    self.timer_state = TimerState::Running;
                }
            }

            event::KeyCode::Char('r') => self.reset(),
            event::KeyCode::Char('к') => self.reset(),

            event::KeyCode::Char('q') => self.app_state = AppState::Quit,
            event::KeyCode::Char('й') => self.app_state = AppState::Quit,
            event::KeyCode::Esc => self.app_state = AppState::Quit,

            _ => (),
        }
    }

    fn reset(&mut self) {
        self.current_time = self.startet_time;

        if self.notification_enable {
            self.send = true;
        }
    }

    fn notification(&mut self) {
        if self.send {
            if self.send_notification {
                Notification::new()
                    .summary(&self.summary)
                    .body(&self.body)
                    .timeout(notify_rust::Timeout::Milliseconds(10000))
                    .show()
                    .unwrap();
            }

            self.send = false;
        }
    }
}

impl Widget for &App {
    fn render(self, area: prelude::Rect, buf: &mut prelude::Buffer)
    where
        Self: Sized,
    {
        let vertical_chunks = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(8),
            Constraint::Fill(1),
        ])
        .split(area);

        match self.mode {
            Mode::Timer => self.render_timer(vertical_chunks[1], buf),
            Mode::Alarm => self.render_alarm(area, buf),
        }
    }
}

impl App {
    fn render_timer(&self, area: Rect, buf: &mut Buffer) {
        let total_seconds = self.current_time.num_seconds();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        let style = {
            if self.current_time != chrono::Duration::zero() {
                Style {
                    fg: match self.timer_state {
                        TimerState::Stop => Some(Color::Gray),
                        TimerState::Running => Some(Color::LightYellow),
                    },
                    ..Default::default()
                }
            } else {
                Style {
                    fg: match self.timer_state {
                        TimerState::Stop => Some(Color::Gray),
                        TimerState::Running => Some(Color::LightYellow),
                    },
                    ..Default::default()
                }
                .add_modifier(Modifier::SLOW_BLINK)
            }
        };
        let lines = {
            if hours > 0 {
                format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
            } else if minutes > 0 {
                format!("{:02}:{:02}", minutes, seconds)
            } else {
                format!("{:02}", seconds)
            }
        };

        let big_text = BigText::builder()
            .pixel_size(PixelSize::Full)
            .style(style)
            .lines(vec![lines.into()])
            .alignment(Alignment::Center)
            .build();

        big_text.render(area, buf);
    }

    fn render_alarm(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Alarm").render(area, buf);
    }
}

fn init_app_and_terminal() -> (App, DefaultTerminal) {
    color_eyre::install().unwrap();

    let cli = Cli::parse();

    let terminal = ratatui::init();
    let mut app = App {
        current_time: chrono::Duration::seconds((cli.duration * 60) as i64),
        notification_enable: { !cli.summary.is_empty() || !cli.body.is_empty() },
        send: { !cli.summary.is_empty() || !cli.body.is_empty() },

        send_notification: { !cli.summary.is_empty() || !cli.body.is_empty() },
        summary: cli.summary,
        body: cli.body,

        ..Default::default()
    };

    app.startet_time = app.current_time;

    (app, terminal)
}

fn init_thread() -> mpsc::Receiver<AppEvent> {
    let (tx, rc) = mpsc::channel::<AppEvent>();
    let key_tx = tx.clone();
    thread::spawn(move || {
        loop {
            if let Event::Key(key) = event::read().unwrap()
                && key_tx.send(AppEvent::Key(key)).is_err()
            {
                break;
            }
        }
    });

    let tick_tx = tx;
    thread::spawn(move || {
        let tick_rate = Duration::from_secs(1);
        let mut last_tick = Instant::now();

        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            thread::sleep(timeout);

            last_tick = Instant::now();

            if tick_tx.send(AppEvent::Tick).is_err() {
                break;
            }
        }
    });

    rc
}
