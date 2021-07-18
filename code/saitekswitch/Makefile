#  Simple makefile
#  Compile for release
#  Strip
#  Copy to git root

compile:
	cargo build --release
	strip target/release/saitekswitch
	cp target/release/saitekswitch ../..
	