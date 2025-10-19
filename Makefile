test-script:
	cargo build && for file in tests/scripts/*.txt; do echo "=== $$file ==="; ./target/debug/sagelang "$$file"; echo; done
test-oop:
	cargo build && for file in tests/oop/*.txt; do echo "=== $$file ==="; ./target/debug/sagelang "$$file"; echo; done