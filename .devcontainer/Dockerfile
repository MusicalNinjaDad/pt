FROM ghcr.io/musicalninjas/pyo3-devcontainer:latest@sha256:601b51f518504c3e0e4e668eea7d3213a65cf9e3d4f040d6d917c9269643e4f7

ENV UV_SYSTEM_PYTHON=true

COPY --from=ghcr.io/astral-sh/uv:0.10.8 /uv /uvx /bin/

RUN uv tool install pytest
