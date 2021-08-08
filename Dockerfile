FROM paritytech/ci-linux:974ba3ac-20201006
WORKDIR /home
COPY target/release/node-template .
RUN chmod +x ./node-template
ENTRYPOINT ["./node-template"]