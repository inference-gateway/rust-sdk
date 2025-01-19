FROM mcr.microsoft.com/devcontainers/rust:1.0.20-bookworm 

ENV ZSH_CUSTOM=/home/vscode/.oh-my-zsh/custom

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libssl-dev \
    musl-tools && \
    rustup target add \
    aarch64-unknown-linux-musl \
    x86_64-unknown-linux-musl && \
    cd /tmp && \
    wget https://musl.cc/aarch64-linux-musl-cross.tgz && \
    tar -xzf aarch64-linux-musl-cross.tgz && \
    mv aarch64-linux-musl-cross /opt/ && \
    rm -rf aarch64-linux-musl-cross.tgz && \
    wget https://musl.cc/x86_64-linux-musl-cross.tgz && \
    tar -xzf x86_64-linux-musl-cross.tgz && \
    mv x86_64-linux-musl-cross /opt/ && \
    rm -rf x86_64-linux-musl-cross.tgz

# Install powerlevel10k theme
RUN git clone --depth=1 https://github.com/romkatv/powerlevel10k.git ${ZSH_CUSTOM}/themes/powerlevel10k

USER vscode
