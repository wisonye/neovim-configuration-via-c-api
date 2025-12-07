///
///
///
pub fn setup() {
    // -----------------------------------------------------------------------------------
    // Change `.sh` filetype to `fish`
    // -----------------------------------------------------------------------------------
    let _ = create_autocmd(
        // Event list
        vec!["FileType"],
        // Auto command options
        &CreateAutocmdOpts::builder()
            .patterns(vec!["sh"])
            .group(
                create_augroup(
                    "custom-sh-to-fish-group",
                    &CreateAugroupOpts::builder().clear(true).build(),
                )
                .unwrap(),
            )
            .callback(|_| {
                let _ = set_option_value(
                    "filetype",
                    "fish",
                    &OptionOpts::builder().buffer(Buffer::current()).build(),
                );

                #[cfg(feature = "enable_auto_groups_debug_print")]
                nvim::print!("\n>>> run auto command: set 'fish' filetype for '.sh' file.");

                //
                // Return `true` to delete the autocommand (means only run once)!!!
                //
                false
            })
            .build(),
    );

    // -----------------------------------------------------------------------------------
    // Enable highlight when yanked, `TextYankPost` event auto command
    // -----------------------------------------------------------------------------------
    let _ = create_autocmd(
        // Event list
        vec!["TextYankPost"],
        // Auto command options
        &CreateAutocmdOpts::builder()
            .group(
                create_augroup(
                    "custom-yank-group",
                    &CreateAugroupOpts::builder().clear(true).build(),
                )
                .unwrap(),
            )
            .callback(|_| {
                let _ = call_function::<_, String>(
                    "luaeval",
                    (r#"vim.highlight.on_yank { higroup='IncSearch', timeout=300 }"#,),
                );

                //
                // Return `true` to delete the autocommand (means only run once)!!!
                //
                false
            })
            .build(),
    );

    // -----------------------------------------------------------------------------------
    // Change the default keybindings for the "help doc buffer" and try to keep
    // them consistent with the LSP default keybindings:
    //
    // "Map 'gd' to '<C-]>': Jump to the definition of the keyword under the cursor.
    // Same as ':tag {name}', where {name} is the keyword under or after cursor.
    // -----------------------------------------------------------------------------------
    let _ = create_autocmd(
        // Event list
        vec!["FileType"],
        // Auto command options
        &CreateAutocmdOpts::builder()
            .group(
                create_augroup(
                    "custom-help-tag-group",
                    &CreateAugroupOpts::builder().clear(true).build(),
                )
                .unwrap(),
            )
            .callback(|_| {
                let mut current_buffer = Buffer::current();
                let buffer_file_type = get_option_value::<NvimString>(
                    "filetype",
                    &OptionOpts::builder().buffer(current_buffer.clone()).build(),
                );

                if let Ok(b_filetype) = buffer_file_type {
                    if b_filetype == "help" {
                        let _ = current_buffer.set_keymap(
                            Mode::Normal,
                            "gd",
                            "<C-]>",
                            &SetKeymapOpts::builder().silent(true).build(),
                        );
                    }
                }

                //
                // Return `true` to delete the autocommand (means only run once)!!!
                //
                false
            })
            .build(),
    );
}

#[cfg(feature = "enable_auto_groups_debug_print")]
use nvim_oxi as nvim;

use nvim_oxi::{
    String as NvimString,
    api::{
        Buffer, call_function, create_augroup, create_autocmd, get_option_value,
        opts::{CreateAugroupOpts, CreateAutocmdOpts, OptionOpts, SetKeymapOpts},
        set_option_value,
        types::Mode,
    },
};
