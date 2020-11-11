# husb

Simple Rust based CLI for managing usb devices.
Uses the HidApi for portable USB access and the ioctl family to reset commands.


The [nix-shell](https://nixos.org/manual/nix/stable/#chap-installation) assumes you have the [Mozilla Rust Overlay](https://github.com/mozilla/nixpkgs-mozilla#rust-overlay) configured...

```shell
$ nix-shell
$ cargo run list
```
