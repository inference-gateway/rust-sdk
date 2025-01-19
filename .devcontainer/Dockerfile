FROM mcr.microsoft.com/devcontainers/rust:1.0.20-bookworm 

ENV ZSH_CUSTOM=/home/vscode/.oh-my-zsh/custom

RUN git clone --depth=1 https://github.com/romkatv/powerlevel10k.git ${ZSH_CUSTOM}/themes/powerlevel10k

USER vscode
