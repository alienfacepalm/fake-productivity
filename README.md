# Fake Productivity 🚀

A Rust application that generates convincing fake console logs to make your computer look like it's doing something incredibly important and sophisticated.

## Features

### Normal Mode (Default)

- **Colorful Console Output**: Beautiful colored logs that look professional
- **Multiple Log Types**:
  - System operations (memory, CPU, threading)
  - Database queries and optimizations
  - Network communications
  - AI/Machine Learning processes
  - Security operations
  - Data processing pipelines
- **Realistic Timestamps**: Every log entry has proper timestamps
- **Random Delays**: Varies timing to look natural
- **Progress Bars**: Occasional progress indicators for long operations

### Matrix Mode

- **Scrolling Green Text**: Classic Matrix-style display with Japanese characters
- **Neural Network Interface**: Simulated brain-computer interface
- **System Breach Alerts**: Matrix-themed security warnings
- **Encrypted Data Streams**: Hexadecimal data flows
- **Classic Matrix Quotes**: "Wake up, Neo..." and other iconic phrases

## Requirements

### Unicode/Japanese Character Support

This application uses Japanese characters (Hiragana, Katakana, Kanji) and Unicode symbols for authentic display. For the best experience:

#### Terminal Setup
- **Modern Terminal**: Use Alacritty, Kitty, iTerm2, Windows Terminal, or GNOME Terminal
- **UTF-8 Encoding**: Ensure your terminal is set to UTF-8 encoding
- **Environment**: Set `LANG=en_US.UTF-8` (or your locale with UTF-8)

#### Font Requirements
Install a font that supports CJK (Chinese, Japanese, Korean) characters:

**Recommended Fonts:**
- **Noto Sans CJK** / **Noto Sans JP** - Google's comprehensive CJK font
- **Source Han Sans** / **Source Code Pro** - Adobe's CJK font family
- **Fira Code** (with CJK fallback) - Popular programming font
- **JetBrains Mono** (with CJK fallback) - JetBrains' monospace font
- **Cascadia Code PL** - Microsoft's terminal font with good Unicode support

#### Installation Examples

**Ubuntu/Debian:**
```bash
# Install CJK fonts
sudo apt update
sudo apt install fonts-noto-cjk fonts-noto-color-emoji fonts-liberation

# Set UTF-8 locale (if not already set)
sudo locale-gen en_US.UTF-8
export LANG=en_US.UTF-8
export LC_ALL=en_US.UTF-8

# Add to your shell profile for persistence
echo 'export LANG=en_US.UTF-8' >> ~/.bashrc
echo 'export LC_ALL=en_US.UTF-8' >> ~/.bashrc
source ~/.bashrc
```

**Arch Linux:**
```bash
sudo pacman -S noto-fonts-cjk noto-fonts-emoji
```

**macOS:**
```bash
brew install --cask font-noto-sans-cjk-jp
```

**Windows PowerShell:**
```powershell
# Using Chocolatey (recommended)
choco install noto-fonts-cjk
refreshenv

# Or using Scoop
scoop bucket add nerd-fonts
scoop install Cascadia-Code
refreshenv

# Or download manually from Google Fonts
# https://fonts.google.com/noto/specimen/Noto+Sans+JP
```

**Windows Terminal Setup:**
1. Open Windows Terminal settings (Ctrl+,)
2. Go to Defaults > Appearance > Font face
3. Select "Cascadia Code PL" or installed Noto font
4. Save settings

#### Testing Your Setup
The application will display a character test on startup. If you see garbled characters or boxes (□), your font doesn't support Japanese characters.

## Usage

### Normal Mode

```bash
cargo run
```

Or use the convenient run scripts:

- Windows: `run.bat`
- Linux/macOS: `./run.sh`

### Matrix Mode

```bash
cargo run -- --matrix
```

Or use the run scripts with the matrix flag:

- Windows: `run.bat --matrix`
- Linux/macOS: `./run.sh --matrix`

Or use the dedicated Matrix scripts:

- Windows: `matrix.bat`
- Linux/macOS: `./matrix.sh`

### Command Line Options

```bash
cargo run -- --help
```

## Installation

### Prerequisites

First, install Rust from [rustup.rs](https://rustup.rs/):

1. Open PowerShell as Administrator
2. Run: `Set-ExecutionPolicy RemoteSigned -Scope CurrentUser`
3. Download and run the Rust installer from rustup.rs
4. Restart your terminal after installation

### Building and Running

```powershell
# Navigate to the project directory
cd "c:\Users\brand\PROJECTS\AlienFacepalm\Mock_Productive\fake-productivity"

# Build and run the project
cargo run

# Or for Matrix mode
cargo run -- --matrix
```

## Sample Output

### Normal Mode

```
🚀 Fake Productivity System v2.1.3 - Starting...
============================================
[2025-08-15 14:23:45.123] SYS CPU_CORE_MANAGER [ID:4567] - Optimizing memory allocation
[2025-08-15 14:23:47.456] DB QUERY_ENGINE - 23,451 rows affected (234ms)
    └─ SELECT * FROM neural_networks WHERE efficiency > 0.95
[2025-08-15 14:23:49.789] NET 192.168.1.100:8443 HTTPS 200 OK - 524,288 bytes
[2025-08-15 14:23:52.012] AI NEURAL_TRANSFORMER_V3 - Training epoch 127/200 completed (Accuracy: 94.73%)
[2025-08-15 14:23:54.345] SEC CRYPTO_ENGINE - Encrypted communication channel established
    └─ Hash: a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456
[2025-08-15 14:23:56.678] PROC DATA_PIPELINE - Processing quantum entanglement calculations [42,337/87,654] (48%)
    └─ [█████████░░░░░░░░░░░]
```

### Matrix Mode

```
THE MATRIX - NEURAL INTERFACE ACTIVE
======================================

アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン
NEURAL.NET.CORE_01 0x7F4A9E2D >> ACTIVE
ENCRYPTED_STREAM: 524288 bytes [A1B2C3D4]
NEURAL_NODE_1337:: ████████████░░░░░░░░ [60%]
WAKE UP, NEO...
FF 00 FF 00 CA FE BA BE DE AD BE EF 13 37 H4 CK
```

## Tips for Maximum Effect

1. **Full Screen**: Run in full screen terminal for maximum impact
2. **Dark Theme**: Use a dark terminal theme for that "hacker" aesthetic
3. **Multiple Windows**: Run multiple instances in different terminals
4. **Background Music**: Play some electronic/ambient music
5. **Dim Lighting**: Works best in dimly lit environments

## Dependencies

- `rand` - For random number generation
- `colored` - For terminal color output
- `chrono` - For timestamp generation
- `clap` - For command line argument parsing

## Cross-Platform

Works on Windows, macOS, and Linux. Both `.bat` and `.sh` run scripts are provided for convenience.

## Disclaimer

This is purely for entertainment purposes. Use responsibly and don't actually try to fool anyone into thinking you're saving the world with your computer! 😄
