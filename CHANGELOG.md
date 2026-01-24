# Changelog

## 1.0.0 - 01/24/2026

First GUI release! Completely rewrote the old CLI tool into a desktop app.

### What's new
- Terminal-style dark UI with blood red theme
- Live activity log that updates while converting
- Progress bar so you know something's happening
- Open folder button after conversion
- Error popups that actually tell you what went wrong

### Technical stuff
- Uses tauri v2 for the app framework
- Conversion runs on a separate thread so the UI doesn't freeze
- Vanilla html/css/js frontend, no frameworks needed

### Removed from original
- CLI

## Before 1.0

This was originally [rbxlx-to-rojo](https://github.com/rojo-rbx/rbxlx-to-rojo), a CLI tool by Kampfkarren. parcel is a GUI rewrite using the same core conversion code.
