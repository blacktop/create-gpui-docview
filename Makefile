TEST_APP := gpui-docview-test

.PHONY: build
build:
	cargo fmt -- --check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo build

.PHONY: generate
generate: build
	@rm -rf $(TEST_APP) || true
	@echo "Generating sample workspace $(TEST_APP)...";
	cargo run -- --name $(TEST_APP)

.PHONY: check
check:
	$(MAKE) generate
	cd $(TEST_APP); cargo check -p $(TEST_APP)

.PHONY: run
run: generate
# 	@if [ ! -d $(TEST_APP) ]; then \
# 		$(MAKE) generate; \
# 	fi
	cd $(TEST_APP); cargo run -p $(TEST_APP)

.PHONY: run-clean
run-clean:
	$(MAKE) clean
	$(MAKE) run

clean:
	@rm -rf $(TEST_APP) || true
	cargo clean