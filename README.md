# PostgreSQL Viewer

PostgreSQL Viewer is a web-based application that allows users to easily browse and interact with PostgreSQL databases. It provides a user-friendly interface for viewing table structures, querying data, and displaying results in both tabular and JSON formats.

## Features

- List all tables in the connected PostgreSQL database
- View table data with pagination
- Display data in an easy-to-read tabular format
- Show JSON representation of the data with syntax highlighting
- Simple and intuitive user interface

## Technology Stack

- Backend: Rust with Rocket framework
- Frontend: HTML, CSS (Bootstrap), and JavaScript
- Database: PostgreSQL
- AJAX: HTMX for seamless interactions

## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust and Cargo installed (https://www.rust-lang.org/tools/install)
- PostgreSQL database server
- Node.js and npm (for managing frontend dependencies, if any)

## Installation

1. Clone the repository:

   ```
   git clone https://github.com/anishpras/postgres-viewer.git
   cd postgres-viewer
   ```

2. Set up the environment variables:
   Create a `.env` file in the project root and add your PostgreSQL database URL:

   ```
   DATABASE_URL=postgres://username:password@localhost/database_name
   ```

3. Build the Rust project:

   ```
   cargo build --release
   ```

4. Install frontend dependencies (if any):
   ```
   npm install
   ```

## Usage

1. Start the application:

   ```
   cargo run --release
   ```

2. Open a web browser and navigate to `http://localhost:8000` (or the port specified in your configuration).

3. Use the interface to:
   - Select tables from the left sidebar
   - View table data in the main content area
   - Explore the JSON representation of the data below the table view

## Development

To run the application in development mode with hot-reloading:

```
cargo watch -x run
```

## Contributing

Contributions to the PostgreSQL Viewer are welcome. Please follow these steps to contribute:

1. Fork the repository
2. Create a new branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Commit your changes (`git commit -m 'Add some amazing feature'`)
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Acknowledgments

- [Rocket](https://rocket.rs/) - The web framework used
- [HTMX](https://htmx.org/) - For easy AJAX interactions
- [Bootstrap](https://getbootstrap.com/) - For responsive design

## Contact

If you have any questions or feedback, please open an issue on the GitHub repository.
