import pathlib


def test_fails():
    assert False


def test_passes():
    assert True


if __name__ == "__main__":
    import traceback

    try:
        test_fails()
    except Exception:
        traceback.print_exc()

    try:
        test_passes()
    except Exception:
        traceback.print_exc()
