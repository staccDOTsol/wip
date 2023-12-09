# syntax=docker/dockerfile:1.4
FROM switchboardlabs/gramine AS builder
SHELL ["/bin/bash", "-ec"]
WORKDIR /home/root/basic-oracle-example

###############################################################
### Bun SETUP
###############################################################
RUN curl -fsSL https://bun.sh/install | bash
ENV PATH="/root/.bun/bin:${PATH}"

COPY package.json tsconfig.json bun.lockb ./
RUN bun install

COPY ./src/ ./src/
RUN bun build --compile --target=bun --outfile=bun-function ./src/index.ts && \
    mv ./bun-function /sgx/bun-function

FROM switchboardlabs/gramine
WORKDIR /sgx/bun

# Copy the bun files
COPY --from=builder --chown=root:root /sgx/bun-function /sgx/bun-function

# Add startup files, user can overwrite these
COPY --chown=root:root ./configs/boot.sh /boot.sh
COPY --chown=root:root ./configs/get_measurement.sh /get_measurement.sh
RUN chmod a+x /get_measurement.sh && \
    chmod a+x /boot.sh

COPY --chown=root:root ./configs/bun.manifest.template /sgx/bun.manifest.template

# Get the measurement from the enclave
RUN rm -f /measurement.txt && \
    /get_measurement.sh && \
    cat /measurement.txt

ENTRYPOINT ["/bin/bash", "/boot.sh"]
