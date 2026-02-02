# https://github.com/casey/just

alias u := update
alias b := build
alias d := dev
alias c := check

# Build the project in release mode
build: 
    npx tauri build

# update the version number (x.y.z | patch | minor | major) for app
update VER:
    ./update-version {{VER}}
    npm install
    cd src-tauri && cargo check

# Run the project in development mode
dev:
    npx tauri dev

check:
    cd src-tauri && cargo check
    cd src-tauri && cargo clippy --all-targets --all-features -- -D warnings
    cd src-tauri && cargo fmt