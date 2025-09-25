# Bible CLI

A simple command-line interface for downloading and reading the Bible.

This is simply a project for me to learn Rust, however if useful, let me know and I'll carry on maintaining it

## Features

*   Download and read the Bible from the command line.
*   Supports multiple translations.
*   Set a default translation.
*   Get a specific verse or a whole chapter.
*   List available translations.
*   Configure the default translation via a config file or an environment variable.

## Installation

1.  Clone the repository:
    ```bash
    git clone https://github.com/your-username/bible-cli.git
    ```
2.  Build the project:
    ```bash
    cd bible-cli
    cargo build --release
    ```
3.  The executable will be in `target/release/bible`.

## Usage

### Get a verse

```bash
bible get <book> <chapter> <verse> [flags]
```

**Example:**

```bash
bible get Genesis 1 1
```

### Get a chapter

```bash
bible get <book> <chapter> [flags]
```

**Example:**

```bash
bible get Genesis 1
```

### Add a new translation

```bash
bible add <translation>...
```

**Example:**

```bash
bible add ASV BBE
```

### List available translations

```bash
bible list
```

### Configure the default translation

You can set the default translation in three ways:

1.  **Using the `config` command:**
    ```bash
    bible config <translation>
    ```
    **Example:**
    ```bash
    bible config ASV
    ```
2.  **Using the configuration file:**

    Create a `config.json` file in the configuration directory (`~/.config/bible-cli/config.json` on Linux) with the following content:

    ```json
    {
      "TRANSLATION": "ASV"
    }
    ```

3.  **Using an environment variable:**

    Set the `BIBLE_DEFAULT_TRANSLATION` environment variable:

    ```bash
    export BIBLE_DEFAULT_TRANSLATION=ASV
    ```

## Available Translations

A list of available translations can be found by running `bible add --help`.

