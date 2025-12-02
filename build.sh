#!/usr/bin/env fish

set -l OS_TYPE (uname -s)
set -l RELEASE_BUILD_OUTPUT "target/release/libmy_neovim_configuration"
set -l NEOVIM_PLUGIN "$HOME/.config/nvim/lua/my_neovim_configuration.so"

if test (string upper "$OS_TYPE") != "DARWIN"
    set RELEASE_BUILD_OUTPUT "$RELEASE_BUILD_OUTPUT.so"
else
    set RELEASE_BUILD_OUTPUT "$RELEASE_BUILD_OUTPUT.dylib"
end

cargo build --release --color=never && mv $RELEASE_BUILD_OUTPUT $NEOVIM_PLUGIN
