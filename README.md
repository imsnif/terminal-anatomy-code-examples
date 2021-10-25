This repository contains examples from the [Anatomy of a Terminal Emulator](https://www.poor.dev/blog/terminal-anatomy/) blog post.

Each folder contains a separate example and can be run separately according to its README.

1. `spawn_and_read_from_shell` - contains a program that spawns a new shell through a PTY, reads output from it and displays it escaped on screen.
2. `raw_castle_ansi` - contains the ANSI art for the castle animation in the post, to allow anyone to `cat` it into their own terminal and play around with it.
3. `responsive_terminal_ui` - contains a program that draws a different UI depending on the current terminal size, updating itself as the window is resized.
4. `terminal_raw_mode` - displays a use case for using the terminal `raw mode` feature to receive user input one character at a time.
