# Calculator App

A desktop calculator application written in Rust with both CLI and GUI modes.

## Features

- **CLI Mode**: Command-line interface with direct expression evaluation and interactive mode
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
./target/release/calculator-app cli "2 + 3"
# Output: 5

./target/release/calculator-app cli "10 / 2"
# Output: 5

./target/release/calculator-app cli "5 * 7"
# Output: 35
```

#### Interactive Mode
```bash
./target/release/calculator-app cli
```
This will start an interactive session where you can enter multiple expressions:
```
Calculator CLI - Interactive Mode
Enter expressions like '2 + 3' or 'quit' to exit
> 2 + 3
= 5
> 10 - 4
= 6
> quit
Goodbye!
```

### GUI Mode

```bash
./target/release/calculator-app gui
# or simply
./target/release/calculator-app
```

This opens a graphical calculator window with:
- Number buttons (0-9)
- Operation buttons (+, -, *, /)
- Equals button (=)
- Clear button (C)
- Decimal point button (.)

## Expression Format

Expressions must be in the format: `number operator number`

Examples:
- `2 + 3`
- `10.5 - 4.2`
- `7 * 8`
- `15 / 3`

## Error Handling

The calculator handles various error cases:
- Division by zero: `Error: Division by zero`
- Invalid expressions: `Error: Invalid expression`
- Invalid numbers: `Error: Invalid number 'abc'`

## Testing

Run the test suite:
```bash
cargo test
```

## Dependencies

- [clap](https://crates.io/crates/clap) - Command line argument parsing
- [gtk4](https://crates.io/crates/gtk4) - GUI framework
- [glib](https://crates.io/crates/glib) - GLib bindings for Rust