test-script:
	cargo build && for file in tests/scripts/*.sge; do echo "=== $$file ==="; ./target/debug/sagelang "$$file"; echo; done
test-oop:
	cargo build && for file in tests/oop/*.sge; do echo "=== $$file ==="; ./target/debug/sagelang "$$file"; echo; done