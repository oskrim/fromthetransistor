#include <iostream>
#include "Vcputest.h"
#include "Vcputest___024root.h"
#include "verilated.h"

void clock_tb(Vcputest &tb) {
  tb.clk = 1;
  tb.eval();
  tb.clk = 0;
  tb.eval();
}

void reset_tb(Vcputest &tb) {
  tb.uart_txd_in = 1;
  tb.i_reset = 1;
  clock_tb(tb);
  clock_tb(tb);
  tb.i_reset = 0;
  clock_tb(tb);
  clock_tb(tb);
}

template <typename Arg, typename... Args>
void print(Arg&& arg, Args&&... args)
{
    std::cout << "0x" << std::hex;
    std::cout << static_cast<unsigned>(std::forward<Arg>(arg));
    using expander = int[];
    (void)expander{0, (void(std::cout << " 0x" << std::hex << static_cast<unsigned>(std::forward<Args>(args))), 0)...};
    std::cout << std::endl;
    std::flush(std::cout);
}

constexpr unsigned bauds = 868;
unsigned instr[10101] = {
  0xE3A00401,
  0xE3A01041,
  0xE5801000,
  0xE1A001B0,
  0xE5801000,
  0xFFFFFFFF,
};
unsigned n_instr = 0;

unsigned get_bit(unsigned val, unsigned bit) {
  return (val >> bit) & 1;
}

void write_instructions(Vcputest &tb) {
  while (instr[n_instr++] != 0xFFFFFFFF);

  unsigned inst = 0;
  unsigned byte = 0;
  unsigned bit  = 0;
  for (unsigned i = 0; i < (bauds*n_instr*4)*10; i++) {
    clock_tb(tb);

    if (!(i % bauds)) {
      if (bit >= 10) {
        if (byte == 3) {
          byte = 0;
          inst++;
        } else {
          byte++;
        }
        if (inst < n_instr) {
          bit = 0;
        }
      } else if (bit == 9) {
        tb.uart_txd_in = 1;
      } else if (bit) {
        tb.uart_txd_in = get_bit(instr[inst], byte * 8 + bit - 1);
      }
      if (bit == 0) {
        tb.uart_txd_in = 0;
      }
      bit++;
    }
  }
}

void run_program(Vcputest &tb) {
  unsigned k = 0;
  unsigned bit = 0xf;
  unsigned buf[100] = {0};
  unsigned i = 0;
  unsigned next = 0;
  while (i++ < bauds*10) {
    clock_tb(tb);
    if (!tb.uart_rxd_out && bit == 0xf) {
      next = i + bauds + bauds / 2;
      bit = 0;
    }
    if (i == next && bit < 8) {
      next = i + bauds;
      buf[k] |= tb.uart_rxd_out << bit;
      bit++;
      if (bit == 8) {
        bit = 0xf;
        k++;
      }
    }
  }
  assert(buf[0] == 0x41);
  assert(k == 1);
}

void verify_mem(Vcputest &tb) {
  // ram
  for (unsigned i = 0; i < n_instr; i++) {
    if (tb.rootp->cputest__DOT__rami__DOT__mem[i] != instr[i]) {
      std::cout << "ram[" << std::hex << i << "] = " << std::hex << tb.rootp->cputest__DOT__rami__DOT__mem[i] << std::endl;
    }
    assert(tb.rootp->cputest__DOT__rami__DOT__mem[i] == instr[i]);
  }

  // regs
  assert(tb.rootp->cputest__DOT__cpui__DOT__regfile[0] == 0x80000000);
  assert(tb.rootp->cputest__DOT__cpui__DOT__regfile[1] == 0x41);
  assert(tb.rootp->cputest__DOT__rami__DOT__mem[0x10] == 0x41);
  assert(tb.rootp->cputest__DOT__cpui__DOT__pc == 4*n_instr);
  assert(!tb.rootp->cputest__DOT__rami__DOT__r_running);

  // fifo
  assert(tb.rootp->cputest__DOT__uarti__DOT__txi__DOT__empty);
  assert(!tb.rootp->cputest__DOT__uarti__DOT__txi__DOT__full);
}

int	main(int argc, char **argv) {
  VerilatedContext *contextp;
	contextp = new VerilatedContext;
	contextp->commandArgs(argc, argv);
	Vcputest tb{contextp};

  reset_tb(tb);
  write_instructions(tb);
  run_program(tb);
  verify_mem(tb);

  printf("cputest pass\n");
}
