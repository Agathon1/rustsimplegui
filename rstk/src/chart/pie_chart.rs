//! Pie chart - display data as slices of a circle.
//!

use crate::chart::plotchart;
use crate::canvas;
use crate::wish;

/// Refers to a pie chart
#[derive(Clone, Debug, PartialEq)]
pub struct TkPieChart {
    pub id: String,
}

/// Refers to a spiral pie chart
#[derive(Clone, Debug, PartialEq)]
pub struct TkSpiralPieChart {
    pub id: String,
}

/// Creates an instance of a pie chart in given canvas.
pub fn make_pie_chart(canvas: &canvas::TkCanvas) -> TkPieChart {
    let id = wish::next_var();
    let msg = format!("global {}; set {} [::Plotchart::createPiechart {}]",
                      id, id, &canvas.id);
    wish::tell_wish(&msg);

    TkPieChart { id }
}

/// Creates an instance of a spiral pie chart in given canvas.
pub fn make_spiral_pie_chart(canvas: &canvas::TkCanvas) -> TkSpiralPieChart {
    let id = wish::next_var();
    let msg = format!("global {}; set {} [::Plotchart::createSpiralPie {}]",
                      id, id, &canvas.id);
    wish::tell_wish(&msg);

    TkSpiralPieChart { id }
}

impl plotchart::TkPlotchart for TkPieChart {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl plotchart::TkPlotchart for TkSpiralPieChart {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

pub trait PieChartMethods: plotchart::TkPlotchart {
    /// Sets colours for slices.
    fn colours(&self, colours: &[&str]) {
        let mut colour_str = String::new();
        for colour in colours {
            colour_str.push_str(&format!("{} ", colour));
        }
        
        let msg = format!("global {}; ${} colours {}",
                          self.id(), self.id(), colour_str);
        wish::tell_wish(&msg);
    }

    /// Plots data, where data is a list of (label, angle) pairs.
    fn plot(&self, data: &[(&str, f64)]) {
        let mut data_str = String::new();
        for (label, angle) in data {
            data_str.push_str(&format!("{{{}}} {} ", label, angle));
        }

        let msg = format!("global {}; ${} plot {{{}}}",
                          self.id(), self.id(), data_str);
        wish::tell_wish(&msg);
    }
}

impl PieChartMethods for TkPieChart {}

impl TkPieChart {
        /// Given slice number is displayed offset from circle.
    pub fn explode(&self, slice_number: u64) {
        let msg = format!("global {}; ${} explode {}",
                          &self.id, &self.id, slice_number);
        wish::tell_wish(&msg);
    }
}

impl PieChartMethods for TkSpiralPieChart {}

