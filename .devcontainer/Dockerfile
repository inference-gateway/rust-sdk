FROM mcr.microsoft.com/devcontainers/rust:1.0.20-bookworm 

ENV ZSH_CUSTOM=/home/vscode/.oh-my-zsh/custom

RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - && \
    apt-get update && \    
    apt-get install -y nodejs && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

RUN npm install -g semantic-release @semantic-release/changelog @semantic-release/exec @semantic-release/git @semantic-release/github conventional-changelog-conventionalcommits

RUN git clone --depth=1 https://github.com/romkatv/powerlevel10k.git ${ZSH_CUSTOM}/themes/powerlevel10k

USER vscode
