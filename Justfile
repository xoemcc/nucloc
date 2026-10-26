build: 
	cargo build 

release: 
	cargo build --release

run *args: 
	cargo run -- {{args}}
