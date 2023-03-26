check_defined = \
    $(strip $(foreach 1,$1, \
        $(call __check_defined,$1,$(strip $(value 2)))))
__check_defined = \
    $(if $(value $1),, \
        $(error Mandatory argument $1$(if $2, ($2))$(if $(value @), \
                not provided. Required by target `$@`)))

log_prefix := \x1b[34m[\u001b[0m\x1b[31m*\x1b[34m\x1b[34m]\u001b[0m
command_prefix := \x1b[34m[\u001b[0m\x1b[31m\#\x1b[34m\x1b[34m]\u001b[0m
examples_dir := examples/$(EXAMPLE)

build: src/lib.rs
	@:$(call check_defined, TYPE, type of build: debug/release)

	@echo -e "${log_prefix} Building library signals-rs..."
	@echo -e "${command_prefix} cargo build --$(TYPE)"
	@cargo build --$(TYPE)

example: examples/
	@:$(call check_defined, EXAMPLE, example to build)

	@echo -e "${log_prefix} Building example $(EXAMPLE)"
	@echo -e "${command_prefix} cd examples/$(EXAMPLE) && cargo build --release"
	@cd ${examples_dir} && cargo build --release
	@echo -e "${command_prefix} mv ./target/release/$(EXAMPLE).exe .."
	@mv ./${examples_dir}/target/release/$(EXAMPLE).exe .

docsgen:
	@echo -e "${log_prefix} Generating docs for signals-rs..."
	@echo -e "${command_prefix} cargo rustdoc --verbose"
	@cargo rustdoc --verbose
	@echo -e "${command_prefix} mv target/doc ."
	@mv target/doc .