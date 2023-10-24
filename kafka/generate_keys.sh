openssl req -new -newkey rsa:4096 -days 365 -x509 -subj "/CN=Demo-Kafka" -keyout ca-key -out ca-cert -nodes

keytool -genkey -keyalg RSA -keystore kafka.server.keystore.jks -validity 365 -storepass password -keypass password -dname "CN=preedee.space" -storetype pkcs12
# verify certificate
keytool -list -v -keystore kafka.server.keystore.jks

keytool -keystore kafka.server.keystore.jks -certreq -file cert-file -storepass password -keypass password


keytool -keystore kafka.server.keystore.jks -alias CARoot -import -file ca-cert -storepass password -keypass password -noprompt


keytool -keystore kafka.server.keystore.jks -import -file cert-file-signed -storepass password -keypass password -noprompt


keytool -keystore kafka.server.truststore.jks -alias CARoot -import -file ca-cert -storepass password -keypass password -noprompt


