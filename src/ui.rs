use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::app::{App, ViewMode};

pub const PANEL_HEIGHT: u16 = 28;
pub const MIN_PANEL_WIDTH: u16 = 70;

pub fn columns_for_width(width: u16) -> usize {
    (width / MIN_PANEL_WIDTH).max(1) as usize
}

const BRIGHT_BLACK: Color = Color::Indexed(8);
const BRIGHT_WHITE: Color = Color::Indexed(15);

pub fn draw(f: &mut Frame, app: &App) {
    match app.mode {
        ViewMode::Grid => draw_grid(f, app),
        ViewMode::FullScreen => draw_fullscreen(f, app),
    }
}

fn draw_grid(f: &mut Frame, app: &App) {
    let area = f.area();

    if app.panels.is_empty() {
        return;
    }

    // Reserve 1 line for status bar
    let grid_area = Rect {
        x: area.x,
        y: area.y,
        width: area.width,
        height: area.height.saturating_sub(1),
    };

    let cols = columns_for_width(grid_area.width);
    let panel_count = app.panels.len();
    let rows = (panel_count + cols - 1) / cols;
    let total_height = rows as u16 * PANEL_HEIGHT;
    let max_scroll = total_height.saturating_sub(grid_area.height);
    let scroll = app.scroll_offset.min(max_scroll);

    let col_constraints: Vec<Constraint> = (0..cols)
        .map(|_| Constraint::Ratio(1, cols as u32))
        .collect();
    let col_rects = Layout::horizontal(col_constraints).split(grid_area);

    for row_idx in 0..rows {
        let virtual_y = row_idx as u16 * PANEL_HEIGHT;
        let screen_y = virtual_y as i32 - scroll as i32 + grid_area.y as i32;

        if screen_y + PANEL_HEIGHT as i32 <= grid_area.y as i32 {
            continue;
        }
        if screen_y >= (grid_area.y + grid_area.height) as i32 {
            break;
        }

        for col_idx in 0..cols {
            let panel_idx = row_idx * cols + col_idx;
            if panel_idx >= panel_count {
                break;
            }

            let panel = &app.panels[panel_idx];
            let grid_bottom = (grid_area.y + grid_area.height) as i32;

            let clip_top = (grid_area.y as i32 - screen_y).max(0) as u16;
            let clip_bottom = (screen_y + PANEL_HEIGHT as i32 - grid_bottom).max(0) as u16;
            let visible_height = PANEL_HEIGHT.saturating_sub(clip_top + clip_bottom);
            if visible_height == 0 {
                continue;
            }

            let rect = Rect {
                x: col_rects[col_idx].x,
                y: screen_y.max(grid_area.y as i32) as u16,
                width: col_rects[col_idx].width,
                height: visible_height,
            };

            let border_style = Style::default().fg(panel.border_color);
            let title_style = Style::default()
                .fg(BRIGHT_WHITE)
                .add_modifier(Modifier::BOLD);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(Line::from(vec![
                    Span::styled(format!(" {} ", panel.icon), border_style),
                    Span::styled(format!("{} ", panel.title), title_style),
                ]));

            let content_scroll = clip_top.saturating_sub(1);

            let paragraph = Paragraph::new(panel.content.clone())
                .block(block)
                .scroll((content_scroll, 0));

            f.render_widget(paragraph, rect);
        }
    }

    // Status bar
    let status_line = Line::from(vec![Span::styled(
        " Tab: fullscreen | j/k: scroll | PgUp/PgDn: page | Home/End: jump | q/Esc: quit ",
        Style::default().fg(BRIGHT_BLACK),
    )]);
    let status_rect = Rect {
        x: area.x,
        y: area.y + area.height.saturating_sub(1),
        width: area.width,
        height: 1,
    };
    f.render_widget(Paragraph::new(status_line), status_rect);
}

fn draw_fullscreen(f: &mut Frame, app: &App) {
    let area = f.area();

    if app.panels.is_empty() {
        return;
    }

    let panel = &app.panels[app.selected];

    let main_area = Rect {
        x: area.x,
        y: area.y,
        width: area.width,
        height: area.height.saturating_sub(1),
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(panel.border_color))
        .title(Line::from(vec![
            Span::styled(
                format!(" {} ", panel.icon),
                Style::default().fg(panel.border_color),
            ),
            Span::styled(
                format!("{} ", panel.title),
                Style::default()
                    .fg(BRIGHT_WHITE)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));

    let paragraph = Paragraph::new(panel.content.clone())
        .block(block)
        .scroll((app.fullscreen_scroll, 0));

    f.render_widget(paragraph, main_area);

    // Status bar
    let total = app.panels.len();
    let current = app.selected + 1;
    let content_lines = panel.content.len() as u16;
    let inner_h = main_area.height.saturating_sub(2);
    let scroll_indicator = if content_lines > inner_h {
        let pct = ((app.fullscreen_scroll as u32 * 100)
            / (content_lines.saturating_sub(inner_h)) as u32)
            .min(100);
        format!(" {}% ", pct)
    } else {
        String::new()
    };
    let status = format!(
        " {}/{}{} | <-/-> or h/l: prev/next | j/k: scroll | Home/End: top/bottom | Tab/Esc: grid | q: quit ",
        current, total, scroll_indicator
    );
    let status_line = Line::from(vec![
        Span::styled(
            format!(" {} {} ", panel.icon, panel.title),
            Style::default()
                .fg(BRIGHT_WHITE)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(status, Style::default().fg(BRIGHT_BLACK)),
    ]);

    let status_rect = Rect {
        x: area.x,
        y: area.y + area.height.saturating_sub(1),
        width: area.width,
        height: 1,
    };
    f.render_widget(Paragraph::new(status_line), status_rect);
}
