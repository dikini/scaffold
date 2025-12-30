#!/bin/bash
# update-github-urls.sh - Update placeholder GitHub URLs with actual repository URL
# Usage: ./update-github-urls.sh <your-github-username> <repository-name>

if [ $# -ne 2 ]; then
    echo "Usage: $0 <github-username> <repository-name>"
    echo "Example: $0 myusername scaffold"
    exit 1
fi

USERNAME=$1
REPO=$2
OLD_URL="your-org/scaffold"
NEW_URL="$USERNAME/$REPO"

echo "Updating GitHub URLs from '$OLD_URL' to '$NEW_URL'..."

# Find all markdown files and update URLs
find . -name "*.md" -type f -exec sed -i "s|your-org/scaffold|$NEW_URL|g" {} \;

echo "✅ Updated $(grep -r "$NEW_URL" --include="*.md" . | wc -l) URLs"
echo "🔍 Verification - new URLs found:"
grep -r "$NEW_URL" --include="*.md" . | head -5