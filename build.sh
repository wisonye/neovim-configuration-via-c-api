#!/usr/bin/env fish
cargo build --release && \
    mv target/release/libmy_neovim_configuration.dylib ~/.config/nvim/lua/my_neovim_configuration.so
