use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::app::App;

const PANEL_HEIGHT: u16 = 20;

const BRIGHT_WHITE: Color = Color::Indexed(15);

pub fn draw(f: &mut Frame, app: &App) {
    let area = f.area();

    // might want responsive columns later
    for (i, panel) in app.panels.iter().enumerate() {
        let y = i as u16 * PANEL_HEIGHT;
        if y >= area.height {
            break;
        }

        let height = PANEL_HEIGHT.min(area.height.saturating_sub(y));
        let rect = Rect {
            x: area.x,
            y,
            width: area.width,
            height,
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

        let paragraph = Paragraph::new(panel.content.clone()).block(block);
        f.render_widget(paragraph, rect);
    }
}
