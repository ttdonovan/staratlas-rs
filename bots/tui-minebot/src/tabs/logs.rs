use ratatui::prelude::*;
use tui_logger::TuiLoggerWidget;

#[derive(Default)]
pub struct LogsTab;

impl Widget for LogsTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        TuiLoggerWidget::default().render(area, buf);
    }
}
