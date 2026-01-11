#!/usr/bin/env fish

set -l OS_TYPE (uname -s)
set -l DEBUG_BUILD_OUTPUT "target/debug/libmy_neovim_configuration"
set -l NEOVIM_PLUGIN "$HOME/.config/nvim/lua/my_neovim_configuration.so"

if test (string upper "$OS_TYPE") != "DARWIN"
    set DEBUG_BUILD_OUTPUT "$DEBUG_BUILD_OUTPUT.so"
else
    set DEBUG_BUILD_OUTPUT "$DEBUG_BUILD_OUTPUT.dylib"
end

cargo build --features enable_plugin_debug_print,enables_support_missing_apis --color=never && mv $DEBUG_BUILD_OUTPUT $NEOVIM_PLUGIN
