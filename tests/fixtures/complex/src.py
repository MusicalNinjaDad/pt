import pathlib


def seven():
    return 7


def test_fails():
    assert False


def test_passes():
    assert True


def test_seven_is_six():
    assert seven() == 6
