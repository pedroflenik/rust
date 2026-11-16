#!/bin/bash

OUTPUT_FILE="stored_time.txt"
echo "threads,run,real_seconds" > "$OUTPUT_FILE"

for t in $(seq 1 12); do
    echo "Running tests for $t threads..."
    
    for ((run=1; run<=10; run++)); do
        elapsed=$(TIMEFORMAT='%R'; { time ./exe5 $t > /dev/null; } 2>&1)
        
        echo "$t,$run,$elapsed" >> "$OUTPUT_FILE"
    done
done

echo "All runs completed. Data saved to $OUTPUT_FILE"