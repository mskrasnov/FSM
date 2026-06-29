# (C) 2026 Michail Krasnov <mskrasnov07@ya.ru>

FROM fedora:42

ENV PATH="/root/.cargo/bin:${PATH}"

COPY packaging/fedora/setup.sh /tmp/setup.sh

RUN chmod +x /tmp/setup.sh \
  && /tmp/setup.sh \
  && rm /tmp/setup.sh

WORKDIR /workspace

COPY . .

CMD ["bash"]
