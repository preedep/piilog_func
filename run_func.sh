cargo build
if [ $? -eq 0 ]; then
    echo "Build succeeded"
    export PII_LOG_ENDPOINT="xx"
    export PII_LOG_KEY_VAULT_ACCOUNT="xxx"
    export PII_LOG_KEY_VAULT_KEY_NAME="xxxx"
    func start --verbose
else
    echo "Build failed"
fi
