#include "Vtxfifotest.h"
#include "Vtxfifotest___024root.h"
#include "verilated.h"

void clock_tb(Vtxfifotest &tb) {
  tb.clk = 1;
  tb.eval();
  tb.clk = 0;
  tb.eval();
}

void reset_tb(Vtxfifotest &tb) {
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

	Vtxfifotest tb{contextp};
  reset_tb(tb);

  constexpr unsigned steps = 200;
  constexpr unsigned n = 9;
  constexpr unsigned m = 12;
  unsigned i = 0;
  unsigned k = 0;
  unsigned bit = 0;
  unsigned readbit = 0;
  unsigned readk = 0xf;
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
        assert(tb.rootp->txfifotest__DOT__txi__DOT__txfifoi__DOT__wr_addr == (bit % 8));
      }
    }

    if (!((i + bauds / 2) % bauds)) {
      if (readk == n) {
        readk = 0xf;

        for (unsigned j = 0; j < n; j++) {
          printf("buf[%i]: 0x%x, bits[%i]: 0x%x\n", j, buf[j], j, bits[readbit][j]);
          assert(buf[j] == bits[readbit][j]);
        }
        assert(tb.uart_rxd_out);

        for (unsigned j = 0; j < n; j++) buf[j] = 0;
        readbit++;
      }
      if (!tb.uart_rxd_out && readk == 0xf) {
        readk = 0;
      }
      if (readk < 0xf) {
        buf[readk++] = tb.uart_rxd_out;
      }
    }
  }

  assert(tb.rootp->txfifotest__DOT__txi__DOT__txfifoi__DOT__wr_addr == (m % 8));
  assert(tb.rootp->txfifotest__DOT__txi__DOT__txfifoi__DOT__rd_addr == (m % 8));
  assert(tb.rootp->txfifotest__DOT__txi__DOT__empty);
  assert(!tb.rootp->txfifotest__DOT__txi__DOT__full);
  printf("txfifotest pass\n");
}
