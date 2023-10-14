# Crosscompilation to Raspberry Pi on RasiOS using system OpenCV
#
# Building this image requries `qemu-arm` to be present on the host system and the corresponding `binfmt-misc` set up (see
# e.g. https://wiki.debian.org/QemuUserEmulation, only `Installing packages` should be enough).
#
# After the successful build you will have an image configured for cross-compilation to Raspberry Pi. It will contain the
# sample build script `/usr/local/bin/cargo-xbuild` that you can check for the correct environment setup and the specific
# command line arguments to use when crosscompiling the project inside the container created from that image.


# Download and extract rpi root filesystem
FROM alpine:3.18

RUN set -xeu && \
    apk add xz

ADD https://downloads.raspberrypi.org/raspios_lite_arm64/root.tar.xz /

RUN set -xeu && \
    mkdir "/rpi-root" && \
    tar xpaf /root.tar.xz -C /rpi-root


# Prepare the root of the rpi filesystem, it's going to be used later for crosscompilation
# This step requries qemu-arm to be present on the host system and the corresponding binfmt-misc set up
FROM scratch

COPY --from=0 /rpi-root /

RUN set -xeu && \
    apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get dist-upgrade -y && \
    apt-get autoremove -y --purge && \
    apt-get -y autoclean

RUN set -xeu && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y symlinks

RUN set -xeu && \
    symlinks -cr /

# Specify dependencies that you need to have on rpi
RUN set -xeu && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y libudev-dev libsqlite3-dev libopencv-dev libstrophe-dev libcamera-dev pkg-config g++-aarch64-linux-gnu

RUN set -xeu && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y clang libclang-dev lld curl git build-essential pkg-config cmake

RUN set -xeu && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile=minimal && \
    rm -rf /root/.rustup/tmp/* # warning: could not delete temp directory: /root/.rustup/tmp/szyc3h06vricp83o_dir

ENV PATH="${PATH}:/root/.cargo/bin"
