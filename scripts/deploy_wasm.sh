#!/bin/bash
if [ -z "$1" ]; then
    echo "Usage: ./scripts/deploy_wasm.sh <deployment_name>"
    exit 1
fi

gsutil -h "Cache-Control:no-cache,max-age=0" cp rpg-explore.html gs://staging.robwil.io/rpg-explore/$1/index.html
gsutil -h "Cache-Control:no-cache,max-age=0" -h "Content-Type: application/wasm" cp rpg-explore.wasm gs://staging.robwil.io/rpg-explore/$1/
gsutil -h "Cache-Control:no-cache,max-age=0" cp -r assets/ gs://staging.robwil.io/rpg-explore/$1/