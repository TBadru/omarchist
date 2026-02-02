# https://github.com/casey/just

alias u := update
alias b := build

# Build the project in release mode
build: 
    npx tauri build

# update the version number (x.y.z | patch | minor | major) for app
update VER:
    ./update-version {{VER}}
    cargo check
    npm install

# Run the project in development mode
dev:
    npx tauri dev