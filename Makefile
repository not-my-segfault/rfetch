default:
	rm -rf target/*
	cargo build --release
test:
	target/release/rfetch
install:
	sudo cp target/release/rfetch /usr/local/bin/
uninstall:
	sudo rm /usr/local/bin/rfetch
