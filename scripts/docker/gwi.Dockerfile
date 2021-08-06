FROM paritytech/ci-linux:974ba3ac-20201006
WORKDIR /var/www/node-template
ADD ../../target/release/node-template .
ADD ../../gwiRaw.json .
RUN chmod +x ./node-template 
ENTRYPOINT ["node-template"]

