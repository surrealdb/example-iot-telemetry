default:
    @just --list

db:
    surreal start -u root -p root

sim:
    cargo run -- -t 10 -d 500
