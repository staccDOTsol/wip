# syntax=docker/dockerfile:1.4
FROM switchboardlabs/sgx-function AS builder

WORKDIR /home/root/basic-oracle-example
COPY package.json tsconfig.json package-lock.json ./
RUN npm install
COPY ./src/ ./src/
RUN npm run build
RUN cp ./dist/index.js /sgx/nodejs/index.js

FROM switchboardlabs/sgx-function

# Copy the binary
WORKDIR /sgx
COPY --from=builder /sgx/nodejs/index.js /sgx/nodejs/index.js

# Get the measurement from the enclave
RUN rm -f /measurement.txt && \
    /get_measurement.sh --nodejs && \
    cat /measurement.txt

ENTRYPOINT ["bash", "/boot.sh", "--nodejs"]
