use ratatui::style::Modifier;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Paragraph, Widget},
};

use tui_widgets::big_text::{BigText, PixelSize};

use ratatui::layout::Alignment;

use super::{App, TimerState, mode::Mode};

pub fn render_timer(app: &App, area: Rect, buf: &mut Buffer) {
    let total_seconds = app.current_time.num_seconds();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    let style = {
        if app.current_time != chrono::Duration::zero() {
            Style {
                fg: match app.timer_state {
                    TimerState::Stop => Some(Color::Gray),
                    TimerState::Running => Some(Color::LightYellow),
                },
                ..Default::default()
            }
        } else {
            Style {
                fg: match app.timer_state {
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

pub fn render_alarm(app: &App, area: Rect, buf: &mut Buffer) {
    Paragraph::new("Alarm").render(area, buf);
}
