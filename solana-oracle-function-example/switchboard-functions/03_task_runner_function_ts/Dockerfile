# syntax=docker/dockerfile:1.4

###############################################
## Builder Image
###############################################
FROM switchboardlabs/gramine AS builder

RUN mkdir -p /sgx/nodejs

WORKDIR /home/root/basic-oracle-example

COPY package.json package-lock.json ./
RUN npm install
COPY ./tsconfig.json ./tsconfig.json
COPY ./src/ ./src/
RUN npm run build
RUN cp ./dist/index.js /sgx/nodejs/index.js

###############################################
## Final Image
###############################################
FROM switchboardlabs/gramine
COPY --from=builder --chown=root:root /sgx/nodejs/ /sgx/nodejs/

# Add startup files, user can overwrite these
COPY --chown=root:root ./configs/boot.sh /boot.sh
COPY --chown=root:root ./configs/get_measurement.sh /get_measurement.sh
RUN chmod a+x /get_measurement.sh && \
    chmod a+x /boot.sh

COPY --chown=root:root ./configs/nodejs.manifest.template /sgx/nodejs.manifest.template

# Get the measurement from the enclave
RUN rm -f /measurement.txt && \
    /get_measurement.sh && \
    cat /measurement.txt

ENTRYPOINT ["/bin/bash", "/boot.sh"]
