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
- **Environment Detection**: Automatically detects WSL and provides setup guidance

### Matrix Mode

- **Scrolling Green Text**: Classic Matrix-style display with Japanese characters
- **Neural Network Interface**: Simulated brain-computer interface
- **System Breach Alerts**: Matrix-themed security warnings
- **Encrypted Data Streams**: Hexadecimal data flows
- **Classic Matrix Quotes**: "Wake up, Neo..." and other iconic phrases
- **Authentic Japanese Characters**: Uses real Hiragana and Katakana for Matrix effect

### Fullscreen Standalone Mode

**NEW**: The application now runs in a proper fullscreen terminal interface:

- **Immersive Experience**: Takes over your entire terminal with clean, professional display
- **Signal Handling**: Graceful exit with Ctrl+C - properly restores terminal state
- **Terminal Control**: Uses ANSI escape sequences for full screen management
- **Clean Restoration**: Automatically restores cursor and screen buffer on exit
- **Quick Mode**: Fast-paced updates for high-intensity fake productivity
- **Daemon Mode**: Background process simulation (perfect for leaving running)

### Command Line Options

```bash
# Standard fullscreen mode
./fake-productivity

# Matrix mode with fullscreen interface
./fake-productivity --matrix

# Quick mode - high frequency updates
./fake-productivity --quick

# Daemon mode - simulated background process
./fake-productivity --daemon

# Skip Unicode check and start immediately
./fake-productivity --quick --matrix

# Show all available options
./fake-productivity --help
```

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

**Ubuntu/Debian (including WSL):**
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

**WSL (Windows Subsystem for Linux) Additional Steps:**
```bash
# After installing fonts in WSL, configure Windows Terminal:
# 1. Open Windows Terminal settings (Ctrl+,)
# 2. Go to your WSL profile settings
# 3. Under Appearance > Font face, select:
#    - "Cascadia Code PL" (pre-installed)
#    - Or install "Noto Sans Mono CJK JP" on Windows
# 4. Save settings and restart terminal

# Test Unicode support in WSL:
echo "Japanese test: 日本語テスト こんにちは"
echo "Symbols: ▲ ◆ ● ■ ★ ◉ ⬢ ⬡"
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

**Windows Terminal Setup (for Windows & WSL):**
1. Open Windows Terminal settings (Ctrl+,)
2. **For WSL**: Go to your specific WSL profile (Ubuntu, etc.)
3. **For PowerShell**: Go to Windows PowerShell profile
4. Under Appearance > Font face, select:
   - "Cascadia Code PL" (pre-installed, good CJK support)
   - "Noto Sans Mono CJK JP" (if manually installed)
   - "Consolas" (limited CJK, but with fallback)
5. Save settings and restart terminal

**WSL Font Troubleshooting:**
- Fonts installed in WSL don't affect Windows Terminal display
- Windows Terminal uses Windows-installed fonts for rendering
- Install CJK fonts on both Windows (for display) and WSL (for fallback)
- If characters appear as boxes, the Windows Terminal font lacks CJK support

#### Testing Your Setup
The application will display a character test on startup. If you see garbled characters or boxes (□), your font doesn't support Japanese characters.

#### Automated Setup
For convenience, you can run the automated font setup script:

```bash
# Make the script executable and run it
chmod +x setup-fonts.sh
./setup-fonts.sh
```

This script will:
- Detect your operating system (Linux, macOS, Windows/WSL)
- Install appropriate CJK fonts using your package manager
- Configure UTF-8 locale settings
- Provide platform-specific guidance

## Usage

### Quick Start

The application now runs in **fullscreen mode** by default, providing an immersive terminal experience:

```bash
# Build and run in fullscreen mode
cargo run

# Or use the convenient run script
./run.sh

# For Windows users
run.bat
```

### Advanced Usage

```bash
# Matrix mode with fullscreen neural interface
cargo run -- --matrix
./run.sh --matrix

# Quick mode - fast updates, skip Unicode check
cargo run -- --quick
./run.sh --quick

# Daemon mode - simulated background process
cargo run -- --daemon
./run.sh --daemon

# Combine flags for customized experience
cargo run -- --matrix --quick  # Fast Matrix mode
./run.sh --matrix --daemon      # Background Matrix mode
```

### Professional Usage Tips

1. **Meetings**: Use `--matrix` mode for impressive tech displays
2. **Quick Demos**: Use `--quick` for fast-paced activity
3. **Background**: Use `--daemon` when you need to step away
4. **Exit Cleanly**: Always use Ctrl+C to properly restore terminal

### Legacy Simple Mode

If you prefer the old simple console output without fullscreen:

```bash
# For development/debugging only
cargo run -- --quick 2>/dev/null | head -20
```

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
� Checking Unicode/Japanese character support...
Test characters: 日本語テスト こんにちは 漢字 ひらがな カタカナ
CJK symbols: ▲ ◆ ● ■ ★ ◉ ⬢ ⬡
🐧 WSL Environment Detected: Ubuntu-22.04
   Make sure Windows Terminal is configured with a CJK font!

📝 For best Japanese character display, use a font that supports CJK:
   • Noto Sans CJK / Noto Sans JP
   • Source Han Sans / Source Code Pro
   • Fira Code (with CJK fallback)
   • JetBrains Mono (with CJK fallback)
   • Cascadia Code PL

🖥️  Terminal recommendations:
   • Modern terminals: Alacritty, Kitty, iTerm2, Windows Terminal
   • For WSL: Use Windows Terminal with CJK font configured
   • Enable UTF-8 encoding in your terminal settings
============================================
�🚀 Fake Productivity System v2.1.3 - Starting...
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
🔍 Checking Unicode/Japanese character support...
Test characters: 日本語テスト こんにちは 漢字 ひらがな カタカナ
CJK symbols: ▲ ◆ ● ■ ★ ◉ ⬢ ⬡
============================================
THE MATRIX - NEURAL INTERFACE ACTIVE
======================================

アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン
NEURAL.NET.CORE_01 0x7F4A9E2D >> ACTIVE
ENCRYPTED_STREAM: 524288 bytes [A1B2C3D4]
NEURAL_NODE_1337:: ████████████░░░░░░░░ [60%]
WAKE UP, NEO...
FF 00 FF 00 CA FE BA BE DE AD BE EF 13 37 H4 CK
AGENT.SMITH.TRACE 0x1337H4CK >> BREACH
MIND_BRIDGE_4069:: █████████████░░░░░░░ [68%]
```

## Troubleshooting

### Japanese Characters Not Displaying

**Symptoms**: You see boxes (□) or question marks (?) instead of Japanese characters.

**Solutions**:
1. **Check terminal font**: Ensure your terminal uses a CJK-compatible font
2. **WSL users**: Configure Windows Terminal font, not WSL fonts
3. **Run setup script**: Use `./setup-fonts.sh` for automated setup
4. **Manual font installation**: Follow the installation examples above
5. **UTF-8 locale**: Ensure `LANG` environment variable includes UTF-8

### WSL-Specific Issues

**Problem**: Characters work in terminal but not in editors like joe/vim
**Solution**: 
- For joe: Use `joe -utf8 filename` or configure `~/.joerc` with `-utf8`
- For vim: Usually works by default with UTF-8
- Alternative: Use `nano` or `code` (VS Code) which handle UTF-8 better

### Performance Issues

**Problem**: Application runs slowly or uses too much CPU
**Solutions**:
- The application is designed to use some CPU for realistic effect
- Use Ctrl+C to stop the application
- Consider running in background: `cargo run &`

### Build Issues

**Problem**: Compilation errors
**Solutions**:
- Ensure you have Rust installed: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Update Rust: `rustup update`
- Clean build: `cargo clean && cargo build`

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
