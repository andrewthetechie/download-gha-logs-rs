
FROM scratch
ARG TARGETARCH

COPY download-gha-logs-$TARGETARCH/download-gha-logs /download-gha-logs

CMD ["/download-gha-logs"]
