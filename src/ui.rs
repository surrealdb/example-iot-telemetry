use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Style, Stylize},
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
        let left_block = Block::new()
            .title("Sensors")
            .title_alignment(Alignment::Center);
        let inner = left_block.inner(main[0]);
        let graph_width = inner.width as usize;
        left_block.render(main[0], buf);

        for i in 0..self.sensor_count {
            if let Some(sensor) = sensors.get(&format!("sensor-{}", i)) {
                constraints.push(Constraint::Fill(1));
                sparklines.push(
                    Sparkline::default()
                        .block(
                            Block::bordered()
                                .title(sensor.sensor.clone())
                                .border_style(if self.selected_sensor == i {
                                    Style::new().blue()
                                } else {
                                    Style::new()
                                })
                                .border_type(if self.selected_sensor == i {
                                    BorderType::QuadrantOutside
                                } else {
                                    BorderType::Plain
                                }),
                        )
                        .max(100)
                        .data(
                            sensor
                                .values
                                .iter()
                                .rev()
                                .take(graph_width - 2)
                                .rev()
                                .map(|x| (x * 100. + 100.) as u64 / 2),
                        ),
                );
            }
        }
        let sensor_containers = Layout::vertical(constraints).split(inner);
        for (i, sparkline) in sparklines.iter().enumerate() {
            let area = sensor_containers[i];
            sparkline.render(area, buf);
        }

        // -- Right container
        let rcol = Layout::vertical([Constraint::Fill(1); 2]).split(main[1]);
        let block = Block::bordered().border_type(BorderType::Rounded);

        let text = format!(
            "Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                Data window in minutes: {}\n\
                Selected: {}\n\
                Sensor count: {}",
            self.window_in_minutes.read().unwrap(),
            self.selected_sensor,
            self.sensor_count
        );

        let mut text2 = String::new();
        if let Ok(avgs) = self.avgs.read() {
            for avg in avgs.iter() {
                text2 += &format!(
                    "{}: {:>8}\n",
                    avg.sensor.key(),
                    format!("{:.2}%", avg.avg * 100.)
                );
            }
        }
        Paragraph::new(text2)
            .block(Block::bordered().title(format!(
                "Last minute averages (query every {}ms)",
                self.query_delay
            )))
            .render(rcol[0], buf);

        Paragraph::new(text).block(block).render(rcol[1], buf);
    }
}
