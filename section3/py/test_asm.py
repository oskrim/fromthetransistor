import io
import struct
from asm import *


def test_insn():
  assert mov('r1', '#0x41') == 0xE3A01041
  assert mov('r1', 'r2') == 0xE1A01002


# create in-memory files for input and output
def test_parse():
  fin = io.StringIO('@ comment\nmov r1, #0x41')
  fout = io.BytesIO()
  parse(fin, fout)
  assert(fout.getvalue() == b'\x41\x10\xa0\xe3')


def test_simple():
  with open('fixtures/simple.s', 'r') as f:
    fout = io.BytesIO()
    parse(f, fout)
    assert(len(fout.getvalue()) == 8)
    assert(fout.getvalue() == b'\x42\x00\xa0\xe3' + struct.pack("<I", 0xE12FFF1E))
