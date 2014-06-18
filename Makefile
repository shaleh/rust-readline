RUSTC ?= rustc

all:
	$(RUSTC) src/lib.rs

clean:
	find . -name '*~' -exec rm {} \;
	rm *.rlib
