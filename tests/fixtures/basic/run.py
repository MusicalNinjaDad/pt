import pathlib


def test_fails():
    assert False


def test_passes():
    assert True


if __name__ == "__main__":
    import traceback
    import sys

    print("UID test_fails RUNNING")
    try:
        test_fails()
    except Exception:
        print("UID test_fails FAIL")
        traceback.print_exc(file=sys.stdout)
    else:
        print("UID test_fails PASS")

    print("UID test_passes RUNNING")
    try:
        test_passes()
    except Exception:
        print("UID test_passes FAIL")
        traceback.print_exc(file=sys.stdout)
    else:
        print("UID test_passes PASS")
