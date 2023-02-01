# Tiny Logger
A small logging library I use in a few other projects

This crate offers different logging macros,
that print out the messages in different colors depending on how important they are

# Examples
```rust
use tiny_logger;

// Removed in release mode
tiny_logger::trace!("Trace message");
tiny_logger::trace!("Trace with data: {:?}", data);

// Removed in release mode
tiny_logger::debug!("Debug message");
tiny_logger::debug!("Debug with data: {:?}", data);

tiny_logger::info!("Info message");
tiny_logger::info!("Info with data: {:?}", data);

tiny_logger::warn!("Warn message");
tiny_logger::warn!("Warn with data: {:?}", data);

tiny_logger::error!("Error message");
tiny_logger::error!("Error with data: {:?}", data);


tiny_logger::assert!(true, "Assert message");

tiny_logger::die!("Die message");
```

# Docs
```bash
cargo doc --open
```

# Getting Started
```bash
cargo run --example logging
```