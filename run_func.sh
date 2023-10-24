cargo build
if [ $? -eq 0 ]; then
    echo "Build succeeded"
    export PII_LOG_ENDPOINT="localhost:9092"
    export PII_LOG_KEY_VAULT_ACCOUNT="nicksecretstoredev001"
    export PII_LOG_KEY_VAULT_KEY_NAME="certkafkadevnick002"
    func start --verbose
else
    echo "Build failed"
fi
