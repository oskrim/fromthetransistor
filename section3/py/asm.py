#!/usr/bin/env python3
#
# ARMv7 assembler
#
import os
import sys
import struct

def parse_reg(reg):
  if reg.startswith('r'):
    return int(reg[1:])
  elif reg == 'lr':
    return 14
  elif reg == 'sp':
    return 13


def parse_int(s):
  if s[0] == '#':
    return parse_int(s[1:])
  elif s[0] == '0' and s[1] == 'x':
    return int(s[2:], 16)
  else:
    return int(s)


def mov(dst_reg, op2):
  src_reg = parse_reg(op2)
  if src_reg:
    return 0xE1A00000 | (parse_reg(dst_reg) << 12) | src_reg
  else:
    return 0xE3A00000 | (parse_reg(dst_reg) << 12) | parse_int(op2)


def bx(reg):
  reg = parse_reg(reg)
  if reg:
    return 0xE12FFF10 | reg


def parse(f, out):
  fns = {}
  buf = bytes()
  i = 0
  for line in f:
    line = line.strip().lower()
    insn = None

    if line.startswith('@'):
      continue

    # TODO: handle directives
    if line.startswith('.'):
      continue

    if line.endswith(':'):
      fns[i] = line[:-1]
      continue

    words = line.split(None, 1)
    asm_insn = words.pop(0).strip()
    if not len(words):
      raise Exception('invalid instruction %s' % line)
    ops = words.pop(0).strip()

    match asm_insn:
      case 'mov':
        ops = [x.strip() for x in ops.split(',')]
        if len(ops) != 2:
          raise Exception('invalid mov instruction %s' % line)
        dst = ops[0]
        op2 = ops[1]
        insn = mov(dst, op2)

      case 'bx':
        insn = bx(ops)

    if not insn:
      raise Exception('invalid instruction %s' % line)
    out.write(struct.pack('<I', insn))
    i += 1


if __name__ == "__main__":
  if len(sys.argv) != 2:
    print("Usage: asm.py <filename>")
    sys.exit(1)
  with open(sys.argv[1], 'r') as f:
    new_fpath = os.path.splitext(sys.argv[1])[0] + '.bin'
    with open(new_fpath, 'wb') as f2:
      parse(f, f2)
