# WebOS TV Guru

A desktop app for Windows, macOS and Linux that puts apps on LG webOS TVs.

LG TVs can run apps that are not in the LG Content Store, for example apps from the webOS Homebrew community or apps you build yourself. These apps come as `.ipk` files. WebOS TV Guru sends `.ipk` files to your TVs over your home network and installs them. It can also watch a folder on your computer and keep your TVs up to date with the apps in it.

## What it does

### Folder sync

- You choose a folder with `.ipk` files and one or more TVs.
- For every file and every TV, the app shows a status:
  - **Not installed**: the app is not on the TV yet.
  - **Update**: the TV has an older version.
  - **Up to date**: the TV has the same version.
  - **TV is newer**: the TV has a newer version than the file. The app does not downgrade it.
  - **Offline**, **Failed** or **Bad file** when something goes wrong. Hover over the status to see why.
- Click **Install pending** to install everything that is missing or old.
- Or turn on **auto install**: when you add or replace a file in the folder, the app installs it on your TVs by itself.
- If the folder has more than one version of the same app, only the newest one is installed.

### Apps

- See the apps installed on the selected TV, with their versions.
- Start an app on the TV or uninstall it.
- Drag `.ipk` files onto the window, or click **Install .ipk**, to install them. A progress bar shows the upload and the install.

### TVs

- Add as many TVs as you like, edit them, remove them, and test the connection.
- Two kinds of TV are supported:
  - **Developer Mode TV**: a normal TV with LG's Developer Mode app. You only enter the TV's IP address and the passphrase shown in the Developer Mode app. WebOS TV Guru gets the TV's key by itself.
  - **Rooted TV**: a TV with root access. You enter the IP address, user, and a password or an SSH key.
- The TV list is the same file the webOS SDK uses (`novacom-devices.json`), so TVs you added with LG's tools show up here too, and the other way round.

## How it works

The app talks to the TV over SSH, the same way LG's developer tools do. To install an app it:

1. Uploads the `.ipk` file to a temporary folder on the TV.
2. Checks that the file arrived without errors.
3. Asks the TV's app installer to install it, and shows its progress.
4. Deletes the uploaded file.

To compare versions, it reads the package name and version from inside each `.ipk` file and asks the TV which apps and versions it has.

## Before you start

On a Developer Mode TV:

1. Install the **Developer Mode** app from the LG Content Store and sign in with an LG developer account.
2. Turn on **Dev Mode Status** and **Key Server** in the app.
3. Your computer and the TV must be on the same network.

Developer Mode ends after some time unless you extend it in the Developer Mode app. When it ends, the TV removes apps that were installed in Developer Mode.

## Download

Installers for Windows, macOS and Linux (`.deb`, `.rpm`, `.AppImage`) are on the [Releases](https://github.com/vatichild/webos-tv-guru/releases) page. The builds are not signed yet, so Windows and macOS show a warning the first time you open the app.

## Develop

Built with Tauri v2 (Rust) and Vue 3 (TypeScript, Naive UI).

Requirements: Rust, Node 20+, and on Windows **Strawberry Perl** (OpenSSL is built from source, and Git Bash's Perl does not work). Put `C:\Strawberry\perl\bin` before other Perls on `PATH` for the first build.

```sh
npm install
npm run tauri:dev       # run the app
cd src-tauri && cargo test
npm run tauri:build     # installer
```

A new tag like `v0.1.0` builds the installers for all systems on GitHub Actions and adds them to a draft release.

## Layout

- `src-tauri/src/commands.rs`: Tauri commands, a thin layer over the modules
- `src-tauri/src/devices.rs`: TV list, Developer Mode key fetch, SSH connection
- `src-tauri/src/apps.rs`: list, install, remove and start apps
- `src-tauri/src/sync.rs`: folder watcher and sync
- `src-tauri/src/ipk.rs`, `version.rs`: read .ipk files, compare versions
- `src/pages/`: one page per tab, built only from components
- `src/components/`: reusable components by feature (`common`, `devices`, `apps`, `sync`)
- `src/composables/`: shared logic (`useSync`, `useInstaller`, `useInstalledApps`, ...)
- `src/lib/api.ts`: typed calls to the Rust commands
