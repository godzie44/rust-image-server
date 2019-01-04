FROM rust:1.31.1-stretch as img-pool-build
USER root

RUN mkdir -p img-pool
COPY ./ ./img-pool

#install opencv deps
RUN apt-get update \
&& apt-get -y install build-essential \
&& apt-get -y install wget unzip cmake git libgtk2.0-dev pkg-config libavcodec-dev libavformat-dev libswscale-dev libjpeg-dev libpng-dev

#install opencv
RUN wget https://github.com/opencv/opencv/archive/3.4.3.zip \
&& unzip 3.4.3.zip && cd opencv-3.4.3 \
&& mkdir build && cd build \
&& cmake -D CMAKE_BUILD_TYPE=Release -D CMAKE_INSTALL_PREFIX=/usr/local .. \
&& make -j7 \
&& make install \
&& sh -c 'echo "/usr/local/lib" >> /etc/ld.so.conf.d/opencv.conf' \
&& ldconfig \
&& cd ../.. \
&& rm 3.4.3.zip \
&& rm -r opencv-3.4.3


RUN cd /img-pool && cargo build --release

EXPOSE 8088

RUN mkdir -p /build-out && mkdir -p /uploads
RUN cp /img-pool/target/release/image_pool /build-out/

USER root
CMD ["./build-out/image_pool"]

#билд: docker build -t imgpool .
#тесты: docker exec -it imgpool bash -c "cd \img-pool && cargo test"
#тесты опенцв: docker exec -it imgpool bash -c "cd \img-pool/opencv && cargo test"
#запуск docker run --rm --name imgpool --network="host" -v "$PWD/uploads":/uploads imgpool