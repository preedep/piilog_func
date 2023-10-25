export DOCKER_KAFKA_HOST=$(ipconfig getifaddr en0)
echo $DOCKER_KAFKA_HOST

docker-compose up 
