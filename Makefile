default:
	rm -rf target/*
	cargo build --release
	cargo build --release --target x86_64-pc-windows-gnu
test:
	target/*/rfetch
	wine64 target/*/*/rfetch.exe
clean:
	rm -rf target/* Cargo.lock
install:
	sudo cp target/release/rfetch /usr/local/bin/
uninstall:
	sudo rm /usr/local/bin/rfetch
