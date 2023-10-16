FROM cgr.dev/chainguard/wolfi-base
RUN apk update && apk add libgcc
COPY --chown=nonroot:nonroot runo /usr/local/bin/runo
RUN chmod +x /usr/local/bin/runo
CMD ["/usr/local/bin/runo"]