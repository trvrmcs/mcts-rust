FROM rust:1.67  
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh 
WORKDIR /app
COPY Cargo.toml Cargo.lock /app/ 
# annoying tricks to speed up build
COPY ./dummy-src /app/src
RUN wasm-pack build --target web 

RUN rm -rf /app/src 
RUN mv /app/pkg /app/pkg.dummy
COPY ./src /app/src

# seems to be necessary
RUN touch /app/src/lib.rs


CMD ["wasm-pack", "build", "--target", "web"]


