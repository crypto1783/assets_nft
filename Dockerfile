FROM ubuntu:20.04
WORKDIR /home
COPY target/release/node-template .
RUN apt-get update && apt-get install -y curl && chmod +x ./node-template
ENTRYPOINT ["./node-template"]