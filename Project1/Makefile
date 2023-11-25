profile:
	RUSTFLAGS='-g' cargo build --release; \
	valgrind --tool=callgrind --callgrind-out-file=callgrind.out --collect \
	--collect-jumps=yes --simulate-cache=yes \
	./target/release/project1

