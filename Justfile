default:
    @just --list

db:
    surreal start -u root -p root

sim:
    cargo run -- -c 8 -d 500 -q 500
