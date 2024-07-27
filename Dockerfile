FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# use light weight debian image
FROM debian:buster-slim

# installing dependencies
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

#directory setup
COPY --from=builder /usr/src/app/target/release/final_project /usr/local/bin/final_project
# prep cert for db operations
COPY ca-certificate.crt /etc/ssl/certs/ca-certificate.crt
# set the env var for cert using provided cert
ENV DB_CA_CERT=/etc/ssl/certs/ca-certificate.crt

# port to open for project
EXPOSE 8080

CMD ["final_project"]
