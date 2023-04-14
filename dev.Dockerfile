FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye 
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh 
CMD ["python3", "-m", "http.server", "--directory", "docs"]
 