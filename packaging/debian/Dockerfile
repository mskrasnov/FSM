# (C) 2026 Michail Krasnov <mskrasnov07@ya.ru>

FROM debian:11

ENV DEBIAN_FRONTEND=noninteractive
ENV PATH="/root/.cargo/bin:${PATH}"

COPY packaging/debian/setup.sh /tmp/setup.sh

RUN chmod +x /tmp/setup.sh \
  && /tmp/setup.sh \
  && rm /tmp/setup.sh

WORKDIR /workspace

COPY . .

# ENTRYPOINT ["bash", "packaging/debian/appimage.sh"]
CMD ["bash"]
