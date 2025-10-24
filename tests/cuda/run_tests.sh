#!/bin/bash
# Script to run all CUDA GPU tests

echo "=================================="
echo "Running SageLang CUDA GPU Tests"
echo "=================================="
echo ""

TESTS_DIR="tests/cuda"
FAILED=0
PASSED=0

for test_file in "$TESTS_DIR"/*.txt; do
    if [ -f "$test_file" ]; then
        echo "Running: $(basename "$test_file")"
        echo "---"
        
        if cargo run "$test_file" 2>&1 | grep -q "complete"; then
            echo "✓ PASSED"
            ((PASSED++))
        else
            echo "✗ FAILED"
            ((FAILED++))
        fi
        
        echo ""
    fi
done

echo "=================================="
echo "Test Results:"
echo "  Passed: $PASSED"
echo "  Failed: $FAILED"
echo "=================================="

if [ $FAILED -eq 0 ]; then
    exit 0
else
    exit 1
fi
