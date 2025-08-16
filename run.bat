@echo off
title Fake Productivity Console
echo Starting Fake Productivity System...
echo Use --matrix flag for Matrix mode: run.bat --matrix
echo.
cargo run -- %*
