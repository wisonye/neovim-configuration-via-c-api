use nvim_oxi as nvim;

use nvim::api::opts::SetKeymapOpts;
use nvim::api::set_keymap;
use nvim::api::types::Mode;

//
// pub enum Mode {
//     #[serde(rename = "c")]
//     CmdLine,
//     #[serde(rename = "i")]
//     Insert,
//     #[serde(rename = "!")]
//     InsertCmdLine,
//     #[serde(rename = "l")]
//     Langmap,
//     #[serde(rename(deserialize = " "))]
//     NormalVisualOperator,
//     #[serde(rename = "n")]
//     Normal,
//     #[serde(rename = "o")]
//     OperatorPending,
//     #[serde(rename = "s")]
//     Select,
//     #[serde(rename = "t")]
//     Terminal,
//     #[serde(rename = "x")]
//     Visual,
//     #[serde(rename = "v")]
//     VisualSelect,
// }
//

///
///
///
pub fn setup() {
    //
    // Leader key: <Space>
    //
    let _ = set_keymap(
        Mode::Normal,
        "<Space>",
        "<NOP>",
        &SetKeymapOpts::builder().silent(true).build(),
    );

    // Y: Copy to the end of the line
    let _ = set_keymap(
        Mode::Normal,
        "Y",
        "y$",
        &SetKeymapOpts::builder().desc("Copy to end of line").build(),
    );

    // ------------------------------------------------------------------------------------
    // Normal settings
    // ------------------------------------------------------------------------------------

    //
    // H and L instead of '^' and '$'
    //
    let _ = set_keymap(
        Mode::Normal,
        "H",
        "^",
        &SetKeymapOpts::builder().desc("Move to begining of line").build(),
    );

    let _ = set_keymap(
        Mode::Normal,
        "L",
        "$",
        &SetKeymapOpts::builder().desc("Move to end of line").build(),
    );

    //
    // W: save, Q: quit
    //
    let _ = set_keymap(
        Mode::Normal,
        "W",
        ":w<CR>",
        &SetKeymapOpts::builder().desc("Save current buffer").build(),
    );
    let _ = set_keymap(
        Mode::Normal,
        "Q",
        ":q<CR>",
        &SetKeymapOpts::builder().desc("Quit current buffer").build(),
    );

    // jj: <ESC> from `insert` mode
    let _ = set_keymap(
        Mode::Insert,
        "jj",
        "<ESC>",
        &SetKeymapOpts::builder().build(),
    );

    //
    // <Tab> and shift+<Tab> to cycle through the opened buffers
    //
    let _ = set_keymap(
        Mode::Normal,
        "<Tab>",
        ":bn<CR>",
        &SetKeymapOpts::builder().desc("Switch to next buffer").build(),
    );

    let _ = set_keymap(
        Mode::Normal,
        "<S-Tab>",
        ":bp<CR>",
        &SetKeymapOpts::builder().desc("Switch to prev buffer").build(),
    );

    // <leader><leader>: toggles between buffers
    let _ = set_keymap(
        Mode::Normal,
        "<Space><Space>",
        "<c-^>",
        &SetKeymapOpts::builder().desc("Swith between last and current buffer").build(),
    );

    //  <leader>th: save current file to HTML
    let _ = set_keymap(
        Mode::Normal,
        "<Space>th",
        ":%TOhtml<CR>",
        &SetKeymapOpts::builder().desc("Save as HTML").build(),
    );


}
