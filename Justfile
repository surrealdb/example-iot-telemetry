default:
    @just --list

db:
    surreal start -u root -p root

sim:
    cargo run -- -c 10 -d 500
