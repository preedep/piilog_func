 export DOCKER_KAFKA_HOST=$(ipconfig getifaddr en0)
 PII_LOG_ENDPOINT=$DOCKER_KAFKA_HOST":9093" \
 PII_LOG_KEY_VAULT_ACCOUNT="nicksecretstoredev001" \
 PII_LOG_KEY_VAULT_KEY_NAME="certkafkadevnick003" \
 RUST_LOG=piilog_func cargo run