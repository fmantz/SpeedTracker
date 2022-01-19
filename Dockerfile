##
## BUILD SECTION
##

FROM httpd:2.4
WORKDIR /speedtest
RUN apt-get -qq --yes update

# Packages needed to compile
RUN apt-get -qq --yes install wget build-essential g++ cmake
RUN apt-get -qq --yes install libcurl4-openssl-dev libxml2 libxml2-dev libssl-dev

# Install RUST:
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.58.0

RUN set -eux; \
    dpkgArch="$(dpkg --print-architecture)"; \
    case "${dpkgArch##*-}" in \
        amd64) rustArch='x86_64-unknown-linux-gnu'; rustupSha256='3dc5ef50861ee18657f9db2eeb7392f9c2a6c95c90ab41e45ab4ca71476b4338' ;; \
        armhf) rustArch='armv7-unknown-linux-gnueabihf'; rustupSha256='67777ac3bc17277102f2ed73fd5f14c51f4ca5963adadf7f174adf4ebc38747b' ;; \
        arm64) rustArch='aarch64-unknown-linux-gnu'; rustupSha256='32a1532f7cef072a667bac53f1a5542c99666c4071af0c9549795bbdb2069ec1' ;; \
        i386) rustArch='i686-unknown-linux-gnu'; rustupSha256='e50d1deb99048bc5782a0200aa33e4eea70747d49dffdc9d06812fd22a372515' ;; \
        *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    url="https://static.rust-lang.org/rustup/archive/1.24.3/${rustArch}/rustup-init"; \
    wget "$url"; \
    echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION --default-host ${rustArch}; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

# Build rust application
COPY ./ ./
RUN cargo build --release

# Build c++ application
RUN cd ./SpeedTest && cmake -DCMAKE_BUILD_TYPE=Release . && make

##
## FINAL IMAGE SECTION
##

#Image step:
FROM httpd:2.4
WORKDIR /root/
RUN apt-get -qq --yes update

RUN mkdir /root/data
RUN mkdir /root/log

RUN chown root:root /usr/local/apache2/htdocs/index.html
RUN chmod u+w /usr/local/apache2/htdocs/index.html
RUN chmod u+w /usr/local/apache2/htdocs/

COPY --from=0 /speedtest/SpeedTest/speedtestJson ./
COPY --from=0 /speedtest/target/release/speedtracker ./

COPY --from=0 /speedtest/docker_files/template.html ./
COPY --from=0 /speedtest/docker_files/speedtracker.toml ./
COPY --from=0 /speedtest/docker_files/iwgetid  /usr/sbin/iwgetid

COPY --from=0 /speedtest/jslibs/*.js /usr/local/apache2/htdocs/
