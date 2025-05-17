import pytest
import datem


def test_sum_as_string():
    assert datem.sum_as_string(1, 1) == "2"
