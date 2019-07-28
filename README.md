# Rust GTK+ Counter App

> The world's most useless application now natively for your Gnome desktop.

This is a code for my [blog post](https://turbomack.github.io/posts/2019-07-28-rust-vs-gui.html).

## Bulding

On Linux or Max with [the Nix package manager](https://nixos.org/nix/).

to build and run

```bash
# build
$ nix-build

# run
$ ./result/bin/rust-gtk-counter
```

or install via nix-env:

```bash
$ nix-env -if .
```

## Development Environment

You can also use `nix-shell` to access the shell with all the native dependecies.

```bash
# enter shell
$ nix-shell

# compile and run via cargo
[nix-shell] cargo run
```
