#include "Vtop.h"
#include "verilated.h"

void clock_tb(Vtop &tb) {
  tb.clk = 1;
  tb.eval();
  tb.clk = 0;
  tb.eval();
}

void reset_tb(Vtop &tb) {
  tb.i_reset = 1;
  clock_tb(tb);
  clock_tb(tb);
  tb.i_reset = 0;
}

int	main(int argc, char **argv) {
  VerilatedContext *contextp;
	contextp = new VerilatedContext;
	contextp->commandArgs(argc, argv);

	Vtop tb{contextp};
  reset_tb(tb);

  constexpr unsigned steps = 100;
  constexpr unsigned n = 9;
  unsigned i = 0;
  unsigned k = 0;
  unsigned readk = 0xf;
  unsigned bauds = 868;
  unsigned bits[n] = {0, 1, 0, 1, 0, 1, 0, 1, 1};
  unsigned buf[n]  = {0};

  for (unsigned i = 0; i < bauds*steps; i++) {
    clock_tb(tb);

    if (!((i + 1) % (bauds * 30))) {
      for (unsigned j = 0; j < n; j++) {
        printf("buf[%i]: 0x%x, bits[%i]: 0x%x\n", j, buf[j], j, bits[j]);
        assert(buf[j] == bits[j]);
      }
      for (unsigned j = 0; j < n; j++) buf[j] = 0;
      k = 0;
    }

    if (!(i % bauds)) {
      tb.uart_txd_in = bits[k];
      k = k < (n - 1) ? k + 1 : k;

      if (readk == n) readk = 0xf;
      if (!tb.uart_rxd_out && readk == 0xf) readk = 0;
      if (readk < 0xf) buf[readk++] = tb.uart_rxd_out;

      printf("uart_rxd_out: 0x%x\n", tb.uart_rxd_out);
      //printf("r_data: 0x%x, out_bit_rx: 0x%x, out_bit_tx: 0x%x\n", tb.out_data, tb.out_bit_rx, tb.out_bit_tx);
    }
  }
}
