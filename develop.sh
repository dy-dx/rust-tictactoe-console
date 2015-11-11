#!/bin/sh

# first, npm install -g nodemon

# re-runs "cargo run" whenever a src/*.rs file changes
nodemon --watch src -e rs --exec cargo run
