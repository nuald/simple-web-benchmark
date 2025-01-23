#!/bin/sh -xe

docker_build="docker build --progress=plain suite/ -t simple-web-benchmark"
docker_run="docker run -it --rm -v $PWD:/src --network=\"host\" simple-web-benchmark"
exit_trap_command=""

tear_down() {
    eval "$exit_trap_command"
}
trap tear_down EXIT

add_exit_trap() {
    to_add=$1
    if [ "$exit_trap_command" ]; then
        exit_trap_command="$exit_trap_command; $to_add"
    else
        exit_trap_command="$to_add"
    fi
}

build() {
    eval "$docker_build"
}

rebuild() {
    eval "$docker_build --no-cache"
}

shell() {
    eval "$docker_run"
}

run() {
    cargo run --manifest-path suite/Cargo.toml -- all
}

versions() {
    cargo run --manifest-path suite/Cargo.toml --bin versions
}

lint() {
    docker run --rm -i hadolint/hadolint < suite/Dockerfile
}

"$@"
