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
  tb.i_reset = 1;
  clock_tb(tb);
  clock_tb(tb);
  tb.i_reset = 0;
  tb.uart_txd_in = 1;
}

int	main(int argc, char **argv) {
  VerilatedContext *contextp;
	contextp = new VerilatedContext;
	contextp->commandArgs(argc, argv);

	Vcputest tb{contextp};
  reset_tb(tb);

  constexpr unsigned steps = 200;
  constexpr unsigned n = 9;
  constexpr unsigned m = 12;
  unsigned i = 0;
  unsigned k = 0;
  unsigned bit = 0;
  unsigned bauds = 868;
  unsigned bits[m][n] = {
    {0, 1, 1, 1, 0, 0, 0, 1, 0},
    {0, 1, 1, 1, 0, 1, 0, 0, 0},
    {0, 1, 1, 0, 0, 0, 1, 0, 0},
    {0, 0, 1, 0, 0, 0, 1, 0, 0},
    {0, 1, 0, 0, 1, 0, 1, 1, 1},
    {0, 1, 0, 1, 1, 0, 1, 1, 1},
    {0, 1, 0, 1, 1, 0, 1, 0, 1},
    {0, 1, 0, 0, 1, 0, 1, 0, 1},
    {0, 1, 1, 1, 1, 0, 1, 1, 1},
    {0, 1, 1, 0, 1, 0, 1, 1, 1},
    {0, 1, 0, 0, 0, 0, 0, 0, 1},
    {0, 1, 0, 0, 0, 1, 0, 0, 1}
  };
  unsigned buf[n]  = {0};

  // write into uart
  for (unsigned i = 0; i < bauds*steps; i++) {
    clock_tb(tb);

    if (!(i % bauds)) {
      if (k > n) {
        if (bit < m - 1) {
          bit++;
          k = 0;
        }
      } else {
        if (k == n) {
          tb.uart_txd_in = 1;
        } else {
          tb.uart_txd_in = bits[bit][k];
        }
        k++;
      }
    }
  }

  printf("cputest pass\n");
}
