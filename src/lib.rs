///
/// `#[nvim_oxi::plugin]` marks this function as the entrypoint of the plugin.
///
/// This function will be called by Neovim when the user loads the plugin by
/// passing its name to the `require` function. It can return any type that
/// implements the `Pushable` trait, and the value will be returned on the
/// `Lua` side by require.
///
/// The `Pushable` trait located in `nvim-oxi/crates/luajit/src/pushable.rs`:
///
/// ```rust
/// /// Trait implemented for types that can be pushed onto the Lua stack.
/// pub trait Pushable {
///     /// Pushes all its values on the Lua stack, returning the number of values
///     /// that it pushed.
///     unsafe fn push(self, lstate: *mut State) -> Result<c_int, crate::Error>;
/// }
///
/// impl Pushable for bool {
///     unsafe fn push(self, lstate: *mut State) -> Result<c_int, crate::Error> {
///         ffi::lua_pushboolean(lstate, self as _);
///         Ok(1)
///     }
/// }
/// ```
///
/// [ Very important!!! ]
///
/// This function name MUST be the same with your plugin name, as the compiled library has
/// the symbol name: `_luaopen_YOUR_PLUGIN_ENTRYPOINT_FUNCTION_NAME`!!!
///
/// For example:
///
/// 1. This plugin entrypoint function name: my_neovim_configuration
///
/// 2. You need to copy/move the library file to your neovim configuration folder with the same name:
///
///    ```fish
///    mv target/release/libmy_neovim_configuration.dylib ~/.config/nvim/lua/my_neovim_configuration.so
///
///    # You can list the `luaopen` symbol name to confirm.
///    nm ~/.config/nvim/lua/my_neovim_configuration.so | rg luaopen
///    0000000000001080 T _luaopen_my_neovim_configuration
///    ```
///
/// 3. Load your plugin in Lua: require('my_neovim_configuration')
///
#[nvim_oxi::plugin]
fn my_neovim_configuration() -> bool {
    settings::setup();
    keybindings::setup();
    my_dired::setup();
    auto_groups::setup();
    // picker::setup();
    project_command::setup();

    #[cfg(feature = "enables_support_missing_apis")]
    {
        let lua_body_string = 
        // r#"
        // local a, b = ...
        // vim.print('>>> exec_lua param: a:'.. a .. ', b: ' ..b)
        // return ('result: %s'):format(a + b)
        // "#
        r#"
            local lsp_enable_result = vim.lsp.enable("clangd")
            -- vim.print("\n>>> lsp_enable_result: " .. lsp_enable_result)
            return lsp_enable_result
        "#;

        let execute_result = nvim::api::exec_lua::<String>(
            lua_body_string,
            vec!["".to_string()]
        );
        nvim::print!(">>> execute_result: {execute_result:#?}");
    }

    #[cfg(feature = "enable_plugin_debug_print")]
    nvim::print!("\n>>> My Neovim Configuration has loaded successfully.");

    true
}

#[cfg(feature = "enable_plugin_debug_print")]
use nvim_oxi as nvim;

mod auto_groups;
mod keybindings;
mod my_dired;
mod picker;
mod project_command;
mod settings;
mod utils;

#[cfg(feature = "enables_support_missing_apis")]
mod extended_api;
