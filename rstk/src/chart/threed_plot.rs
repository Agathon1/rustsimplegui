//! 3D plot - draws points across three axes
//!

use crate::chart::plotchart;
use crate::canvas;
use crate::widget;
use crate::wish;

/// Refers to a 3D plot
#[derive(Clone, Debug, PartialEq)]
pub struct Tk3DPlot {
    pub id: String,
}

/// Creates an instance of a 3D plot in given canvas.
pub fn make_3d_plot(
    canvas: &canvas::TkCanvas,
    x_axis: (f64, f64, f64),
    y_axis: (f64, f64, f64),
    z_axis: (f64, f64, f64)
) -> Tk3DPlot {
    let id = wish::next_var();
    let msg = format!(
        "global {}; set {} [::Plotchart::create3DPlot {} {{ {} {} {} }} {{ {} {} {} }} {{ {} {} {} }}]",
        id, id, &canvas.id, 
        x_axis.0, x_axis.1, x_axis.2, 
        y_axis.0, y_axis.1, y_axis.2, 
        z_axis.0, z_axis.1, z_axis.2
    );
    wish::tell_wish(&msg);

    Tk3DPlot { id }
}

/// Creates an instance of a 3D plot in given canvas with given labels for x-axis.
pub fn make_3d_plot_with_labels(
    canvas: &canvas::TkCanvas,
    x_axis: (f64, f64, f64),
    y_axis: (f64, f64, f64),
    z_axis: (f64, f64, f64),
    x_labels: &[&str]
) -> Tk3DPlot {
    let id = wish::next_var();
    let msg = format!(
        "global {}; set {} [::Plotchart::create3DPlot {} {{ {} {} {} }} {{ {} {} {} }} {{ {} {} {} }} {{{}}}]",
        id, id, &canvas.id, 
        x_axis.0, x_axis.1, x_axis.2, 
        y_axis.0, y_axis.1, y_axis.2, 
        z_axis.0, z_axis.1, z_axis.2,
        widget::strings_list(x_labels)
    );
    wish::tell_wish(&msg);

    Tk3DPlot { id }
}

impl plotchart::TkPlotchart for Tk3DPlot {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl Tk3DPlot  {
    /// Sets colours for drawing plot.
    pub fn colours(&self, fill_colour: &str, border_colour: &str) {
        let msg = format!("global {}; ${} colour {} {}",
                          &self.id, &self.id, fill_colour, border_colour);
        wish::tell_wish(&msg);
    }

    /// Sets number of cells in x/y directions for grid.
    pub fn grid_size(&self, num_x: u64, num_y: u64) {
        let msg = format!("global {}; ${} gridsize {} {}",
                          &self.id, &self.id, num_x, num_y);
        wish::tell_wish(&msg);
    }

    /// Plots given data with interpolated contours.
    pub fn interpolate_data<M: AsRef<[R]>, R: AsRef<[f64]>>(&self, data: M, contours: &[f64]) {
        let msg = format!("global {}; ${} interpolatedata {{{}}} {{{}}}",
                          &self.id, &self.id, 
                          widget::str_list_lists(&data), widget::str_list(contours));
        wish::tell_wish(&msg);
    }

    /// Plot given data.
    pub fn plot_data<M: AsRef<[R]>, R: AsRef<[f64]>>(&self, data: M) {
        let msg = format!("global {}; ${} plotdata {{{}}}",
                          &self.id, &self.id, widget::str_list_lists(&data));
        wish::tell_wish(&msg);
    }

    /// Plot given data as a ribbon.
    pub fn ribbon(&self, yz_pairs: &[(f64, f64)]) {
        let mut yz_str = String::new();
        for (y, z) in yz_pairs {
            yz_str.push_str(&format!("{{ {} {} }} ", y, z));
        }

        let msg = format!("global {}; ${} ribbon {{{}}}",
                          &self.id, &self.id, yz_str);
        wish::tell_wish(&msg);
    }
}

