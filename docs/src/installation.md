# Installation

The command `linlog` is the package `linlog-cli`. Each release on
[GitHub](https://github.com/sequentium/sequentium/releases) has it built
for Linux, macOS and Windows, with packages for Debian and Ubuntu,
Fedora and Arch Linux; it is on [crates.io](https://crates.io/crates/linlog-cli)
for `cargo install`, and the flake of this repository builds it with
Nix. Pick your system:

| system | install | needs |
|---|---|---|
| Debian, Ubuntu | the release's `.deb` | nothing |
| Fedora | the release's `.rpm` | nothing |
| Arch Linux | the release's package | nothing |
| any Linux | the release's archive, one static binary | nothing |
| NixOS, Nix | the flake | flakes enabled |
| macOS | the release's archive | nothing |
| Windows | the release's `.zip` | nothing |
| any system | `cargo install --locked linlog-cli` | Rust 1.92 or later |

The files below are those of version 0.2.0; a later release names its
own.

**Which to prefer.** The release's Linux files hold one binary linked
statically against musl, so that it runs on every distribution. musl's
C library is slower than glibc's at what a search does most: on two
searches of about four seconds (cores of one machine, 2026-11-02) the
static binary took 1.26 to 1.45 times as long as a build against glibc,
on one thread and on four alike. For long searches, build the command
on your machine, with `cargo install` or Nix, which link glibc; for
everything else the packages are the quicker way in. Every release lists the SHA-256 sums of its files in `SHA256SUMS`
and carries an attestation of where they were built, which
`gh attestation verify FILE --repo sequentium/sequentium` checks with the
GitHub command; the files of 0.2.0 and earlier were attested under the
repository's former name, so for them it is
`gh attestation verify FILE --repo linlog-prover/linlog`. The packages and binaries are not signed: what that
means on each system is said in its section.

## Debian and Ubuntu

```sh
curl -LO https://github.com/sequentium/sequentium/releases/download/v0.2.0/linlog_0.2.0-1_amd64.deb
sudo apt install ./linlog_0.2.0-1_amd64.deb
linlog --version
```

`arm64` instead of `amd64` on an ARM machine. The package holds the
binary, `/usr/bin/linlog`, and the licences under
`/usr/share/doc/linlog`; it depends on nothing. To update, install the
next release's `.deb` the same way; to remove, `sudo apt remove linlog`.

## Fedora

```sh
sudo dnf install https://github.com/sequentium/sequentium/releases/download/v0.2.0/linlog-0.2.0-1.x86_64.rpm
linlog --version
```

`aarch64` instead of `x86_64` on an ARM machine. The package is not
signed, which dnf accepts for a package named on its command line. To
update, install the next release's `.rpm`; to remove, `sudo dnf remove
linlog`.

## Arch Linux

```sh
curl -LO https://github.com/sequentium/sequentium/releases/download/v0.2.0/linlog-0.2.0-1-x86_64.pkg.tar.zst
sudo pacman -U ./linlog-0.2.0-1-x86_64.pkg.tar.zst
linlog --version
```

The package is for x86_64; on Arch Linux ARM, take the archive below.
It is not signed, so pacman, which checks the signatures of packages from its
repositories, asks nothing for a file. To update, install the next
release's package; to remove, `sudo pacman -R linlog`.

## Any Linux

The archive holds one statically linked binary, which runs on any
distribution, with README and the licences:

```sh
curl -LO https://github.com/sequentium/sequentium/releases/download/v0.2.0/linlog-0.2.0-x86_64-unknown-linux-musl.tar.gz
curl -LO https://github.com/sequentium/sequentium/releases/download/v0.2.0/SHA256SUMS
sha256sum --check --ignore-missing SHA256SUMS
tar -xzf linlog-0.2.0-x86_64-unknown-linux-musl.tar.gz
install -D linlog-0.2.0-x86_64-unknown-linux-musl/linlog ~/.local/bin/linlog
linlog --version
```

`aarch64` instead of `x86_64` on an ARM machine; `~/.local/bin` is on
the path of most distributions' shells. To update, do the same with the
next release; to remove, `rm ~/.local/bin/linlog`.

## NixOS and Nix

The flake's default package is the command. Flakes are on with
Determinate Systems' installer; with the official installer, add
`experimental-features = nix-command flakes` to `~/.config/nix/nix.conf`
once.

```sh
nix run github:sequentium/sequentium -- --help           # run it once, without installing
nix profile add github:sequentium/sequentium/v0.2.0      # install a release into your profile
nix profile remove linlog                                # and to remove it
```

An entry added by a release's tag stays that release: to update,
remove it and add the next release's tag. One added without a tag
(`github:sequentium/sequentium`, the `main` branch, which may be ahead
of the last release) follows it with `nix profile upgrade linlog`.
On Nix before 2.30, `nix profile add` is `nix profile install`. The
package carries the EUPL and the embedded fonts' licences under
`share/doc/linlog-cli/`.

In a flake of your own, a NixOS or Home Manager configuration say, take
it as an input:

```nix
{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.linlog.url = "github:sequentium/sequentium/v0.2.0";

  outputs = { nixpkgs, linlog, ... }: {
    nixosConfigurations.HOST = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        ({ pkgs, ... }: {
          environment.systemPackages = [
            linlog.packages.${pkgs.stdenv.hostPlatform.system}.default
          ];
        })
      ];
    };
  };
}
```

(`home.packages` takes it the same way.) To update, move the input's tag
and rebuild; to remove, take it out of the list. Nix builds it from
source on first use, a few minutes; the binary cache under
[Development](development.md) holds the builds of `main` for
x86_64-linux and saves that, once it is trusted.

## macOS

```sh
curl -LO https://github.com/sequentium/sequentium/releases/download/v0.2.0/linlog-0.2.0-aarch64-apple-darwin.tar.gz
curl -LO https://github.com/sequentium/sequentium/releases/download/v0.2.0/SHA256SUMS
shasum -a 256 --check --ignore-missing SHA256SUMS
tar -xzf linlog-0.2.0-aarch64-apple-darwin.tar.gz
mkdir -p ~/.local/bin
cp linlog-0.2.0-aarch64-apple-darwin/linlog ~/.local/bin/
~/.local/bin/linlog --version
```

`x86_64` instead of `aarch64` on an Intel Mac; add `~/.local/bin` to
your shell's path (`echo 'export PATH="$HOME/.local/bin:$PATH"' >>
~/.zprofile`). The binary is not signed by an Apple developer. Fetched
with `curl` as above it runs; fetched with a browser, macOS marks it and
refuses to open it the first time ("cannot be opened because Apple
cannot check it for malicious software"): remove the mark with `xattr
-d com.apple.quarantine ~/.local/bin/linlog`, or allow it once under
System Settings, Privacy & Security, "Open Anyway". To update, do the
same with the next release; to remove, `rm ~/.local/bin/linlog`.

## Windows

In PowerShell:

```powershell
Invoke-WebRequest -OutFile linlog.zip https://github.com/sequentium/sequentium/releases/download/v0.2.0/linlog-0.2.0-x86_64-pc-windows-msvc.zip
(Get-FileHash linlog.zip -Algorithm SHA256).Hash    # compare with the line of SHA256SUMS
Expand-Archive linlog.zip -DestinationPath "$env:LOCALAPPDATA\Programs"
Rename-Item "$env:LOCALAPPDATA\Programs\linlog-0.2.0-x86_64-pc-windows-msvc" linlog
$bin = "$env:LOCALAPPDATA\Programs\linlog"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$bin", "User")
& "$bin\linlog.exe" --version
```

`aarch64` instead of `x86_64` on an ARM machine. A new terminal finds
`linlog` on its path. The program is not signed: if Windows shows
"Windows protected your PC" when it first starts, choose More info, then
Run anyway, or clear the download's mark beforehand with `Unblock-File
"$bin\linlog.exe"`. To update, replace the folder with the next
release's; to remove, delete it and its entry of the user's `Path`
(Settings, System, About, Advanced system settings, Environment
Variables).

## Any system, with Rust

`cargo install` builds the command from crates.io:

```sh
cargo install --locked linlog-cli
cargo install --locked linlog-cli      # later: the newest release
cargo uninstall linlog-cli             # and to remove it
```

It needs Rust 1.92 or later (a release raises that only with its second
number, 0.2 to 0.3) and a C compiler, which links the program. Not every
distribution's own Rust reaches 1.92: Fedora's (`sudo dnf install cargo
gcc`), Arch's (`sudo pacman -S rust gcc`), Ubuntu 26.04's (`sudo apt
install cargo build-essential`) and Homebrew's (`brew install rust`) do;
Debian 13's (1.85; 1.96 in `trixie-backports`) and Ubuntu 24.04's
(1.75) do not. Elsewhere, install Rust with
[rustup](https://rustup.rs), after the C compiler (`sudo apt install
build-essential` on Debian and Ubuntu, `xcode-select --install` on
macOS): on Linux and macOS

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

and on Windows `rustup-init.exe` from [rustup.rs](https://rustup.rs),
which needs Microsoft's C++ build tools (Visual Studio's "Desktop
development with C++", which rustup-init offers to install). `rustup
update` keeps it current and `rustup self uninstall` removes it.

## The library

The library is the crate [`linlog`](https://crates.io/crates/linlog):

```toml
[dependencies]
linlog = "0.2"
```

with the default features `parse`, `serialize`, `interactive`, `latex`,
`typst`, `svg` and `rocq`, and `parallel` (a thread pool, not for wasm),
`png` and `pdf` to add. The documentation's examples read and write JSON
with `serde_json`, the caller's own dependency.

## The Rocq library

The Rocq certificates the command writes (`--format rocq`) are checked
by the NanoYalla kernel or by linlog's own Rocq library, `rocq-linlog`,
in its repository
[sequentium/rocq-linlog](https://github.com/sequentium/rocq-linlog);
[Rocq certificates](rocq.md) says how to install each and compile a
certificate.
