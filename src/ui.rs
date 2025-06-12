use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Sparkline, Widget},
};

use crate::app::App;

impl Widget for &App {
    /// Renders the user interface widgets.
    ///
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let mut constraints = Vec::new();
        let mut sparklines = Vec::new();
        let sensors = self.sensors.read().unwrap();

        // -- Left container
        let left_block = Block::bordered()
            .title("Sensors")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);
        left_block.render(main[0], buf);

        for (key, sensor) in sensors.iter() {
            constraints.push(Constraint::Fill(1));
            sparklines.push(
                Sparkline::default()
                    .block(
                        Block::new()
                            .title(key.to_string())
                            .border_type(BorderType::Rounded),
                    )
                    .max(100)
                    .data(sensor.values.iter().map(|x| (x * 100. + 100.) as u64 / 2)),
            );
        }
        let sensor_containers = Layout::vertical(constraints).split(main[0]);
        for (i, sparkline) in sparklines.iter().enumerate() {
            sparkline.render(sensor_containers[i], buf);
        }

        // -- Right container
        let block = Block::bordered()
            .title("block")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let text = format!(
            "This is a tui template.\n\
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                Selected: {}\n\
                Sensor count: {}",
            self.selected_sensor, self.sensor_count
        );

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::Cyan)
            .bg(Color::Black)
            .centered();

        paragraph.render(main[1], buf);
    }
}
