# rust-link-shortener

This is a Rust project that provides a URL shortening service. It uses the Rocket framework for building web applications and utilizes a `HashMap` with a `Mutex` for storing the shortened URLs.

## Getting Started

To get started with the project, follow these instructions:

### Prerequisites

Make sure you have Rust and Cargo installed on your system. You can download them from the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/helixDevelopment/rust-link-shortener
   ```

2. Change to the project directory:

   ```bash
   cd rust-link-shortener
   ```

3. Build the project:

   ```bash
   cargo build
   ```

4. Run the project:

   ```bash
   cargo run
   ```

   The project will start a web server on `http://localhost:8000`.

## Usage

### Get

- Endpoint: `/get/<slug>`
- Method: GET

This endpoint is used to retrieve and redirect to the original URL associated with the given `<slug>`. If the `<slug>` exists in the hashmap, it will redirect to the associated URL. Otherwise, it will redirect to the error page.

### Shorten

- Endpoint: `/shorten/<url>`
- Method: GET

This endpoint is used to shorten a provided `<url>`. It generates a random `<slug>` and associates it with the `<url>` in the hashmap. The shortened URL can be accessed using `http://localhost:8000/get/<slug>`.

## Contributing

Contributions to this project are welcome. To contribute, follow these steps:

1. Fork the repository.
2. Create a new branch.
3. Make your changes.
4. Test your changes.
5. Commit your changes and push them to your fork.
6. Create a pull request.

Please ensure that your code adheres to the existing coding style and conventions.

## License

This project is licensed under the [MIT License](LICENSE).