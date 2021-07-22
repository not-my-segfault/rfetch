linux:
	cargo build --release
win:
	cargo build --release --target x86_64-pc-windows-gnu
fbsd:
	cargo build --release --target x86_64-unknown-freebsd
test_l:
	target/*/rfetch;
test_w:
	wine64 target/x86_64-pc-windows-gnu/*/rfetch.exe
clean:
	rm -rf target/* Cargo.lock
install:
	sudo cp target/release/rfetch /usr/local/bin/
uninstall:
	sudo rm /usr/local/bin/rfetch
