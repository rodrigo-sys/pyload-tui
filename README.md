# pyload-tui

[features](#features) • [installation](#installation) • [prerequisites](#prerequisites) • [usage](#usage) • [bindings](#bindings) • [build from source](#build-from-source)

---

Terminal UI client for pyLoad.

## Features

- Vim-style navigation keys
- Browse files and packages
- Add packages
- Append links to existing packages
- Delete packages or individual files
- Changes update in real time via API event polling

## Installation

```bash
cargo install --git https://github.com/rodrigo-sys/pyload-tui.git
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

| Key | Action |
|---|---|
| `j` / `k` | Move down / up |
| `l` | Open package |
| `h` | Go back (file list) |
| `q` | Quit |
| `d` | Delete selected |
| `A` | Add new package |
| `a` | Add links to selected package |
| `Esc` | Go back (forms) |
| `Tab` / `Shift+Tab` | Cycle form fields |
| `Enter` | Toggle / submit (forms) |

## Build from source

```bash
git clone https://github.com/rodrigo-sys/pyload-tui.git
cd pyload-tui
cargo build --release # target/release/pyload-tui
```
