# Parcel

Convert roblox place files to rojo projects with a nice gui.

Forked from [rbxlx-to-rojo](https://github.com/rojo-rbx/rbxlx-to-rojo) by [Kampfkarren](https://github.com/Kampfkarren)

<img width="1920" height="1032" alt="{06A26287-C250-45D1-9C7B-512CD60DE7B7}" src="https://github.com/user-attachments/assets/0e2cefb3-c67a-45d2-a960-33e7c59efc7e" />

# Showcase
https://www.youtube.com/watch?v=PHIzqpJm34k

## What it does

- Converts `.rbxl` and `.rbxlx` files to rojo project structure
- Extracts all your scripts, folders, and services
- Shows live logs so you can see what's happening
- Dark terminal style UI because why not

## Requirements

- Rojo 0.5.0 alpha 12+ to actually use the generated projects
- A place file with at least one script in it

> Tip: Use `.rbxl` (binary) format if `.rbxlx` gives you errors. Some newer XML features aren't supported yet.

## Installation

Grab the latest release: [here](https://github.com/beeeegi/parcel/releases)

Or build it yourself:

```bash
npm install
npm run build
```

You'll find the exe in `target/release/parcel.exe`

## How to use

1. Open parcel
2. Pick where you want the project saved
3. Pick your place file
4. Hit convert
5. Wait few secs
6. Done

## Output

```
your-project/
├── default.project.json
└── src/
    ├── ReplicatedStorage/
    ├── ServerScriptService/
    ├── ServerStorage/
    └── ...
```

## Credits

- Original tool by [Kampfkarren](https://github.com/Kampfkarren)
- Uses [rbx-dom](https://github.com/rojo-rbx/rbx-dom) for parsing
- Built with [Tauri](https://tauri.app/) and [Rust](https://rust-lang.org)

## License

MPL-2.0, see [LICENSE.md](LICENSE.md)

## :star: If you found this tool useful, consider starring the repository to show your support!
