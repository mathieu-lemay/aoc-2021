set dotenv-load := true

run day='':
    if [ -n "{{ day }}" ]; then \
        cargo run --bin day"$(printf "%02d" "{{ day }}")"; \
    else \
        cargo run --bin day"$(date "+%d")"; \
    fi


bench day='':
    if [ -n "{{ day }}" ]; then \
        cargo run --release --bin day"$(printf "%02d" "{{ day }}")"; \
    else \
        cargo run --release --bin day"$(date "+%d")"; \
    fi

test day='':
    if [ -n "{{ day }}" ]; then \
        RUST_BACKTRACE=1 cargo test --bin day"$(printf "%02d" "{{ day }}")"; \
    else \
        RUST_BACKTRACE=1 cargo test --bin day"$(date "+%d")"; \
    fi

prepare day='':
    #! /bin/sh

    set -eu

    if [ -n "{{ day }}" ]; then
        day="$(printf "%d" "{{ day }}")"
    else
        day="$(date "+%-d")"
    fi

    filename="$(printf "day%02d" "${day}")"

    [ -d "input" ] || mkdir input

    curl --fail --cookie "session=${SESSION_COOKIE:?Session cookie unavailable}" "https://adventofcode.com/2021/day/${day}/input" > "input/${filename}.txt"
    git add "input/${filename}.txt"
