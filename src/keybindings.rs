use crate::utils::{kill_other_windows, open_centred_floating_terminal_window};

use nvim_oxi::api::{
    Window, get_option_value,
    opts::{OptionOpts, SetKeymapOpts},
    set_keymap, set_option_value, set_var,
    types::Mode,
};

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

    // ------------------------------------------------------------------------------------
    // Reload the current config
    // ------------------------------------------------------------------------------------
    let _ = set_keymap(
        Mode::Normal,
        "<leader>rr",
        // ":luafile ~/.config/nvim/init.lua<CR>:setlocal nospell<CR>",
        ":lua print(\">>> Reload configuration\")",
        &SetKeymapOpts::builder()
            .desc("Reload neovim config")
            .build(),
    );

    // ------------------------------------------------------------------------------------
    // Normal settings
    // ------------------------------------------------------------------------------------

    // Y: Copy to the end of the line
    let _ = set_keymap(
        Mode::Normal,
        "Y",
        "y$",
        &SetKeymapOpts::builder().desc("Copy to end of line").build(),
    );

    //
    // H and L instead of '^' and '$'
    //
    let _ = set_keymap(
        Mode::Normal,
        "H",
        "^",
        &SetKeymapOpts::builder()
            .desc("Move to begining of line")
            .build(),
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
        &SetKeymapOpts::builder()
            .desc("Switch to next buffer")
            .build(),
    );

    let _ = set_keymap(
        Mode::Normal,
        "<S-Tab>",
        ":bp<CR>",
        &SetKeymapOpts::builder()
            .desc("Switch to prev buffer")
            .build(),
    );

    // <leader><leader>: toggles between buffers
    let _ = set_keymap(
        Mode::Normal,
        "<Space><Space>",
        "<c-^>",
        &SetKeymapOpts::builder()
            .desc("Swith between last and current buffer")
            .build(),
    );

    //  <leader>th: save current file to HTML
    let _ = set_keymap(
        Mode::Normal,
        "<Space>th",
        ":%TOhtml<CR>",
        &SetKeymapOpts::builder().desc("Save as HTML").build(),
    );

    // ------------------------------------------------------------------------------------
    // ctrl+s: replace all words under cursor.
    // <c-r><c-w> to grab the word under cursor
    // ------------------------------------------------------------------------------------
    let _ = set_keymap(
        Mode::Normal,
        "<c-s>",
        ":%s/<C-r><C-w>//g<left><left>",
        &SetKeymapOpts::builder()
            .desc("Replace all words under the cursor")
            .build(),
    );

    // ------------------------------------------------------------------------------------
    // Split & window movement
    // ------------------------------------------------------------------------------------

    // Vertical; split
    let _ = set_keymap(
        Mode::Normal,
        "<leader>vs",
        ":vsplit<CR>",
        &SetKeymapOpts::builder()
            .desc("Vertical split")
            .silent(true)
            .build(),
    );

    // Move between windows
    let _ = set_keymap(
        Mode::Normal,
        "<leader>j",
        ":wincmd j<CR>",
        &SetKeymapOpts::builder()
            .desc("Move to down window")
            .silent(true)
            .build(),
    );

    let _ = set_keymap(
        Mode::Normal,
        "<leader>k",
        ":wincmd k<CR>",
        &SetKeymapOpts::builder()
            .desc("Move to up window")
            .silent(true)
            .build(),
    );

    let _ = set_keymap(
        Mode::Normal,
        "<C-h>",
        ":wincmd h<CR>",
        &SetKeymapOpts::builder()
            .desc("Move to left window")
            .silent(true)
            .build(),
    );

    let _ = set_keymap(
        Mode::Normal,
        "<C-l>",
        ":wincmd l<CR>",
        &SetKeymapOpts::builder()
            .desc("Move to right window")
            .silent(true)
            .build(),
    );

    // Resize windows
    let _ = set_keymap(
        Mode::Normal,
        "-",
        ":vertical resize -5<CR>",
        &SetKeymapOpts::builder()
            .desc("Decrease window size")
            .silent(true)
            .build(),
    );
    let _ = set_keymap(
        Mode::Normal,
        "=",
        ":vertical resize +5<CR>",
        &SetKeymapOpts::builder()
            .desc("Increase window size")
            .silent(true)
            .build(),
    );
    let _ = set_keymap(
        Mode::Normal,
        "|",
        "<C-w>=",
        &SetKeymapOpts::builder()
            .desc("Equal window size")
            .silent(true)
            .build(),
    );

    // ------------------------------------------------------------------------------------
    // Selections:
    //
    // shift+j: move selection down
    // shift+k: move selection up
    // <: left indent
    // >: right indent
    // ------------------------------------------------------------------------------------
    let _ = set_keymap(
        Mode::VisualSelect,
        "J",
        ":m '>+1<CR>gv=gv",
        &SetKeymapOpts::builder()
            .desc("Move selection down")
            .silent(true)
            .build(),
    );
    let _ = set_keymap(
        Mode::VisualSelect,
        "K",
        ":m '<-2<CR>gv=gv",
        &SetKeymapOpts::builder()
            .desc("Move selection up")
            .silent(true)
            .build(),
    );
    // let _ = set_keymap(Mode::VisualSelect,
    //     "<",
    //     "<gv",
    //     &SetKeymapOpts::builder().desc("Left indent selections").silent(true).build(),
    // );
    // let _ = set_keymap(Mode::VisualSelect,
    //     ">",
    //     ">gv",
    //     &SetKeymapOpts::builder().desc("Right indent selections").silent(true).build(),
    // );

    // Space -> Newline: replace all spaces to newlines
    let _ = set_keymap(
        Mode::VisualSelect,
        "<leader>sn",
        ":s/ /\\r/g<CR>",
        &SetKeymapOpts::builder()
            .desc("Space->Newline: replace all spaces to newlines")
            .silent(true)
            .build(),
    );

    // Function Newline: replace all ', ' --> ',\r'"
    let _ = set_keymap(
        Mode::VisualSelect,
        "<leader>fn",
        ":s/, /,\\r/g<CR>$F)i<CR><ESC>",
        &SetKeymapOpts::builder()
            .desc("Function->Newline: replace all ', ' --> ',\r'")
            .silent(true)
            .build(),
    );

    // ------------------------------------------------------------------------------------
    // Basic searching improvement
    // ------------------------------------------------------------------------------------

    let _ = set_keymap(
        Mode::Normal,
        "<leader>n",
        ":nohl<CR>",
        &SetKeymapOpts::builder()
            .desc("No highlight")
            .silent(true)
            .build(),
    );

    let _ = set_keymap(
        Mode::Normal,
        "n",
        "nzz",
        &SetKeymapOpts::builder()
            .desc("Jump to next matching and center")
            .silent(true)
            .build(),
    );

    let _ = set_keymap(
        Mode::Normal,
        "N",
        "Nzz",
        &SetKeymapOpts::builder()
            .desc("Jump to prev matching and center")
            .silent(true)
            .build(),
    );

    // Snippets (suggestion list) select item up and down
    let _ = set_keymap(
        Mode::Insert,
        "<c-j>",
        "<c-n>",
        &SetKeymapOpts::builder().silent(true).build(),
    );
    let _ = set_keymap(
        Mode::Insert,
        "<c-k>",
        "<c-p>",
        &SetKeymapOpts::builder().silent(true).build(),
    );

    let _ = set_keymap(
        Mode::Normal,
        "<leader>oq",
        ":copen<CR>",
        &SetKeymapOpts::builder()
            .desc("Open quick list")
            .silent(true)
            .build(),
    );
    let _ = set_keymap(
        Mode::Normal,
        "<leader>cq",
        ":cclose<CR>",
        &SetKeymapOpts::builder()
            .desc("Close quick list")
            .silent(true)
            .build(),
    );

    let _ = set_keymap(
        Mode::Normal,
        "<leader>ol",
        ":lopen<CR>",
        &SetKeymapOpts::builder()
            .desc("Open location list")
            .silent(true)
            .build(),
    );
    let _ = set_keymap(
        Mode::Normal,
        "<leader>cl",
        ":lclose<CR>",
        &SetKeymapOpts::builder()
            .desc("Close location list")
            .silent(true)
            .build(),
    );

    // Cycle through the quick fix list and center the current result line
    let _ = set_keymap(
        Mode::Normal,
        "<c-j>",
        ":cnext<CR>zz",
        &SetKeymapOpts::builder().silent(true).build(),
    );
    let _ = set_keymap(
        Mode::Normal,
        "<c-k>",
        ":cNext<CR>zz",
        &SetKeymapOpts::builder().silent(true).build(),
    );

    // ------------------------------------------------------------------------------------
    // <leader>sc: Toggle spell checking
    // ------------------------------------------------------------------------------------
    let _ = set_keymap(
        Mode::Normal,
        "<leader>sc",
        "",
        &SetKeymapOpts::builder()
            .desc("Toggle spell check")
            .callback(|_| {
                let current_win = Window::current();
                let opts = OptionOpts::builder().win(current_win).build();
                let toggled_value = !get_option_value::<bool>("spell", &opts).unwrap();
                let _ = set_option_value("spell", toggled_value, &opts);
                ()
            })
            .build(),
    );

    // ------------------------------------------------------------------------------------
    // Handy book marks
    // ------------------------------------------------------------------------------------

    // `mm`: Make a gloabl mark
    let _ = set_keymap(
        Mode::Normal,
        "mm",
        "mM",
        &SetKeymapOpts::builder()
            .desc("Mark current position")
            .silent(true)
            .build(),
    );
    // `gb`: Go back to the global mark
    let _ = set_keymap(
        Mode::Normal,
        "gb",
        "`Mzz",
        &SetKeymapOpts::builder()
            .desc("Go back to last marked position")
            .silent(true)
            .build(),
    );

    // ------------------------------------------------------------------------------------
    // Tab related
    // ------------------------------------------------------------------------------------
    let _ = set_keymap(
        Mode::Normal,
        "<leader>to",
        "<cmd>tabnew<CR>",
        &SetKeymapOpts::builder()
            .desc("Tab: open new tab")
            .silent(true)
            .build(),
    );
    let _ = set_keymap(
        Mode::Normal,
        "<leader>tc",
        "<cmd>tabclose<CR>",
        &SetKeymapOpts::builder()
            .desc("Tab: close current tab")
            .silent(true)
            .build(),
    );
    let _ = set_keymap(
        Mode::Normal,
        "<leader>tn",
        "<cmd>tabn<CR>",
        &SetKeymapOpts::builder()
            .desc("Tab: go to next tab")
            .silent(true)
            .build(),
    );
    let _ = set_keymap(
        Mode::Normal,
        "<leader>tp",
        "<cmd>tabp<CR>",
        &SetKeymapOpts::builder()
            .desc("Tab: go to prev tab")
            .silent(true)
            .build(),
    );
    let _ = set_keymap(
        Mode::Normal,
        "<leader>tb",
        "<cmd>tabnew %<CR>",
        &SetKeymapOpts::builder()
            .desc("Tab: open current buffer in new tab")
            .silent(true)
            .build(),
    );

    // ------------------------------------------------------------------------------------
    // Terminal related
    // ------------------------------------------------------------------------------------
    let _ = set_keymap(
        Mode::Normal,
        "<leader>ot",
        ":vsplit<CR>:terminal<CR>",
        &SetKeymapOpts::builder()
            .desc("Terminal: open terminal")
            .silent(true)
            .build(),
    );

    let _ = set_keymap(
        Mode::Terminal,
        "<ESC>",
        "<C-\\><C-n>",
        &SetKeymapOpts::builder()
            .desc("Terminal: Press `<ESC>` to back to normal mode")
            .silent(true)
            .build(),
    );

    let _ = set_keymap(
        Mode::Terminal,
        "<C-h>",
        "<C-\\><C-n><C-w>h",
        &SetKeymapOpts::builder()
            .desc("Terminal: Press `<C-h>` to go back to the left window")
            .silent(true)
            .build(),
    );

    let _ = set_keymap(
        Mode::Normal,
        "<leader>ft",
        "",
        &SetKeymapOpts::builder()
            .desc("Terminal: open a floating terminal")
            .silent(true)
            .callback(|_| {
                open_centred_floating_terminal_window();
                ();
            })
            .build(),
    );

    // ------------------------------------------------------------------------------------
    // Command line related
    // ------------------------------------------------------------------------------------
    let _ = set_keymap(
        Mode::CmdLine,
        "<C-j>",
        "<Down>",
        &SetKeymapOpts::builder()
            .desc("Next history command")
            .build(),
    );

    let _ = set_keymap(
        Mode::CmdLine,
        "<C-k>",
        "<Up>",
        &SetKeymapOpts::builder()
            .desc("Previous history command")
            .build(),
    );

    // ------------------------------------------------------------------------------------
    // Evacuate/run the selected lua code
    // ------------------------------------------------------------------------------------
    let _ = set_keymap(
        Mode::VisualSelect,
        "<leader>ee",
        ":'<,'>lua<CR>",
        &SetKeymapOpts::builder()
            .desc("Evaluate selected lua code")
            .silent(true)
            .build(),
    );

    // ------------------------------------------------------------------------------------
    // 'my_utils' plugin lazy load
    // ------------------------------------------------------------------------------------
    let _ = set_keymap(
        Mode::Normal,
        "<leader>1",
        "",
        &SetKeymapOpts::builder()
            .desc("Kill other windows.")
            .silent(true)
            .callback(|_|{
                kill_other_windows();
                ()
            })
            .build(),
    );

    // // ------------------------------------------------------------------------------------
    // // 'my_project_command' plugin lazy load
    // // ------------------------------------------------------------------------------------
    // local run_project_command_plugin = function()
    //     -- require('my_project_command_telescope').run({
    //     --     open_source_on_left_split_win = true,
    //     --     -- enable_script_files = true,
    //     -- })

    //     require('my_project_command_fzf_lua').run({
    //         open_source_on_left_split_win = true,
    //         enable_script_files = true,
    //     })
    // end

    // let _ = set_keymap('n',
    //     '<leader>pc',
    //     run_project_command_plugin,
    //     {
    //         silent = true,
    //         desc = "Project command"
    //     }
    // )
}
