 export DOCKER_KAFKA_HOST=$(ipconfig getifaddr en0)
 PII_LOG_ENDPOINT="localhost:9092" \
 PII_LOG_KEY_VAULT_ACCOUNT="nicksecretstoredev001" \
 PII_LOG_KEY_VAULT_KEY_NAME="certkafkadevnick005" \
 RUST_LOG=trace cargo run
