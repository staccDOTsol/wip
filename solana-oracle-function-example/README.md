<div align="center">

![Switchboard Logo](https://github.com/switchboard-xyz/switchboard/raw/main/website/static/img/icons/switchboard/avatar.png)

</div>

This example shows how to receive data to your Solana program.

## Example #1: Basic Oracle

The basic oracle will push data to your contract at some pre-defined schedule.
You are responsible for storing this data yourself in some pre-defined struct.

**Build Switchboard Function (Rust)**

```bash
docker buildx build --pull --platform linux/amd64 \
    -f ./switchboard-functions/01_basic_oracle_function/Dockerfile \
    -t ${DOCKERHUB_ORGANIZATION}/solana-basic-oracle-function:latest \
    ./

# to publish

docker buildx build --pull --platform linux/amd64 \
    -f ./switchboard-functions/01_basic_oracle_function/Dockerfile \
    -t ${DOCKERHUB_ORGANIZATION}/solana-basic-oracle-function:latest \
    --push \
    ./

# to get the MRENCLAVE measurement
docker pull --platform=linux/amd64 ${DOCKERHUB_ORGANIZATION}/solana-basic-oracle-function:latest
docker run -d --platform=linux/amd64 -q \
    --name=my-switchboard-function \
    ${DOCKERHUB_ORGANIZATION}/solana-basic-oracle-function:latest
docker cp my-switchboard-function:/measurement.txt measurement.txt
echo -n 'MrEnclve: '
cat measurement.txt
docker stop my-switchboard-function
docker rm my-switchboard-function
```

**Build Switchboard Function (Typescript)**

```bash
docker buildx build --pull --platform linux/amd64 \
    -f ./switchboard-functions/01_basic_oracle_function_ts/Dockerfile \
    -t ${DOCKERHUB_ORGANIZATION}/solana-basic-oracle-function:typescript \
    ./switchboard-functions/01_basic_oracle_function_ts

# to publish

docker buildx build --pull --platform linux/amd64 \
    -f ./switchboard-functions/01_basic_oracle_function/Dockerfile \
    -t ${DOCKERHUB_ORGANIZATION}/solana-basic-oracle-function:typescript \
    --push \
    ./

# to get the MRENCLAVE measurement
docker pull --platform=linux/amd64 ${DOCKERHUB_ORGANIZATION}/solana-basic-oracle-function:typescript
docker run -d --platform=linux/amd64 -q \
    --name=my-switchboard-function \
    ${DOCKERHUB_ORGANIZATION}/solana-basic-oracle-function:typescript
docker cp my-switchboard-function:/measurement.txt measurement.txt
echo -n 'MrEnclve: '
cat measurement.txt
docker stop my-switchboard-function
docker rm my-switchboard-function
```
