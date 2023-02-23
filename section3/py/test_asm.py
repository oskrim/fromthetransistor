import io
import struct
from asm import *


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


def test_if_else():
  encoded = [
    0xe24dd008,
    0xe3a00000,
    0xe58d0004,
    0xe3a00380,
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
    for i in range(0, len(fout.getvalue()), 4):
      insn = struct.unpack('<I', fout.getvalue()[i:i+4])[0]
      if insn != encoded[i//4]:
        print('[%d] expected: %x, got: %x' % (i//4, encoded[i//4], insn))
        assert False
    assert(fout.getvalue() == b''.join([struct.pack('<I', insn) for insn in encoded]))
