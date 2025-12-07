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

    let _ = set_var("mapleader", " ");

    let my_common_keybindings: Vec<(Mode, &str, &str, &str)> = vec![
        // -----------------------------------------------------------------------------------,
        // Normal settings
        // -----------------------------------------------------------------------------------,
        (Mode::Normal, "Y", "y$", "Copy to end of line"),
        (Mode::Normal, "H", "^", "Move to begining of line"),
        (Mode::Normal, "L", "$", "Move to end of line"),
        (Mode::Normal, "W", ":w<CR>", "Save current buffer"),
        (Mode::Normal, "Q", ":q<CR>", "Quit current buffer"),
        (Mode::Insert, "jj", "<ESC>", "'jj': Escape from insert mode"),
        (
            Mode::Normal,
            "<Tab>",
            ":bn<CR>",
            "'Tab': Switch to next buffer",
        ),
        (
            Mode::Normal,
            "<S-Tab>",
            ":bp<CR>",
            "'Shift + Tab': Switch to prev buffer",
        ),
        (
            Mode::Normal,
            "<Space><Space>",
            "<c-^>",
            "'<leader><leader>': Toggles between buffers",
        ),
        (
            Mode::Normal,
            "<Space>th",
            ":%TOhtml<CR>",
            "'<leader>th': To HTML",
        ),
        // ------------------------------------------------------------------------------------
        // ctrl+s: replace all words under cursor.
        // <c-r><c-w> to grab the word under cursor
        // ------------------------------------------------------------------------------------
        (
            Mode::Normal,
            "<c-s>",
            ":%s/<C-r><C-w>//g<left><left>",
            "'<C-s>': Replace all words under the cursor",
        ),
        // ------------------------------------------------------------------------------------
        // Split & window movement
        // ------------------------------------------------------------------------------------

        // Vertical; split
        (Mode::Normal, "<leader>vs", ":vsplit<CR>", "Vertical split"),
        // Move between windows
        (
            Mode::Normal,
            "<leader>j",
            ":wincmd j<CR>",
            "Move to down window",
        ),
        (
            Mode::Normal,
            "<leader>k",
            ":wincmd k<CR>",
            "Move to up window",
        ),
        (
            Mode::Normal,
            "<C-h>",
            ":wincmd h<CR>",
            "Move to left window",
        ),
        (
            Mode::Normal,
            "<C-l>",
            ":wincmd l<CR>",
            "Move to right window",
        ),
        // Resize windows
        (
            Mode::Normal,
            "-",
            ":vertical resize -5<CR>",
            "Decrease window size",
        ),
        (
            Mode::Normal,
            "=",
            ":vertical resize +5<CR>",
            "Increase window size",
        ),
        (Mode::Normal, "|", "<C-w>=", "Equal window size"),
        // ------------------------------------------------------------------------------------
        // Selections:
        //
        // shift+j: move selection down
        // shift+k: move selection up
        // <: left indent
        // >: right indent
        // ------------------------------------------------------------------------------------
        (
            Mode::VisualSelect,
            "J",
            ":m '>+1<CR>gv=gv",
            "Move selection down",
        ),
        (
            Mode::VisualSelect,
            "K",
            ":m '<-2<CR>gv=gv",
            "Move selection up",
        ),
        // Space -> Newline: replace all spaces to newlines
        (
            Mode::VisualSelect,
            "<leader>sn",
            ":s/ /\\r/g<CR>",
            "Space->Newline: replace all spaces to newlines",
        ),
        // Function Newline: replace all ', ' --> ',\r'"
        (
            Mode::VisualSelect,
            "<leader>fn",
            ":s/, /,\\r/g<CR>$F)i<CR><ESC>",
            "Function->Newline: replace all ', ' --> ',\r'",
        ),
        // ------------------------------------------------------------------------------------
        // Basic searching improvement
        // ------------------------------------------------------------------------------------
        (Mode::Normal, "<leader>n", ":nohl<CR>", "No highlight"),
        (Mode::Normal, "n", "nzz", "Jump to next matching and center"),
        (Mode::Normal, "N", "Nzz", "Jump to prev matching and center"),
        // Snippets (suggestion list) select item up and down
        (Mode::Insert, "<c-j>", "<c-n>", ""),
        (Mode::Insert, "<c-k>", "<c-p>", ""),
        (Mode::Normal, "<leader>oq", ":copen<CR>", "Open quick list"),
        (
            Mode::Normal,
            "<leader>cq",
            ":cclose<CR>",
            "Close quick list",
        ),
        (
            Mode::Normal,
            "<leader>ol",
            ":lopen<CR>",
            "Open location list",
        ),
        (
            Mode::Normal,
            "<leader>cl",
            ":lclose<CR>",
            "Close location list",
        ),
        // Cycle through the quick fix list and center the current result line
        (Mode::Normal, "<c-j>", ":cnext<CR>zz", ""),
        (Mode::Normal, "<c-k>", ":cNext<CR>zz", ""),
        // ------------------------------------------------------------------------------------
        // Handy book marks
        // ------------------------------------------------------------------------------------

        // `mm`: Make a gloabl mark
        (Mode::Normal, "mm", "mM", "Mark current position"),
        // `gb`: Go back to the global mark
        (
            Mode::Normal,
            "gb",
            "`Mzz",
            "Go back to last marked position",
        ),
        // ------------------------------------------------------------------------------------
        // Tab related
        // ------------------------------------------------------------------------------------
        (
            Mode::Normal,
            "<leader>to",
            "<cmd>tabnew<CR>",
            "Tab: open new tab",
        ),
        (
            Mode::Normal,
            "<leader>tc",
            "<cmd>tabclose<CR>",
            "Tab: close current tab",
        ),
        (
            Mode::Normal,
            "<leader>tn",
            "<cmd>tabn<CR>",
            "Tab: go to next tab",
        ),
        (
            Mode::Normal,
            "<leader>tp",
            "<cmd>tabp<CR>",
            "Tab: go to prev tab",
        ),
        (
            Mode::Normal,
            "<leader>tb",
            "<cmd>tabnew %<CR>",
            "Tab: open current buffer in new tab",
        ),
        // ------------------------------------------------------------------------------------
        // Terminal related
        // ------------------------------------------------------------------------------------
        (
            Mode::Normal,
            "<leader>ot",
            ":vsplit<CR>:terminal<CR>",
            "Terminal: open terminal",
        ),
        (
            Mode::Terminal,
            "<ESC>",
            "<C-\\><C-n>",
            "Terminal: Press `<ESC>` to back to normal mode",
        ),
        (
            Mode::Terminal,
            "<C-h>",
            "<C-\\><C-n><C-w>h",
            "Terminal: Press `<C-h>` to go back to the left window",
        ),
        // ------------------------------------------------------------------------------------
        // Command line related
        // ------------------------------------------------------------------------------------
        (Mode::CmdLine, "<C-j>", "<Down>", "Next history command"),
        (Mode::CmdLine, "<C-k>", "<Up>", "Previous history command"),
        // ------------------------------------------------------------------------------------
        // Evacuate/run the selected lua code
        // ------------------------------------------------------------------------------------
        (
            Mode::VisualSelect,
            "<leader>ee",
            ":'<,'>lua<CR>",
            "Evaluate selected lua code",
        ),
    ];

    for bindings in my_common_keybindings {
        let _ = set_keymap(
            bindings.0,
            bindings.1,
            bindings.2,
            &SetKeymapOpts::builder().desc(bindings.3).build(),
        );
    }

    let my_keybindings_with_callback: Vec<(Mode, &str, &str, Box<dyn Fn()>)> = vec![
        (
            Mode::Normal,
            "<leader>1",
            "'<leader>1': Kill other windows.",
            Box::new(|| {
                kill_other_windows();
            }),
        ),
        (
            Mode::Normal,
            "<leader>sc",
            "'<leader>sc': Toggle spell checking.",
            Box::new(|| {
                toggle_spell_checking();
            }),
        ),
        (
            Mode::Normal,
            "<leader>ft",
            "'<leader>ft': Open a floating terminal.",
            Box::new(|| {
                open_centred_floating_terminal_window();
            }),
        ),
    ];

    for bindings in my_keybindings_with_callback {
        let _ = set_keymap(
            bindings.0,
            bindings.1,
            "",
            &SetKeymapOpts::builder()
                .desc(bindings.2)
                .callback(move |_| {
                    bindings.3();
                    ()
                })
                .build(),
        );
    }
}

use crate::utils::{
    kill_other_windows, open_centred_floating_terminal_window, toggle_spell_checking,
};

use nvim_oxi::api::{opts::SetKeymapOpts, set_keymap, set_var, types::Mode};
