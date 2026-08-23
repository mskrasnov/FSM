/* line_charts.rs
 *
 * Copyright 2025-2026 Michail Krasnov <mskrasnov07@ya.ru>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

//! Linear charts
//!
//! ## Example
//! ```rust,ignore
//! use iced::Color;
//! use ferrix_app::widgets::line_chart::{LineChart, YAxisFormat};
//! use ferrix_app::message::Message;
//!
//! // Step 1. Initialize the chart with optional predefined colors
//! let mut chart = LineChart::new(vec![
//!     Color::from_rgb8(255, 99, 71),
//!     Color::from_rgb8(60, 179, 113),
//! ]);
//!
//! // Step 2. Configure the chart
//! chart.set_y_axis_format(YAxisFormat::Percentage);
//! chart.set_max_values(100); // keep last 100 data points
//!
//! // Step 3. Add data series (colors are assigned automatically)
//! for i in 0..32 {
//!     chart.add_series(format!("CPU #{i}"));
//! }
//!
//! // Step 4. Push the data
//! let mut j = 0.1;
//! for i in 0..32 {
//!     chart.push_value(j, i); // j% for CPU #{i}
//!     j += 0.5;
//! }
//!
//! // Step 5. Render the chart in yor `view() -> Element<'a, Message>`
//! // function
//! chart.view();
//! ```

use ferrix_lib::utils::Size as UnitSize;
use iced::{
    Color as IColor, Element, Font, Length, Size, Theme, font,
    widget::{
        canvas::{Cache, Frame, Geometry},
        column, container, grid, row, text,
    },
};
use plotters::prelude::*;
use plotters_iced2::{Chart, ChartBuilder, ChartWidget, DrawingBackend};
use std::collections::VecDeque;

use crate::message::Message;

/// Line chart widget structure
#[derive(Debug, Clone)]
pub struct LineChart {
    data: Vec<LineSeries>,
    max_points: usize,
    style: Style,
    show_legend: bool,
    y_axis_format: YAxisFormat,
    palette: ColorPalette,
}

/// Represents a single line (series) on the chart
///
/// Uses a `VecDeque` for efficient insertion at the back and removal from
/// the front (for rolling time-series data).
#[derive(Debug, Clone)]
pub struct LineSeries {
    name: String,
    data: VecDeque<f64>,
    color: RGBColor,
    max_points: usize,
    y_max: f64,
}

/// Visual styling options for the chart
#[derive(Debug, Clone)]
pub struct Style {
    /// The color used for the y-axis labels and grid lines
    pub y_axis_color: IColor,

    /// The thickness of the line drawn for each data series
    pub line_thickness: u32,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            y_axis_color: IColor::WHITE,
            line_thickness: 1,
        }
    }
}

fn to_rgbcolor(color: IColor) -> RGBColor {
    let oc = color.into_rgba8();
    RGBColor(oc[0], oc[1], oc[2])
}

fn to_icolor(color: RGBColor) -> IColor {
    let (r, g, b) = (color.0, color.1, color.2);
    IColor::from_rgb8(r, g, b)
}

impl LineSeries {
    /// Create a new data series with the given name, color and
    /// maximum capacity
    pub fn new(name: String, color: IColor, max_len: usize) -> Self {
        Self {
            name,
            max_points: max_len,
            color: to_rgbcolor(color),
            data: VecDeque::with_capacity(max_len),
            y_max: 100.,
        }
    }

    /// Push a new value to the series
    ///
    /// If the number of data points exceeds `max_points`, the oldest
    /// value is automatically removed from the front of the queue
    pub fn push(&mut self, value: f64) {
        if self.data.len() > self.max_points {
            self.data.pop_front();
        }

        self.data.push_back(value);
    }

    /// Sets a custom maximum Y value specifically for this series
    pub fn set_y_max(&mut self, y: f64) {
        self.y_max = y;
    }
}

impl LineChart {
    /// Create a new `LineChart`
    ///
    /// `predefined_colors` - a vector of `iced::Color` to be used first
    /// when assigning colors to new series. Once exhausted, the chart will
    /// automatically generate distinguishable colors
    pub fn new(predefined_colors: Vec<IColor>) -> Self {
        Self {
            data: Vec::with_capacity(8),
            max_points: 100,
            style: Style::default(),
            show_legend: true,
            y_axis_format: YAxisFormat::default(),
            palette: ColorPalette::new(predefined_colors),
        }
    }

    /// Set the chart's style to match the current application theme
    pub fn set_style(&mut self, theme: &Theme) {
        let style = Style {
            y_axis_color: theme.palette().text,
            line_thickness: self.style.line_thickness,
        };
        self.style = style;
    }

    /// Set the formatting style for the Y-axis labels
    pub fn set_y_axis_format(&mut self, fmt: YAxisFormat) {
        self.y_axis_format = fmt;
    }

    /// Set the thickness (px) of the lines drawn for each data series
    pub fn set_line_thickness(&mut self, thickness: u32) {
        self.style.line_thickness = thickness;
    }

    /// Set the maximum number of data points to retain for all series
    ///
    /// Existing data exceeding the new limit will be truncated
    pub fn set_max_values(&mut self, value: usize) {
        self.max_points = value;
        for s in &mut self.data {
            s.max_points = value;
        }
        self.update_axis();
    }

    /// Returns the current number of data series on the chart
    pub fn series_count(&self) -> usize {
        self.data.len()
    }

    /// Add a new data series to the chart with an automatically assigned
    /// color
    ///
    /// The color is pulled from the predefined palette, or generating
    /// automatically.
    pub fn add_series(&mut self, name: String) {
        let color = self.palette.next_color();
        let series = LineSeries::new(name, color, self.max_points);
        self.push_series(series);
    }

    /// Manually adds a pre-configured `LineSeries` to the chart
    pub fn push_series(&mut self, value: LineSeries) {
        self.data.push(value);
    }

    /// Push a value to a specific series by its index
    ///
    /// This method uses the series' internal `push` logic, which handles
    /// capacity limits but does *not* trigger a global axis update.
    pub fn push_to(&mut self, idx: usize, value: f64) {
        if self.data.len() < idx {
            return;
        }
        self.data[idx].push(value);
    }

    /// Push a value to a specific series by its index and triggers an axis
    /// update
    ///
    /// Unlike `push_to`, this method directly manipulates the `VecDeque`
    /// and calls `update_axis()` to ensure the global Y-axis maximum
    /// is recalculated.
    pub fn push_value(&mut self, value: f64, idx: usize) {
        if self.data.len() < idx {
            return;
        }
        self.update_axis();
        self.data[idx].data.push_back(value);
    }

    /// Toggle the visibility of the chart legend
    pub fn set_show_legend(&mut self, show: bool) {
        self.show_legend = show;
    }

    /// Generate the UI widget for the chart legend
    ///
    /// Displays the series name, its assigned color, and the most recent
    /// value, with conditional text styling (e.g. warning colors for
    /// values > 70%).
    pub fn legend_parameters<'a>(&'a self) -> Element<'a, Message> {
        let mut items = Vec::with_capacity(self.data.len());
        let bold_font = {
            let mut font = Font::default();
            font.weight = font::Weight::Bold;
            font
        };

        for line in &self.data {
            let value = line.data.back().copied().unwrap_or(0.);
            items.push(
                row![
                    text(format!("{}:", &line.name))
                        .color(to_icolor(line.color))
                        .font(bold_font),
                    text(self.y_axis_format.format(&value)).style(move |t| if value > 90. {
                        text::danger(t)
                    } else if value > 70. {
                        text::warning(t)
                    } else {
                        text::default(t)
                    }),
                ]
                .spacing(3),
            );
        }

        let mut gr = grid([]).columns(8).fluid(125.).height(Length::Shrink);
        for item in items {
            gr = gr.push(item);
        }
        container(gr).into()
    }

    /// Generate the main `Element` with chart and (optionally) legend below this chart
    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        let chart = ChartWidget::new(self);
        if self.show_legend {
            column![chart, self.legend_parameters()].into()
        } else {
            chart.into()
        }
    }

    /// Internal method to trim data points across all series if they exceed
    /// `max_points`
    fn update_axis(&mut self) {
        'm: loop {
            for s in &mut self.data {
                if s.data.len() > self.max_points {
                    s.data.pop_front();
                } else {
                    break 'm;
                }
            }
        }
    }

    /// Calculate the maximum Y-axis value across all series and their data
    /// points
    ///
    /// This ensures the chart's Y-axis dynamically scales to fit the data,
    /// falling back to the series' predefined `y_max` if the data is
    /// empty or low.
    fn y_max(&self) -> f64 {
        let mut y = 0.;
        for series in &self.data {
            let mx = series.y_max;
            if y < mx {
                y = mx;
                break;
            }

            for &value in &series.data {
                if value > y {
                    y = value;
                }
            }
        }
        y
    }

    /// Force a specific maximum Y-axis value for all series on the chart
    ///
    /// Use this to override the automatic dynamic scaling if a fixed scale is
    /// required.
    pub fn set_y_max(&mut self, y: f64) {
        for series in &mut self.data {
            series.set_y_max(y);
        }
    }
}

impl Chart<Message> for LineChart {
    type State = ();

    #[inline]
    fn draw<R: plotters_iced2::Renderer, F: Fn(&mut Frame)>(
        &self,
        renderer: &R,
        size: Size,
        f: F,
    ) -> Geometry {
        renderer.draw_cache(&Cache::new(), size, f)
    }

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut builder: ChartBuilder<DB>) {
        let y_max = self.y_max();

        let mut chart = builder
            .x_label_area_size(0)
            .y_label_area_size(35)
            .margin(5)
            .build_cartesian_2d(0..(self.max_points), 0.0..y_max)
            .expect("Failed to build chart");

        chart
            .configure_mesh()
            .axis_style(to_rgbcolor(self.style.y_axis_color).mix(0.05))
            .bold_line_style(to_rgbcolor(self.style.y_axis_color).mix(0.05))
            .light_line_style(TRANSPARENT)
            .disable_x_axis()
            .disable_x_mesh()
            .y_labels(8)
            .x_labels(self.max_points)
            .y_label_style(
                ("sans-serif", 10)
                    .into_font()
                    .color(&to_rgbcolor(self.style.y_axis_color))
                    .transform(FontTransform::Rotate270),
            )
            .y_label_formatter(&|y: &f64| self.y_axis_format.format(y))
            .draw()
            .expect("Failed to draw chart mesh");

        for series in &self.data {
            chart
                .draw_series(
                    AreaSeries::new(
                        series.data.iter().enumerate().map(|x| (x.0, *x.1 as f64)),
                        0.,
                        plotters::style::TRANSPARENT,
                    )
                    .border_style(
                        ShapeStyle::from(series.color).stroke_width(self.style.line_thickness),
                    ),
                )
                .expect("Failed to draw chart data")
                .label(&series.name)
                .legend(|(x, y)| {
                    Rectangle::new([(x - 5, y - 3), (x + 15, y + 8)], series.color.filled())
                });
        }
    }
}

/// Formatting style for the Y-axis labels
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum YAxisFormat {
    #[default]
    Percentage,

    /// Human-readable bytes (e.g. `1.5 GB`) using `ferrix_lib::utils::Size`
    Bytes,

    /// Frequency, MHz
    Frequency,

    /// Plain f64
    Plain,
}

impl YAxisFormat {
    /// Format the given `f64` value according to the selected `YAxisFormat`
    /// variant
    pub fn format(&self, value: &f64) -> String {
        match self {
            Self::Percentage => format!("{value:.3}%"),
            Self::Bytes => {
                let size = UnitSize::B(*value as u64).round(2).unwrap_or_default();
                size.to_string()
            }
            Self::Frequency => format!("{value:.3} MHz"),
            Self::Plain => format!("{value:.3}"),
        }
    }
}

/// Colors of data series management structure
#[derive(Debug, Clone)]
pub struct ColorPalette {
    predefined: Vec<IColor>,
    current_idx: usize,
}

impl ColorPalette {
    pub fn new(predefined: Vec<IColor>) -> Self {
        Self {
            predefined,
            current_idx: 0,
        }
    }

    pub fn next_color(&mut self) -> IColor {
        if self.current_idx < self.predefined.len() {
            let color = self.predefined[self.current_idx];
            self.current_idx += 1;
            color
        } else {
            let color = generate_color(self.current_idx);
            self.current_idx += 1;
            color
        }
    }

    pub fn reset(&mut self) {
        self.current_idx = 0;
    }
}

fn generate_color(idx: usize) -> IColor {
    let hue = ((idx as f32) * 137.50776) % 360.0;
    hsv_to_rgb(hue, 0.75, 0.9)
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> IColor {
    let c = v * s;
    let x = c * (1.0 - (((h / 60.0) % 2.0) - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match h {
        h if h < 60.0 => (c, x, 0.0),
        h if h < 120.0 => (x, c, 0.0),
        h if h < 180.0 => (0.0, c, x),
        h if h < 240.0 => (0.0, x, c),
        h if h < 300.0 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    IColor {
        r: r + m,
        g: g + m,
        b: b + m,
        a: 1.0,
    }
}
