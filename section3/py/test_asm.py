import io
import struct
from asm import *


def test_twos_complement():
  assert(reverse_twos_complement(0, 2) == 0)
  assert(reverse_twos_complement(1, 2) == 1)
  assert(reverse_twos_complement(2, 2) == 2)
  assert(reverse_twos_complement(3, 2) == 3)
  assert(reverse_twos_complement(-4, 2) == 4)
  assert(reverse_twos_complement(-3, 2) == 5)
  assert(reverse_twos_complement(-2, 2) == 6)
  assert(reverse_twos_complement(-1, 2) == 7)


def test_insn():
  assert mov('r1', '#0x41', 14) == 0xE3A01041
  assert mov('r1', 'r2', 13) == 0xD1A01002


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
  main(fin, fout)
  assert(fout.getvalue() == expected)


def test_simple():
  with open('../fixtures/simple.s', 'r') as f:
    fout = io.BytesIO()
    main(f, fout)
    assert(len(fout.getvalue()) == 8)
    assert(fout.getvalue() == b'\x42\x00\xa0\xe3' + struct.pack("<I", 0xE12FFF1E))


def insn_compare(out, expect):
  for i in range(0, len(out), 4):
    insn = struct.unpack('<I', out[i:i+4])[0]
    if type(expect) == bytes:
      expect_insn = struct.unpack('<I', expect[i:i+4])[0]
    else:
      expect_insn = expect[i//4]
    if insn != expect_insn:
      print('[%d]\texpected: %x, got: %x' % (i, expect_insn, insn))
      assert False
  if type(expect) == bytes:
    assert(out == expect)
  else:
    assert(out == b''.join([struct.pack('<I', insn) for insn in expect]))


def test_if_else():
  encoded = [
    0xe24dd008,
    0xe3a00000,
    0xe58d0004,
    0xe3a00240,
    0xe5900000,
    0xe58d0000,
    0xe350007b,
    0xda000002,
    0xe3a0002a,
    0xe58d0004,
    0xea000004,
    0xe59d0000,
    0xe3500063,
    0xe3a00014,
    0xd3a0000a,
    0xe58d0000,
    0xe28d0004,
    0xe28dd008,
    0xe12fff1e
  ]
  with open('../fixtures/if_else.s', 'r') as f:
    fout = io.BytesIO()
    main(f, fout)
    insn_compare(fout.getvalue(), encoded)


def test_while1():
  with open('../fixtures/while1.s', 'r') as f:
    fout = io.BytesIO()
    expect = b"\x04\xd0\x4d\xe2\x00\x00\xa0\xe3\x80\x1a\xa0\xe3\x00\x00\x8d\xe5\x40\x05\xa0\xe3\x80\x0a\x80\xe3\x00\x20\x90\xe5\x01\x20\x82\xe2\x00\x20\x81\xe5\xfb\xff\xff\xea"
    main(f, fout)
    insn_compare(fout.getvalue(), expect)
