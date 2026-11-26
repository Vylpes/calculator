# Calculator (Vyxcalc)

A desktop calculator application written in Rust with both CLI and GUI modes.

## Features

- **CLI Mode**: Command-line interface with direct expression evaluation and interactive mode
- **Session History**: In interactive CLI mode, use Up/Down to recall expressions from the current session
- **GUI Mode**: Graphical calculator interface built with GTK4
- **Basic Operations**: Addition (+), Subtraction (-), Multiplication (*), Division (/)
- **Error Handling**: Proper error messages for invalid input and division by zero

## Building

Make sure you have Rust installed and the necessary system dependencies for GTK4:

### On Ubuntu/Debian:
```bash
sudo apt install libgtk-4-dev libglib2.0-dev
```

### Build the application:
```bash
cargo build --release
```

## Usage

### CLI Mode

#### Direct Expression Evaluation
```bash
./target/release/vyxcalc cli "2 + 3"
# Output: 5

./target/release/vyxcalc cli "10 / 2"
# Output: 5

./target/release/vyxcalc cli "5 * 7"
# Output: 35
```

#### Interactive Mode
```bash
./target/release/vyxcalc cli
```
This will start an interactive session where you can enter multiple expressions:
```
Calculator CLI - Interactive Mode
Enter expressions like '2 + 3' or 'quit' to exit
Use Up/Down arrow keys to cycle through this session's history
> 2 + 3
= 5
> 10 - 4
= 6
> quit
Goodbye!
```

In interactive mode, Up and Down cycle through expressions from the current session so you can edit and re-run them (including fixing invalid input). History is not kept after the session ends. Empty lines are not stored.

### GUI Mode

```bash
./target/release/vyxcalc gui
# or simply
./target/release/vyxcalc
```

This opens a graphical calculator window with:
- Number buttons (0-9)
- Operation buttons (+, -, *, /)
- Equals button (=)
- Clear button (C)
- Decimal point button (.)

## Expression Format

Expressions are chains of `number operator number ...`. Whitespace is optional
(`2+3` and `2 + 3` are equivalent). Multiple operations are evaluated
**left to right** (no operator precedence).

Examples:
- `2 + 3` / `2+3`
- `10.5 - 4.2`
- `7 * 8`
- `15 / 3`
- `5 + 2 * 6` → `42` (evaluated as `(5 + 2) * 6`)

## Error Handling

The calculator handles various error cases:
- Division by zero: `Error: Division by zero`
- Invalid expressions: `Error: Invalid expression`
- Invalid numbers: `Error: Invalid number 'abc'`

## Arch package (desktop launcher)

After installing the `vyxcalc` Arch package (`pacman -U vyxcalc-*.pkg.tar.zst`), confirm desktop integration:

1. Check the launcher file exists: `/usr/share/applications/com.vylpes.vyxcalc.desktop`
2. Check the icon exists: `/usr/share/icons/hicolor/128x128/apps/com.vylpes.vyxcalc.png`
3. Open your desktop environment’s application menu, search for **Vyxcalc**, and confirm the branded icon appears
4. Launch from the menu — the GUI should open (same as running `vyxcalc` with no arguments)

If the menu icon looks missing after install, refresh the icon cache or log out/in:

```bash
gtk-update-icon-cache /usr/share/icons/hicolor
```

## Testing

Run the test suite:
```bash
cargo test
```

## Dependencies

- [clap](https://crates.io/crates/clap) - Command line argument parsing
- [rustyline](https://crates.io/crates/rustyline) - Interactive CLI line editing and session history
- [gtk4](https://crates.io/crates/gtk4) - GUI framework
- [glib](https://crates.io/crates/glib) - GLib bindings for Rust