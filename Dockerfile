FROM debian:stable-20210511-slim
WORKDIR ~

# Copy in the binaries
COPY ./src-tauri/melwalletd-* .
COPY ./src-tauri/target/release/ginkou .
COPY ./ziptar.sh .

# Zip up tar file
ENTRYPOINT "/bin/bash"
#CMD ["sh", "ziptar.sh"]
#CMD ["sh", "-c", "ls", "melwalletd*", "|", "tar", "-cvf", "ginkou.tar.gz", "ginkou", "-T", "-"]
