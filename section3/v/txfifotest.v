`timescale 1ns / 1ps

module txfifotest #(
    parameter                     BW = 9,
    parameter                     TIMER_BITS = 32,
    parameter [(TIMER_BITS-1):0]  CLOCKS_PER_BAUD = 868
  ) (
    input wire            clk,
    input wire            i_reset,

    output wire            led0_b,
    output wire            tx_ready,
    output wire [(BW-2):0] tx_data,

    input wire            uart_txd_in,
    output wire           uart_rxd_out
  );

  wire tx_valid;

  assign led0_b = uart_rxd_out;

  tx #(BW, TIMER_BITS, CLOCKS_PER_BAUD) txi (
    clk,
    i_reset,
    tx_valid,
    tx_data,
    tx_ready,
    uart_rxd_out
  );

  rx_uart #(BW, TIMER_BITS, CLOCKS_PER_BAUD) rx_uarti (
    clk,
    i_reset,
    tx_valid,
    tx_data,
    uart_txd_in
  );
endmodule
