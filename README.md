# RustCLI

## Description

RustCLI is a command-line application developed in Rust. It provides robust features to manage your tasks efficiently.

## Installation

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system. Then, clone the repository and build the project:

```zsh
git clone https://github.com/TheScattyPotty/rustCli.git
cd rustCli
cargo build
```

## Usage

### Basic Usage

To run the application:

```zsh
cargo run
```

### Advanced Usage

To run the application with specific files:

```zsh
cargo run -- -f ./file.http ./file2.http ./*.http
```

### File HTTP Format

```http
http://localhost:8080/api/v1/posts GET --specialHeader1=value1 --specialHeader2=value2
--body = {
    "key1":"value1",
    "key2":"value2",
}
```

## Create Executable & Start Using the CLI

### Build It

```zsh
cargo build --release
```

### Add it to PATH

For example, add it to `~/.zshrc`:

```zsh
export PATH="$PATH:$HOME/Desktop/rustCli/target/release"
```

You can modify this step as needed for your setup.

### Output

After finishing requests, the CLI will generate a file `output.http` that contains information about each executed request.

## Have Fun

```bash
:-)
```