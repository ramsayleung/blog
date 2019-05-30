FROM rustlang/rust:nightly

ADD https://github.com/ramsayleung/blog/archive/master.tar.gz /usr/local/src/blog-master.tar.gz
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
WORKDIR /usr/local/src/
RUN tar xvfz blog-master.tar.gz 
RUN rustup default nightly
RUN cargo install diesel_cli --no-default-features --features postgres
# RUN diesel migration run
RUN ls -alrt
RUN cd blog-master
RUN ls -alrt
RUN cargo build --release
EXPOSE 8000

CMD ["cargo run"]