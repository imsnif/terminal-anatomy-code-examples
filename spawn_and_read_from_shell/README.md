This program is one of the examples from the [Anatomy of a Terminal Emulator](https://www.poor.dev/blog/terminal-anatomy/) blog post.

It spawns a shell on the secondary side of a PTY, reads its output and displays it on screen in an escaped fashion so that the terminal emulator running it will not interpret it.

To run:

`cargo run`
