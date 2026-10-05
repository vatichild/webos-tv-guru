# WebOS TV Guru

Desktop app (Tauri v2 + Vue 3) that installs `.ipk` apps on LG webOS TVs and keeps a folder of them in sync.

- **Folder sync**: watch a folder of `.ipk` files; see per-TV status (not installed / update / up to date) and install pending ones, or let it auto-install on change.
- **Apps**: list, launch and uninstall apps on a TV; drag-and-drop `.ipk` files to install.
- **TVs**: add Developer Mode TVs (key fetched from the TV's key server with the passphrase) or rooted TVs (SSH as root). The list is the webOS SDK's `novacom-devices.json`.

## Develop

Requirements: Rust, Node 20+, and on Windows **Strawberry Perl** (OpenSSL is built from source; Git Bash's msys Perl doesn't work). Put `C:\Strawberry\perl\bin` ahead of other Perls on `PATH` for the first build.

```sh
npm install
npm run tauri dev       # run the app
cd src-tauri && cargo test
npm run tauri build     # installer
```

## Layout

- `src-tauri/src/devices.rs`: device list, Dev Mode key fetch, SSH connect
- `src-tauri/src/apps.rs`: list / install / remove / launch via luna (`appInstallService/dev/install`)
- `src-tauri/src/sync.rs`: folder watcher and sync engine (emits `sync://entries`)
- `src-tauri/src/ipk.rs`, `version.rs`: `.ipk` control parsing, Debian version compare
- `src/components/`: `SyncView`, `AppsView`, `DevicesView`
