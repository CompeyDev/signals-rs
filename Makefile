build: src/lib.rs
	@echo -e "\x1b[34m[\u001b[0m\x1b[31m*\x1b[34m\x1b[34m]\u001b[0m Building library signals-rs..."
	@echo -e "\x1b[34m[\u001b[0m\x1b[31m#\x1b[34m\x1b[34m]\u001b[0m cargo build --$(TYPE)"
	@cargo build --$(TYPE)

example: examples/
	@echo -e "\x1b[34m[\u001b[0m\x1b[31m*\x1b[34m\x1b[34m]\u001b[0m Building example $(EXAMPLE)"
	@echo -e "\x1b[34m[\u001b[0m\x1b[31m#\x1b[34m\x1b[34m]\u001b[0m cd examples/$(EXAMPLE) && cargo build --release"
	@cd examples/$(EXAMPLE) && cargo build --release
	@echo -e "\x1b[34m[\u001b[0m\x1b[31m#\x1b[34m\x1b[34m]\u001b[0m mv ./target/release/$(EXAMPLE).exe .."

docsgen:
	@echo -e "\x1b[34m[\u001b[0m\x1b[31m*\x1b[34m\x1b[34m]\u001b[0m Generating docs for signals-rs..."
	@echo -e "\x1b[34m[\u001b[0m\x1b[31m#\x1b[34m\x1b[34m]\u001b[0m cargo rustdoc --verbose"
	@cargo rustdoc --verbose
	@echo -e "\x1b[34m[\u001b[0m\x1b[31m#\x1b[34m\x1b[34m]\u001b[0m mv target/doc ."
	@mv target/doc .