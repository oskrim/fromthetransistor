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

constexpr unsigned bauds = 868;
constexpr unsigned n_instr = 3;
unsigned instr[n_instr] = {
  0x0000A0E3,
  0x4110A0E3,
  0x001080E5,
};

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

unsigned get_bit(unsigned val, unsigned bit) {
  return (val >> bit) & 1;
}

unsigned write_instructions(Vcputest &tb) {
  unsigned inst = 0;
  unsigned byte = 0;
  unsigned bit  = 0;
  for (unsigned i = 0; i < (bauds*n_instr*4 + 1)*10; i++) {
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

void run_dry(Vcputest &tb) {
  for (unsigned i = 0; i < bauds*100; i++) clock_tb(tb);
}

void verify_mem(Vcputest &tb) {
  // ram
  for (unsigned i = 0; i < n_instr; i++) {
    if (tb.rootp->cputest__DOT__rami__DOT__mem[i] != instr[i]) {
      std::cout << "ram[" << std::hex << i << "] = " << std::hex << tb.rootp->cputest__DOT__rami__DOT__mem[i] << std::endl;
    }
    assert(tb.rootp->cputest__DOT__rami__DOT__mem[i] == instr[i]);
  }

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
  run_dry(tb);
  verify_mem(tb);

  printf("cputest pass\n");
}
