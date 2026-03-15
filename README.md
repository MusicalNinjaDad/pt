# Pt

A drop-in replacement for pytest, written in rust. This is currently a _very sparse_ PoC - see Todos / Limitations.

6x faster when running simple tests

## Todos / Limitations

- Requires python >=3.13
- Doesn't handle:
  - Tests that fail due to an unexpected Exception
  - Parametrized tests
  - Fixtures & custom conftest.py
  - Tests that produce output to stdout/stderr
  - No tests found -> ExitCode 1
  - Test discovery across multiple files
  - Tests which are part of the package namespace (with `__init__.py`) rather than which expect `uv` / `pip install -e .` installation of the tested package
- Will fail fast if something goes wrong, rather than attempt to run as many tests as possible
- Errors won't always contain the best context details
