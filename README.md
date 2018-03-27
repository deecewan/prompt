# prompt

> My terminal prompt

This is a personal terminal prompt written in rust. It is based rather heavily
on [pure](https://github.com/sindresorhus/pure), as that was my go-to
previously.

The goals of this project:

- [ ] fast (still need to confirm, but feels fast)
- [x] Vi-keys status

I moved off pure because I found it was slowing down quite a lot in large
repositories. I found an alternative, [purs](https://github.com/xcambar/purs),
which provided some inspiration for this. I wanted a different vi-keys display,
and I found the speed dropping in my main work codebase.

So, `prompt`. It's super-fast, but I haven't played with it day-to-day yet. It
almost certainly has some side-effects and unintended consequences. This is not
meant to be a 'supports all workflows' tool. It supports my workflow, and
supports it as quick as possible, while still looking good.

If you want to add/remove/change segments, fork the codebase, and keep your own
prompt.

## Installation

- clone the repo (the location is `$CODE` from now on)
- `cd $CODE`
- `cargo build --release`
- open your `~/.zshrc`
- add the following:

```zsh
function zle-line-init zle-keymap-select {
  PROMPT=$($CODE/target/release/prompt $? "$KEYMAP")
  zle reset-prompt
}

zle -N zle-line-init
zle -N zle-keymap-select
```

- if you don't want to display the vi-keys status, simply remove the `$KEYMAP`
  at the end of the command.
- `source ~/.zshrc` (you may want to open a new terminal window if you had
  another theme before)
- That's it
