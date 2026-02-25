import pathlib


def seven():
    return 7


def test_fails():
    assert False


def test_passes():
    assert True


def test_seven_is_six():
    s = seven()
    assert s == 6


if __name__ == "__main__":
    from traceback import TracebackException
    import sys

    print("UID test_fails RUNNING")
    try:
        test_fails()
    except Exception:
        TracebackException.from_exception(sys.exception(), capture_locals=True).print(file=sys.stdout)
        print("UID test_fails FAIL")
    else:
        print("UID test_fails PASS")

    print("UID test_passes RUNNING")
    try:
        test_passes()
    except Exception:
        TracebackException.from_exception(sys.exception(), capture_locals=True).print(file=sys.stdout)
        print("UID test_passes FAIL")
    else:
        print("UID test_passes PASS")

    print("UID test_seven_is_six RUNNING")
    try:
        test_seven_is_six()
    except Exception:
        TracebackException.from_exception(sys.exception(), capture_locals=True).print(file=sys.stdout)
        print("UID test_seven_is_six FAIL")
    else:
        print("UID test_seven_is_six PASS")
