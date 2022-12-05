import io
import struct
from asm import *


def test_insn():
  assert mov('r1', '#0x41') == 0xE3A01041
  assert mov('r1', 'r2') == 0xE1A01002


def test_parse():
  code = '''
    @ comment
    mov r1, #0x41
    mov r2, r1
    bx r2
    str r1, [r2, #4]
    ldr r3, [r3, #8]
  '''
  encoded = [
    0xE3A01041,
    0xE1A02001,
    0xE12FFF12,
    0xE5821004,
    0xE5933008,
  ]
  expected = bytes()
  for insn in encoded:
    expected += struct.pack('<I', insn)
  fin = io.StringIO(code)
  fout = io.BytesIO()
  parse(fin, fout)
  assert(fout.getvalue() == expected)


def test_simple():
  with open('../fixtures/simple.s', 'r') as f:
    fout = io.BytesIO()
    parse(f, fout)
    assert(len(fout.getvalue()) == 8)
    assert(fout.getvalue() == b'\x42\x00\xa0\xe3' + struct.pack("<I", 0xE12FFF1E))
