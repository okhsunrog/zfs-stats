#!/bin/bash

# Helper script to start Tauri dev server and stop it when ready
# This regenerates TypeScript bindings and tests compilation

echo "🚀 Starting Tauri dev server..."

# Clean up any existing processes on port 1420
lsof -ti:1420 | xargs kill -9 2>/dev/null || true

# Create a temporary file to capture output
TEMP_LOG=$(mktemp)
SUCCESS=false

# Function to cleanup
cleanup() {
    echo "🧹 Cleaning up..."
    lsof -ti:1420 | xargs kill -9 2>/dev/null || true
    pkill -f "tauri dev" 2>/dev/null || true
    rm -f "$TEMP_LOG" 2>/dev/null
}

# Set trap for cleanup
trap cleanup EXIT

# Start the dev server in background, capturing output
echo "Starting compilation (this may take up to 2 minutes)..."
timeout 120s bun tauri dev > "$TEMP_LOG" 2>&1 &
DEV_PID=$!

# Monitor the log file for success indicators
echo "Monitoring for success indicators..."
START_TIME=$(date +%s)

while kill -0 $DEV_PID 2>/dev/null; do
    # Check if we found success indicators in the log
    if grep -q "Logging initialized" "$TEMP_LOG" 2>/dev/null; then
        echo ""
        echo "✅ Found 'Logging initialized' - Tauri app compiled successfully!"
        echo "✅ TypeScript bindings should be regenerated!"
        SUCCESS=true
        break
    fi
    
    if grep -q "Successfully fetched ZFS stats" "$TEMP_LOG" 2>/dev/null; then
        echo ""
        echo "✅ Found ZFS stats fetch - App is running correctly!"
        SUCCESS=true
        break
    fi
    
    # Check for compilation errors
    if grep -q "could not compile" "$TEMP_LOG" 2>/dev/null; then
        echo ""
        echo "❌ Compilation failed!"
        break
    fi
    
    # Show progress every 10 seconds
    CURRENT_TIME=$(date +%s)
    ELAPSED=$((CURRENT_TIME - START_TIME))
    if [ $((ELAPSED % 10)) -eq 0 ] && [ $ELAPSED -gt 0 ]; then
        echo "⏳ Still compiling... (${ELAPSED}s elapsed)"
    fi
    
    sleep 1
done

# Kill the dev process if it's still running
if kill -0 $DEV_PID 2>/dev/null; then
    kill $DEV_PID 2>/dev/null
    wait $DEV_PID 2>/dev/null
fi

echo ""
if [ "$SUCCESS" = true ]; then
    echo "🎉 Development server test completed successfully!"
    
    # Show the last few lines of output for context
    echo ""
    echo "📋 Last few lines of output:"
    tail -5 "$TEMP_LOG" 2>/dev/null | sed 's/^/   /'
    
    exit 0
else
    echo "⚠️  Development server test failed or timed out"
    
    # Show the last few lines of output for debugging
    echo ""
    echo "📋 Last few lines of output for debugging:"
    tail -10 "$TEMP_LOG" 2>/dev/null | sed 's/^/   /'
    
    exit 1
fi