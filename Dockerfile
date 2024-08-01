FROM rust:latest as builder
WORKDIR /usr/src/app
#copy full dir
COPY . .
RUN cargo build --release


# use ubuntu image? libssl.so.3 missing from buster-deb
FROM ubuntu:22.04

# installing dependencies
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

#directory setup
COPY --from=builder /usr/src/app/target/release/final_project /usr/src/app/final_project
# try just importing from repo
COPY --from=builder /usr/src/app/ca-certificate.crt /usr/src/app/ca-certificate.crt
# FIXME: COMMENT THIS LINE OUT BEFORE USE WITH DIGIOCEAN APP PLATFORM
# FIXME: MAKE SURE THE COMMENTED LINE IN DOCKIGNORE FOR CONFIG REMOVED IF SWAPPING TO PROD
COPY --from=builder /usr/src/app/config.toml /usr/src/app/config.toml


# list dir
RUN ls -la /usr/src/app

RUN chmod 644 /usr/src/app/ca-certificate.crt && \
    chmod +x /usr/src/app/final_project && \
    chmod 644 /usr/src/app/config.toml && \
    ls -l /usr/src/app/final_project
    ## USED TO TEST / GET BUILD WORKING FOR REMOTE ENV
    #echo "CERT CONTENTS:" && \
    #cat /usr/src/app/ca-certificate.crt && \
    #echo "CERT PERM:" && \
    #ls -l /usr/src/app/ca-certificate.crt && \
    #echo "EXEC PERM:" && \
    #ls -l /usr/src/app/final_project


ENV RUST_LOG=debug

# add to path
ENV PATH="/usr/src/app:${PATH}"

# set the env var for cert using provided cert
ENV DB_CA_CERT=/usr/src/app/ca-certificate.crt
ENV CONFIG_PATH=/usr/src/app/config.toml

# local docker image build, use for prof?
ENV ENVIRONMENT=local

# uncomment this line to switch to the remote / production build for pushing changes to repo / app platform
#BE SURE TO COMMENT local ENV line
#ENV ENVIRONMENT=production

# port to open for project
EXPOSE 8080

# full path?
CMD ["./final_project"]
