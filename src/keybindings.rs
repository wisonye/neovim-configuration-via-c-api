use nvim_oxi::{self as nvim, api::opts::OptionScope};

use nvim::api::{
    Buffer, Window, cmd as vim_cmd, get_option_value, open_win,
    opts::{CmdOpts, OptionOpts, SetKeymapOpts},
    set_keymap, set_option_value, set_var,
    types::{CmdInfos, Mode, WindowBorder, WindowConfig, WindowRelativeTo},
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
                // Get terminal width and height
                let opts = OptionOpts::builder().scope(OptionScope::Local).build();
                let terminal_width = get_option_value::<u32>("columns", &opts).unwrap();
                let terminal_height = get_option_value::<u32>("lines", &opts).unwrap();

                // Calculate popup window width and height by ratio
                let default_width_ratio = 0.8f32;
                let default_height_ratio = 0.7f32;
                let width = ((terminal_width as f32) * default_width_ratio).floor();
                let height = ((terminal_height as f32) * default_height_ratio).floor();

                // Center window in `editor` area by calculating the (left, top)
                let cols = (((terminal_width as f32 - width) / 2f32).floor()) as u32;
                let rows = (((terminal_height as f32 - height) / 2f32).floor()) as u32;

                // // Debug print
                // nvim::print!(
                //     concat!(
                //         "\n>>> Terminal size: {{",
                //         "\n\t terminal_width: {}",
                //         "\n\t terminal_height: {}",
                //         "\n\t popup_width: {}",
                //         "\n\t popup_height: {}",
                //         "\n\t popup_left: {}",
                //         "\n\t popup_right: {}",
                //         "\n}}"
                //     ),
                //     terminal_width,
                //     terminal_height,
                //     width,
                //     height,
                //     cols,
                //     rows,
                // );

                // Open popup window with current buffer
                let enter_into_window = true;
                let open_win_config = WindowConfig::builder()
                    .relative(WindowRelativeTo::Editor)
                    .width(width as u32)
                    .height(height as u32)
                    .row(rows)
                    .col(cols)
                    .border(WindowBorder::Rounded)
                    .build();
                let _ = open_win(&Buffer::current(), enter_into_window, &open_win_config);

                // Run the `:terminal` command inside the popup window's buffer
                let command = "terminal";
                let infos = CmdInfos::builder().cmd(command).build();
                let opts = CmdOpts::builder().output(false).build();
                let _ = vim_cmd(&infos, &opts);

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

    // // ------------------------------------------------------------------------------------
    // // 'my_utils' plugin lazy load
    // // ------------------------------------------------------------------------------------
    // let _ = set_keymap(
    //     "n",
    //     "<leader>1",
    //     "<cmd>lua require('my_utils').kill_other_windows()<CR>",
    //     {
    //         silent = true,
    //         desc = "Kill other windows"
    //     }
    // )

    // // ------------------------------------------------------------------------------------
    // // 'my_dired' plugin lazy load
    // // ------------------------------------------------------------------------------------
    // let _ = set_keymap(
    //     "n",
    //     "<C-c>j",
    //     "<cmd>lua require('my_dired').open()<CR>",
    //     {
    //         silent = true,
    //         desc = "Open my dired with current buffer directory"
    //     }
    // )

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
