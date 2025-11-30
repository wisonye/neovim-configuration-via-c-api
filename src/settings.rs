use nvim_oxi as nvim;

use nvim::api::opts::{OptionOpts, OptionScope};
use nvim::api::set_option_value;

///
///
///
pub fn run() {
    let opts = OptionOpts::builder().scope(OptionScope::Local).build();

    //
    // Enable hybrid number (Absolute and relative number)
    //
    let _ = set_option_value("number", true, &opts);
    let _ = set_option_value("relativenumber", true, &opts);
    let _ = set_option_value("signcolumn", "yes", &opts);

    // Hide the vim mode
    let _ = set_option_value("showmode", false, &opts);

    // Share system clipboard
    let _ = set_option_value("clipboard", "unnamedplus", &opts);

    // Case-insensitive searching
    let _ = set_option_value("ignorecase", true, &opts);
    let _ = set_option_value("smartcase", true, &opts);

    // Show or disable a vertical bar to indicate the column limit position.
    let _ = set_option_value("colorcolumn", "80", &opts);

    //
    // Tab related
    //
    const TAB_INDENT_WIDTH: usize  = 4;
    let _ = set_option_value("expandtab", true, &opts);
    let _ = set_option_value("shiftwidth", TAB_INDENT_WIDTH, &opts);
    let _ = set_option_value("smartindent", true, &opts);
    let _ = set_option_value("tabstop", TAB_INDENT_WIDTH, &opts);
    let _ = set_option_value("softtabstop", TAB_INDENT_WIDTH, &opts);
    let _ = set_option_value("shiftround", true, &opts);

}
