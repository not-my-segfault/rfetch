default:
	rm -rf target/*
	cargo build --release
test:
	target/release/rfetch
clean:
	rm -rf target/* Cargo.lock
install:
	sudo cp target/release/rfetch /usr/local/bin/
uninstall:
	sudo rm /usr/local/bin/rfetch
