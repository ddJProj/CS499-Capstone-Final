FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release


# use ubuntu image? libssl.so.3 missing from buster-deb
FROM ubuntu:22.04

# installing dependencies
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

ENV RUST_LOG=debug

#directory setup
COPY --from=builder /usr/src/app/target/release/final_project /usr/src/app/final_project
# try just importing from repo
COPY --from=builder /usr/src/app/ca-certificate.crt /usr/src/app/ca-certificate.crt
# list dir
RUN ls -la 

RUN chmod 644 /usr/src/app/ca-certificate.crt && \
    echo "CERT CONTENT:" && \
    cat /usr/src/app/ca-certificate.crt && \
    echo "CERT PERMS:" && \
    ls -l /usr/src/app/ca-certificate.crt


# set the env var for cert using provided cert
ENV DB_CA_CERT=/usr/src/app/ca-certificate.crt

# port to open for project
EXPOSE 8080

CMD ["final_project"]
