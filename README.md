# pyload-tui

[features](#features) • [installation](#installation) • [prerequisites](#prerequisites) • [usage](#usage) • [bindings](#bindings) • [faq](#faq) • [build](#build-from-source)

Terminal UI client for pyLoad.
<img width="1200" height="700" alt="demo" src="https://github.com/user-attachments/assets/cf229eb1-e04a-4d87-acef-361c7f8250e5" />

## Features

- Vim-style navigation keys
- Browse files and packages
- Add packages
- Append links to existing packages
- Delete packages or individual files
- Pause, unpause, and toggle download queue
- Abort active downloads
- Restart failed, finished, or selected files
- Reorder packages and files
- Move packages between collector and queue
- Changes update in real time via API event polling

## Installation

```bash
cargo install --git https://github.com/rodrigo-sys/pyload-tui.git
```

or with [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) (precompiled):

```bash
cargo binstall --git 'https://github.com/rodrigo-sys/pyload-tui' pyload-tui
```

## Prerequisites

<details>
<summary>Install pyLoad</summary>

With [pipx](https://pipx.pypa.io/):

```bash
pipx install "pyload-ng[all]" --pip-args="--pre" --force
```

Or with your OS package manager.

</details>
<details>
<summary>Generate an API key</summary>

From the web interface (http://localhost:8000/):  
**Settings** → **Users** → **Actions** → **Manage API Keys** → **Generate**.  
Enter your password and a name for the key.  
Save it — you can't see it again.

</details>
<details>
<summary>Edit the config file</summary>

Located at:

| Platform | Path |
|---|---|
| Linux | `~/.config/pyload-tui/config.kdl` |
| macOS | `~/Library/Application Support/pyload-tui/config.kdl` |
| Windows | `%APPDATA%/pyload-tui/config.kdl` |

Replace the placeholders with your own values.

```kdl
pyload-url "http://localhost:8000/"
api-key "YOUR_API_KEY"
```

</details>

## Usage

Run `pyload-tui` in your terminal.

### Bindings

#### Main

| Key | Action |
|---|---|
| `h`, `j`, `k`, `l` | navigate |
| `q` | Quit |
| `A` | Add new package |
| `a` | Add links to package |
| `d` | Delete selected item |
| `J` / `K` | Reorder down / up |
| `r` | Restart selected item |


#### Packages screen

| Key | Action |
|---|---|
| `m` | Move between collector / queue |


#### Files screen

| Key | Action |
|---|---|
| `s` | Stop file download |

#### General

| Key | Action |
|---|---|
| `S` | Abort active downloads |
| `P` | Pause queue |
| `U` | Unpause queue |
| `T` | Toggle queue state |
| `X` | Abort downloads + pause queue / restart failed + unpause queue |
| `R` | Restart all failed files |

#### Forms

| Key | Action |
|---|---|
| `Esc` | Go back |
| `Tab` / `Shift+Tab` | Cycle form fields |
| `Enter` | Toggle / submit |


## FAQ

<details>
<summary>FAQ</summary>
<br>
<details>
<summary>How stop all downloads?</summary>

There are _active downloads_ and _packages on the queue_.

If you want to stop everything you need to:
- Pause the queue with `P`
- Abort active downloads with `S`

`X` do those both things at once:
  - first press: pause + abort
  - second press:
    resets files with `ABORTED` status +
    unpaused queue
</details>

<details>
<summary>How to download files with ABORTED status?</summary>

When you stop a download the status of the file changes to `ABORTED`.

To reset it you can:
- press `r` with the aborted file selected
- or press `R` anywhere to reset all aborted files at once

reset means set status to `QUEUED`

Pressing `r` on a package will reset every file in that package,
including `FINISHED` ones, meaning they will be re-downloaded.
</details>

<details>
<summary>What's queue?</summary>

The **queue** holds packages that are waiting to be downloaded.

pyLoad processes them in order, starting from the top.

You can reorder them with `J` / `K`.
</details>

<details>
<summary>What's collector?</summary>

The **collector** is a staging area for packages you don't want to download yet.

Move packages between collector/queue with `m` on the selected package.
</details>

<details>
<summary>What's a package?</summary>

A **package** is a container for files (download links) inside pyLoad.

When you add a package, it goes to either the **collector** or the **queue** depending on how you set it up.

Each package can have multiple links inside it.
</details>

</details>

## Build from source

```bash
git clone https://github.com/rodrigo-sys/pyload-tui.git
cd pyload-tui
cargo build --release # target/release/pyload-tui
```
