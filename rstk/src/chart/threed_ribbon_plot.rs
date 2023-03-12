//! 3D ribbon plot - draws a line on x-y plot, but with 3D appearance.
//!

use crate::chart::plotchart;
use crate::canvas;
use crate::wish;

/// Refers to a 3D ribbon plot
#[derive(Clone, Debug, PartialEq)]
pub struct Tk3DRibbonPlot {
    pub id: String,
}

/// Creates an instance of a 3D ribbon plot in given canvas.
pub fn make_3d_ribbon_plot(
    canvas: &canvas::TkCanvas,
    y_axis: (f64, f64, f64),
    z_axis: (f64, f64, f64)
) -> Tk3DRibbonPlot {
    let id = wish::next_var();
    let msg = format!(
        "global {}; set {} [::Plotchart::create3DRibbonPlot {} {{ {} {} {} }} {{ {} {} {} }}]",
        id, id, &canvas.id, 
        y_axis.0, y_axis.1, y_axis.2, 
        z_axis.0, z_axis.1, z_axis.2
    );
    wish::tell_wish(&msg);

    Tk3DRibbonPlot { id }
}

impl plotchart::TkPlotchart for Tk3DRibbonPlot {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl Tk3DRibbonPlot  {
    /// Plot given data.
    pub fn plot(&self, yz_pairs: &[(f64, f64)]) {
        let mut yz_str = String::new();
        for (y, z) in yz_pairs {
            yz_str.push_str(&format!("{{ {} {} }} ", y, z));
        }

        let msg = format!("global {}; ${} plot {{{}}}",
                          &self.id, &self.id, yz_str);
        wish::tell_wish(&msg);
    }
}

