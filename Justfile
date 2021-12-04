set dotenv-load := true

run day:
    cargo run --bin day"$(printf "%02d" "{{ day }}")"

bench day:
    cargo run --release --bin day"$(printf "%02d" "{{ day }}")"

test day:
    RUST_BACKTRACE=1 cargo test --bin day"$(printf "%02d" "{{ day }}")"

prepare day:
    #! /bin/sh

    set -eu

    day="$(printf "%d" "{{ day }}")"
    filename="$(printf "day%02d" "{{ day }}")"

    [ -d "input" ] || mkdir input

    curl --fail --cookie "session=${SESSION_COOKIE:?Session cookie unavailable}" "https://adventofcode.com/2021/day/${day}/input" > "input/${filename}.txt"
    git add "input/${filename}.txt"
