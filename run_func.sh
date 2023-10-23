cargo build
if [ $? -eq 0 ]; then
    echo "Build succeeded"
    func start --verbose
else
    echo "Build failed"
fi
