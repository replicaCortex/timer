use chrono::Timelike;
use ratatui::style::{Modifier, Stylize};
use ratatui::widgets::Paragraph;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

use tui_widgets::big_text::{BigText, PixelSize};

use ratatui::layout::Alignment;

use super::{App, TimerState};

pub fn render_timer(app: &App, area: Rect, buf: &mut Buffer) {
    let (hours, minutes, seconds) = get_time(app);
    let style = get_style(app);

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

pub fn render_alarm(
    app: &App,
    area_for_big_text: Rect,
    area_for_small_text: Rect,
    buf: &mut Buffer,
) {
    let (hours, minutes, seconds) = get_time(app);
    let style = get_style(app);

    let lines = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);

    let big_text = BigText::builder()
        .pixel_size(PixelSize::Full)
        .style(style)
        .lines(vec![lines.into()])
        .alignment(Alignment::Center)
        .build();

    big_text.render(area_for_big_text, buf);

    let alarm_time = format!(
        "{:02}:{:02}:{:02}",
        app.alarm_time.hour(),
        app.alarm_time.minute(),
        app.alarm_time.second()
    );
    Paragraph::new(String::from("Alarm set for ") + &alarm_time)
        .alignment(Alignment::Center)
        .style(style)
        .bold()
        .render(area_for_small_text, buf);
}

fn get_time(app: &App) -> (i64, i64, i64) {
    let total_seconds = app.current_time.num_seconds();

    (
        total_seconds / 3600,
        (total_seconds % 3600) / 60,
        total_seconds % 60,
    )
}

fn get_style(app: &App) -> ratatui::prelude::Style {
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
}
