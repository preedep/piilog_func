 export DOCKER_KAFKA_HOST=$(ipconfig getifaddr en0)
 PII_LOG_ENDPOINT="kafka.preedee.space:9092" \
 PII_LOG_KEY_VAULT_ACCOUNT="nicksecretstoredev001" \
 PII_LOG_KEY_VAULT_KEY_NAME="certkafkadevnick006" \
 RUST_LOG=piilog_func cargo run
