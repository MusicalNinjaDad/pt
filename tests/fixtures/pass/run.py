import pathlib


def test_passes():
    assert True


if __name__ == "__main__":
    from traceback import TracebackException
    import sys

    print("UID test_passes RUNNING")
    try:
        test_passes()
    except Exception:
        TracebackException.from_exception(sys.exception(), capture_locals=True).print(file=sys.stdout)
        print("UID test_passes FAIL")
    else:
        print("UID test_passes PASS")
