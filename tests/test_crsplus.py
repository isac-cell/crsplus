import unittest

from crsplus import hello, hi, __version__


class TestCRSPlus(unittest.TestCase):

    def test_version(self):
        self.assertEqual(__version__, '0.2.1')

    def test_hello(self):
        self.assertEqual(hello(), 'Hello')

    def test_hi_when_name_is_Python_then_Hi_Python(self):
        self.assertEqual(hi('Python'), 'Hi Python')

    def test_hi_when_name_is_Rust_then_Hi_Rust(self):
        self.assertEqual(hi('Rust'), 'Hi Rust')


if __name__ == '__main__':
    unittest.main()
