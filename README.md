# 📚🎥 Media Catalog 📚🎥

Welcome to the Media Catalog project! This Rust application allows you to manage a collection of different media types, including books, movies, audiobooks, podcasts, and placeholders. The catalog can store and retrieve media items using a unique identifier.

## 📦 Project Structure

- **main.rs**: The entry point of the application.
- **content**: A module containing the `catalog` and `media` modules.
  - **catalog.rs**: Contains the `Catalog` struct and its implementation.
  - **media.rs**: Defines the `Media` enum and its implementation.

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your system.

### Running the Project

1. **Clone the repository**:

   git clone

2. **Run the application**:
   ```sh
   cargo run
   ```

## 📋 Usage

The application demonstrates the use of enums and structs in Rust to handle various types of media. Here's an overview of the main components:

### 📝 Media Enum

The `Media` enum can represent different types of media:

- **Book**: Contains a `title` and `author`.
- **Movie**: Contains a `title` and `director`.
- **Audiobook**: Contains a `title`.
- **Podcast**: Contains an episode number.
- **Placeholder**: Represents a placeholder item.

Example:

```rust
let audiobook = Media::Audiobook {
    title: String::from("Flipper"),
};
```
