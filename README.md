# Pinball Space Cadet

## Introduction

This is an attempt to provide a rust implementation of Space Cadet pinball.

## Install dependencies

You'll have to install some dependencies to run this application.

### On all systems

Copy the original space cadet files (like `PINBALL.DAT`) into the `src/data` directory.

### On MacOS

#### Using Homebrew

`brew install sdl2 sdl2_image sdl2_sound sdl2_gfx sdl2_mixer sdl2_ttf`

## Compilation

### On MacOS

#### Using Homebrew

`LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib" cargo run`

## Links

- https://github.com/k4zmu2a/SpaceCadetPinball
