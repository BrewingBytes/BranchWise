FROM ubuntu:22.04

RUN apt update
RUN apt install -y libwebkit2gtk-4.1-dev \
        build-essential \
        curl \
        wget \
        file \
        libxdo-dev \
        libssl-dev \
        libayatana-appindicator3-dev \
        librsvg2-dev \
        git \
        cmake

RUN curl -sL https://deb.nodesource.com/setup_22.x | bash
RUN apt install -y nodejs
RUN npm install --global yarn

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup update
