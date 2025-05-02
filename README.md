# Rust Calculator

A modern calculator application built with Rust and WebAssembly, featuring both a command-line interface and a web-based interface.

## Features

### Command-Line Calculator
- Basic arithmetic operations (addition, subtraction, multiplication, division)
- Input validation
- Interactive command-line interface
- Continuous operation until user chooses to exit

### Web Interface
- Modern, responsive design
- Real-time calculation
- Operation history
- Input validation
- Clear button functionality
- Dropdown menu for operations
- Orange result display
- Black background with white calculator box

## Project Structure

```
calculator/
├── src/
│   ├── main.rs      # Command-line calculator implementation
│   ├── lib.rs       # Web interface implementation
│   └── Cargo.toml   # Project dependencies and configuration
├── index.html       # Web interface HTML and CSS
└── README.md        # Project documentation
```

## Building and Running

### Command-Line Calculator
```bash
cargo run
```

### Web Interface
1. Build the WebAssembly package:
```bash
wasm-pack build --target web
```

2. Serve the application:
```bash
python3 -m http.server 8080
```

3. Open your browser and navigate to:
```
http://localhost:8080
```

## Dependencies

- Rust (latest stable version)
- wasm-pack (for building WebAssembly)
- Python 3 (for serving the web interface)

## Implementation Details

### Command-Line Calculator
The command-line calculator is implemented in `main.rs` and provides:
- A loop-based interface for continuous calculations
- Input validation for numbers
- Support for basic arithmetic operations
- Clear error messages for invalid inputs

### Web Interface
The web interface is implemented using:
- Yew framework for Rust WebAssembly
- Modern CSS for styling
- Responsive design principles
- State management for calculations and history

## Contributing

Feel free to submit issues and enhancement requests!

## License

This project is open source and available under the MIT License. 
