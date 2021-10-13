import unittest

from crsplus import hello

def test_hello():
    assert hello() == 'Hello'


if __name__ == '__main__':
    unittest.main()
