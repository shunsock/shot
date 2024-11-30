#!/bin/bash -eu

function check() {
  local file_path=$1
  local executable="./target/release/shot"
  if "$executable" -f "$file_path" -d > "logs/$(basename "$file_path").log" ; then
    echo "[Notice] ✅ Run Example $file_path successfully"
    return 0
  else
    echo "[Error] ❌ Fail to Run Example $file_path"
    return 1
  fi
}

function main() {
  cargo build --release

  # Get list of files
  files=$(find examples -name "*.blt")

  # Run the tests
  local all_passed=true
  for file in $files; do
    echo "[Notice] Running test for $file"
    if ! check "$file" ; then
      all_passed=false
    fi
  done

  # Check if all tests passed
  if $all_passed; then
    echo "[Notice] 🎉 All Example Cases Passed"
    exit 0
  else
    echo "[Error] ❌ Not All Cases Passed. Check the log files for more information"
    exit 1
  fi
}

main
