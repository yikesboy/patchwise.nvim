# patchwise.nvim

a rust native neovim plugin to use llms in a reasonable way and to prevent slop from entering your repo

## development

enter the development requirement

`nix develop`

check if all main components are available

`cargo xtask health`

build the plugin and launch a clean and isolated neovim instance

`cargo xtask testvim`

open a file with the test instance

`cargo xtask testvim path/to/file`

pass arbitrary neovim arguments to the test instance

`cargo xtask testvim --<argument>`

cleanup testvim build artifcats

`cargo xtask clean`

## useful rust commands

check compilation

`cargo check`

build the plugin

`cargo build --package patchwise`

run tests

`cargo test`

format

`cargo format`

## useful nix commands

format nix files

`nix fmt`

check the flake

`nix flake check`

build the packaged plugin

`nix build`

update flake inputs

`nix flake update`

## useful neovim commands

check if plugin is loaded

`:lua print(package.loaded.patchwise ~= nil)`
