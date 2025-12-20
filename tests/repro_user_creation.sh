#!/bin/bash

# Configuration
API_URL="http://localhost:8081"
USER_EMAIL="test_$(date +%s)@example.com"

echo "Checking health endpoint..."
curl -s "$API_URL/health" | jq .

echo -e "\nCreating user: $USER_EMAIL"
RESPONSE=$(curl -s -X POST "$API_URL/users" \
     -H "Content-Type: application/json" \
     -d "{
           \"name\": \"Test User\",
           \"email\": \"$USER_EMAIL\",
           \"password\": \"password123\"
         }")

echo "$RESPONSE" | jq .

# Verify conflict
echo -e "\nVerifying email conflict..."
curl -s -X POST "$API_URL/users" \
     -H "Content-Type: application/json" \
     -d "{
           \"name\": \"Another User\",
           \"email\": \"$USER_EMAIL\",
           \"password\": \"password456\"
         }" | jq .
