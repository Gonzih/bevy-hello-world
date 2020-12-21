# SHELLFLAGS = -I nixpkgs=/home/gnzh/mydev/nixpkgs

default: run-nix

run-nix:
	nix-shell $(SHELLFLAGS) shell.nix --run 'make run'

run:
	cargo run --features bevy/dynamic

shell:
	nix-shell $(SHELLFLAGS) shell.nix
