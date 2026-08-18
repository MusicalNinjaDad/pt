FROM ghcr.io/musicalninjas/pyo3-devcontainer:latest@sha256:eb6bac7f292e08393d8cdc1ada9bdd4a15cdf457688b40931468a3d97805e114

ENV UV_SYSTEM_PYTHON=true

COPY --from=ghcr.io/astral-sh/uv:0.10.8 /uv /uvx /bin/

RUN uv tool install pytest
