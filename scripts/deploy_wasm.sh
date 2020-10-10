#!/bin/bash

gsutil -h "Cache-Control:no-cache,max-age=0" cp rpg-explore.html gs://staging.robwil.io/
gsutil -h "Cache-Control:no-cache,max-age=0" -h "Content-Type: application/wasm" cp rpg-explore.wasm gs://staging.robwil.io/
gsutil -h "Cache-Control:no-cache,max-age=0" cp -r assets/ gs://staging.robwil.io/