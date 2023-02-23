#!/usr/bin/env python3
#
# ARMv7 assembler
#
import os
import sys
import struct

cond_table = {
  'eq': 0,
  'ne': 1,
  'cs': 2,
  'cc': 3,
  'mi': 4,
  'pl': 5,
  'vs': 6,
  'vc': 7,
  'hi': 8,
  'ls': 9,
  'ge': 10,
  'lt': 11,
  'gt': 12,
  'le': 13,
  'al': 14,
}

operation_codes = {
  'and': 0,
  'eor': 1,
  'sub': 2,
  'rsb': 3,
  'add': 4,
  'adc': 5,
  'sbc': 6,
  'rsc': 7,
  'tst': 8,
  'teq': 9,
  'cmp': 10,
  'cmn': 11,
  'orr': 12,
  'mov': 13,
  'bic': 14,
  'mvn': 15,
}


def get_cond(c):
  if c in cond_table:
    return cond_table[c]
  else:
    return 14


def get_opcode(op):
  if op in operation_codes:
    return operation_codes[op]
  else:
    raise Exception('invalid opcode %s' % op)


def strip_all(l):
  return [x.strip() for x in l]


def parse_reg(reg):
  if reg.startswith('r'):
    return int(reg[1:])
  elif reg == 'lr':
    return 14
  elif reg == 'sp':
    return 13


def parse_int(s):
  if isinstance(s, int):
    return s
  if s[0] == '#':
    return parse_int(s[1:])
  elif s[0:2] == '0x':
    return int(s[2:], 16)
  else:
    return int(s)


def int_to_imm(i):
  i = parse_int(i)
  if i < 0:
    raise Exception('invalid immediate %d' % i)
  highbits = i >> 8
  if highbits == 0:
    return i
  shift = 0
  while highbits:
    highbits >>= 1
    shift += 1
  if shift > 15:
    raise Exception('invalid immediate %d shift %d' % (i, shift))
  return (i >> shift) | (shift << 8)


def mov(dst_reg, op2, cond):
  src_reg = parse_reg(op2)
  if src_reg:
    return 0x01A00000 | (parse_reg(dst_reg) << 12) | src_reg | (cond << 28)
  else:
    return 0x03A00000 | (parse_reg(dst_reg) << 12) | int_to_imm(op2) | (cond << 28)


def bx(reg, cond):
  reg = parse_reg(reg)
  if reg:
    return 0x012FFF10 | reg | (cond << 28)


def data_processing3(base, dst_reg, op1_reg, op2, cond):
  opcode = get_opcode(base)
  src_reg = parse_reg(op2)
  if src_reg:
    return 0x00000000 | (parse_reg(dst_reg) << 12) | (parse_reg(op1_reg) << 16) | src_reg | (cond << 28) | (opcode << 21)
  else:
    return 0x02000000 | (parse_reg(dst_reg) << 12) | (parse_reg(op1_reg) << 16) | int_to_imm(op2) | (cond << 28) | (opcode << 21)


def cmpp(op1_reg, op2, cond):
  src_reg = parse_reg(op2)
  if src_reg:
    return 0x01500000 | (parse_reg(op1_reg) << 16) | src_reg | (cond << 28)
  else:
    return 0x03500000 | (parse_reg(op1_reg) << 16) | int_to_imm(op2) | (cond << 28)


def transfer(reg, base_reg, offset, cond, load=False):
  base = 0x05900000 if load else 0x05800000
  return base | (parse_reg(reg) << 12) | (parse_reg(base_reg) << 16) | int_to_imm(offset) | (cond << 28)

def parse(f, out, label={}, labels_only=False):
  buf = bytes()
  i = 0
  for line in f:
    line = line.strip().lower()
    insn = None

    if not line:
      continue

    if line.startswith('@'):
      continue

    # TODO: handle directives
    if line.startswith('.'):
      continue

    if line.endswith(':'):
      label[line[:-1]] = i
      continue

    words = line.split(None, 1)
    asm_insn = words.pop(0).strip()
    if not len(words):
      raise Exception('invalid instruction %s' % line)

    if not labels_only:
      if os.getenv('DEBUG'):
        print(line)

      ops = words.pop(0).strip()

      if len(asm_insn) == 3 or len(asm_insn) == 5 or len(asm_insn) == 1:
        base_insn = asm_insn[:3]
        cond = get_cond(asm_insn[3:])
        match base_insn:
          case 'mov':
            ops = strip_all(ops.split(','))
            if len(ops) != 2:
              raise Exception('invalid mov instruction %s' % line)
            dst = ops[0]
            op2 = ops[1]
            insn = mov(dst, op2, cond)

          case 'ldr' | 'str':
            # find the content between [], e.g. [r0, #4]
            ops = ops.split(',', 1)
            if len(ops) != 2:
              raise Exception('invalid ldr/str instruction %s' % line)
            reg = ops.pop(0).strip()
            ops = strip_all(ops[0].split('[', 1)[1].split(']', 1)[0].split(','))
            if len(ops) < 2:
              insn = transfer(reg, ops[0], 0, cond, asm_insn == 'ldr')
            else:
              insn = transfer(reg, ops[0], ops[1], cond, asm_insn == 'ldr')

          case 'sub' | 'add':
            ops = strip_all(ops.split(','))
            if len(ops) != 3:
              raise Exception('invalid sub instruction %s' % line)
            dst = ops[0]
            op1 = ops[1]
            op2 = ops[2]
            insn = data_processing3(base_insn, dst, op1, op2, cond)

          case 'cmp':
            ops = strip_all(ops.split(','))
            if len(ops) != 2:
              raise Exception('invalid cmp instruction %s' % line)
            op1 = ops[0]
            op2 = ops[1]
            insn = cmpp(op1, op2, get_cond(asm_insn[3:]))

          case 'b' | 'beq' | 'bne' | 'bcs' | 'bcc' | 'bmi' | 'bpl' | 'bvs' | 'bvc' | 'bhi' | 'bls' | 'bge' | 'blt' | 'bgt' | 'ble' | 'bal':
            cond = get_cond(asm_insn[1:])
            insn = 0x0A000000 | (label[ops] - i - 2) | (cond << 28)

      if len(asm_insn) == 4 or len(asm_insn) == 2:
        match asm_insn[:2]:
          case 'bl' | 'bleq' | 'blne' | 'blcs' | 'blcc' | 'blmi' | 'blpl' | 'blvs' | 'blvc' | 'blhi' | 'blls' | 'blge' | 'bllt' | 'blgt' | 'blle' | 'blal':
            cond = get_cond(asm_insn[2:])
            insn = 0x0B000000 | (label[ops] - i - 2) | (cond << 28)

          case 'bx':
            insn = bx(ops, get_cond(asm_insn[3:]))

      if not insn:
        raise Exception('invalid instruction %s' % line)
      out.write(struct.pack('<I', insn))

    i += 1


def main(f, f2):
  label = {}
  lines = f.readlines()
  parse(lines, f2, label, labels_only=True)
  parse(lines, f2, label)


if __name__ == "__main__":
  if len(sys.argv) != 2:
    print("Usage: asm.py <filename>")
    sys.exit(1)
  with open(sys.argv[1], 'r') as f:
    new_fpath = os.path.splitext(sys.argv[1])[0] + '.bin'
    with open(new_fpath, 'wb') as f2:
      main(f, f2)
