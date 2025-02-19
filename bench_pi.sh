#!/bin/bash

# Define project directories dynamically
projects=( $(find ~<your-path-to/Pi_multithread>-maxdepth 1 -type d -name 'pi_*') )

# Number of runs per project
runs=10

# Log file
log_file="benchmark_results.log"
echo "Benchmarking Rust Pi Calculations..." | tee "$log_file"

# Function to run benchmarks sequentially
run_benchmark() {
    local project="$1"
    local project_name=$(basename "$project")
    local total_time=0
    local binary="$project/target/release/$project_name"

    echo "Running $project..." | tee -a "$log_file"

    # Build the project
    cargo build --release --manifest-path "$project/Cargo.toml" > /dev/null 2>&1
    if [[ ! -f "$binary" ]]; then
        echo "Error: Binary not found after build!" | tee -a "$log_file"
        return
    fi

    for ((i=1; i<=runs; i++)); do
        echo -n "Run $i: " | tee -a "$log_file"

        # Capture execution time
        raw_time=$( { /usr/bin/time -p "$binary" > /dev/null; } 2>&1 | grep real )

        if [[ -z "$raw_time" ]]; then
            echo "Error capturing execution time!" | tee -a "$log_file"
            continue
        fi

        # Extract minutes and seconds properly
        total_seconds=$(echo "$raw_time" | awk '{print $2}')

        # Validate numeric output
        if ! [[ "$total_seconds" =~ ^[0-9]+(\.[0-9]+)?$ ]]; then
            echo "Error: Invalid time format!" | tee -a "$log_file"
            continue
        fi

        # Format output to always have leading zero
        formatted_time=$(printf "%.3f" "$total_seconds")

        echo "$formatted_time seconds" | tee -a "$log_file"
        total_time=$(echo "$total_time + $total_seconds" | bc)
    done

    # Calculate average if valid data is collected
    if (( $(echo "$total_time > 0" | bc -l) )); then
        avg_time=$(echo "scale=3; $total_time / $runs" | bc)
    else
        avg_time="N/A"
    fi

    echo "Average real time for $project_name: $avg_time seconds" | tee -a "$log_file"
    echo "-----------------------------------" | tee -a "$log_file"
}

# Run benchmarks sequentially
for project in "${projects[@]}"; do
    run_benchmark "$project"
done

echo "All benchmarks completed." | tee -a "$log_file"
