FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release
# list dir
RUN ls -la 

# use ubuntu image? libssl.so.3 missing from buster-deb
FROM ubuntu:22.04

# installing dependencies
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

ENV RUST_LOG=debug

#directory setup
COPY --from=builder /usr/src/app/target/release/final_project /usr/local/bin/final_project
# prep cert for db operations
#COPY ca-certificate.crt /etc/ssl/certs/ca-certificate.crt
# is it permissions issue?

#JUST SET UP CERT WITH BASH SCRIPT
RUN echo '#!/bin/bash\n\
echo "$CA_CERTIFICATE_DATA" > /etc/ssl/certs/ca-certificate.crt\n\
chmod 644 /etc/ssl/certs/ca-certificate.crt\n\
echo "Certificate content:"\n\
cat /etc/ssl/certs/ca-certificate.crt\n\
echo "Certificate permissions:"\n\
ls -l /etc/ssl/certs/ca-certificate.crt\n\
exec "$@"' > /entrypoint.sh && chmod +x /entrypoint.sh





# set the env var for cert using provided cert
ENV DB_CA_CERT=/etc/ssl/certs/ca-certificate.crt

# port to open for project
EXPOSE 8080

CMD ["final_project"]
