#!/bin/bash

# Font setup script for fake-productivity
# Ensures proper Japanese character support

echo "🔧 Setting up Japanese font support for fake-productivity..."
echo "=============================================="

# Detect OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "📱 Detected Linux system"
    
    # Check for package manager
    if command -v apt &> /dev/null; then
        echo "📦 Installing fonts via apt..."
        sudo apt update
        sudo apt install -y fonts-noto-cjk fonts-noto-color-emoji fonts-liberation
        echo "✅ Fonts installed via apt"
    elif command -v pacman &> /dev/null; then
        echo "📦 Installing fonts via pacman..."
        sudo pacman -S --noconfirm noto-fonts-cjk noto-fonts-emoji
        echo "✅ Fonts installed via pacman"
    elif command -v dnf &> /dev/null; then
        echo "📦 Installing fonts via dnf..."
        sudo dnf install -y google-noto-cjk-fonts google-noto-emoji-fonts
        echo "✅ Fonts installed via dnf"
    elif command -v zypper &> /dev/null; then
        echo "📦 Installing fonts via zypper..."
        sudo zypper install -y noto-sans-cjk-fonts noto-coloremoji-fonts
        echo "✅ Fonts installed via zypper"
    else
        echo "⚠️  Package manager not detected. Please install CJK fonts manually."
        echo "   Look for: noto-fonts-cjk or similar packages"
    fi
    
elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo "🍎 Detected macOS system"
    
    if command -v brew &> /dev/null; then
        echo "📦 Installing fonts via Homebrew..."
        brew install --cask font-noto-sans-cjk-jp
        brew install --cask font-source-han-code-jp
        echo "✅ Fonts installed via Homebrew"
    else
        echo "⚠️  Homebrew not found. Please install Japanese fonts manually:"
        echo "   1. Download Noto Sans CJK JP from Google Fonts"
        echo "   2. Install via Font Book"
    fi
    
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
    echo "🪟 Detected Windows system"
    echo "📝 For Windows, please:"
    echo "   1. Use Windows Terminal (recommended)"
    echo "   2. Download and install Noto Sans CJK JP from Google Fonts"
    echo "   3. Or use Cascadia Code PL (pre-installed with Windows Terminal)"
    echo "   4. Set terminal font to one of the above"
else
    echo "❓ Unknown OS type: $OSTYPE"
    echo "📝 Please install a CJK-compatible font manually"
fi

echo ""
echo "🔧 Setting up locale..."

# Check and set UTF-8 locale
if [[ -z "$LANG" ]] || [[ "$LANG" != *"UTF-8"* ]]; then
    echo "⚠️  Setting UTF-8 locale..."
    
    # Add to shell config
    SHELL_CONFIG=""
    if [[ "$SHELL" == *"zsh"* ]]; then
        SHELL_CONFIG="$HOME/.zshrc"
    elif [[ "$SHELL" == *"bash"* ]]; then
        SHELL_CONFIG="$HOME/.bashrc"
    fi
    
    if [[ -n "$SHELL_CONFIG" ]]; then
        echo "export LANG=en_US.UTF-8" >> "$SHELL_CONFIG"
        echo "export LC_ALL=en_US.UTF-8" >> "$SHELL_CONFIG"
        echo "✅ Added UTF-8 locale to $SHELL_CONFIG"
        echo "📝 Please restart your terminal or run: source $SHELL_CONFIG"
    fi
else
    echo "✅ UTF-8 locale already configured: $LANG"
fi

echo ""
echo "🧪 Testing character support..."
echo "Japanese test: 日本語テスト こんにちは"
echo "Symbols test: ▲ ◆ ● ■ ★ ◉ ⬢ ⬡"
echo ""
echo "If you see boxes (□) instead of characters, your terminal needs font configuration."
echo ""
echo "🚀 Setup complete! Run the application to test:"
echo "   ./run.sh"
echo "   ./run.sh --matrix"
