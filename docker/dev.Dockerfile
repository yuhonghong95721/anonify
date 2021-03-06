# inherit the baidu sdk image
FROM baiduxlab/sgx-rust:1804-1.1.2
MAINTAINER osuke
WORKDIR /root

RUN rm -rf /root/sgx && \
    echo 'source /opt/sgxsdk/environment' >> /root/.docker_bashrc && \
    echo 'source /root/.cargo/env' >> /root/.docker_bashrc

RUN set -x && \
    apt-get update && \
    apt-get upgrade -y --no-install-recommends && \
    apt-get install -y --no-install-recommends libzmq3-dev llvm clang-3.9 llvm-3.9-dev libclang-3.9-dev software-properties-common nodejs && \
    curl -o- -L https://yarnpkg.com/install.sh | bash && \
    export PATH="$HOME/.yarn/bin:$PATH" && \
    yarn global add ganache-cli && \
    rm -rf /var/lib/apt/lists/* && \
    curl -o /usr/bin/solc -fL https://github.com/ethereum/solidity/releases/download/v0.5.16/solc-static-linux && \
    chmod u+x /usr/bin/solc

RUN /root/.cargo/bin/cargo install bindgen cargo-audit && \
    rm -rf /root/.cargo/registry && rm -rf /root/.cargo/git && \
    git clone --depth 1 -b v1.1.2 https://github.com/baidu/rust-sgx-sdk.git sgx
