use srclang::compiler::ir;

use crate::core::session::Session;

pub(crate) fn update_channel(tree: Option<&ir::Program>) {
    // assume errors; use red
    let mut color = "rgb(255, 87, 51)";
    if let Ok(channel_syntax) = Session::get_channel_syntax() {

        channel_syntax
            .style()
            .set_property("background-color", color)
            .expect("failed to set style");
    }
}
