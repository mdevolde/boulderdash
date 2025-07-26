# watchexec -r -e rs -- powershell -NoProfile -ExecutionPolicy Bypass -File ./dev_watch_and_build.ps1
# watchexec must be installed to use this script. Use `cargo install watchexec` to install it.

Write-Host "1/3 Building Boulder Dash..."
wasm-pack build --target web --out-dir ./static/out

# A python virtual environment is expected to be set up in the `env` directory
Write-Host "2/3 Activating virtual environment..."
.\env\Scripts\Activate.ps1

Write-Host "3/3 Running Python server..."
Write-Host "When the server is running, make a change to the Rust code to trigger a rebuild."
python app.py
