//! Radial chart - plots points along a circular axis.
//!

use crate::chart::plotchart;
use crate::canvas;
use crate::wish;

/// Refers to a radial chart
#[derive(Clone, Debug, PartialEq)]
pub struct TkRadialChart {
    pub id: String,
}

/// Creates an instance of a radial chart in given canvas.
pub fn make_radial_chart(
    canvas: &canvas::TkCanvas,
    labels: &[&str],
    scale: f64,
    style: plotchart::RadialStyle
) -> TkRadialChart {
    let mut labels_str = String::new();
    for label in labels {
        labels_str.push('{');
        labels_str.push_str(label);
        labels_str.push('}');
        labels_str.push(' ');
    }

    let id = wish::next_var();
    let msg = format!(
        "global {}; set {} [::Plotchart::createRadialchart {} {{{}}} {} {}]",
        id, id, &canvas.id, labels_str, scale, style
    );
    wish::tell_wish(&msg);

    TkRadialChart { id }
}

impl plotchart::TkPlotchart for TkRadialChart {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl TkRadialChart {
    /// Plot given data.
    pub fn plot(&self, data: &[f64], colour: &str, thickness: u64) {
        let mut data_str = String::new();
        for datum in data {
            data_str.push_str(&format!("{} ", datum));
        }

        let msg = format!("global {}; ${} plot {{{}}} {} {}",
                          &self.id, &self.id, data_str, colour, thickness);
        wish::tell_wish(&msg);
    }
}

