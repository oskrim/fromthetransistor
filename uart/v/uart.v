`timescale 1ns / 1ps

module uart #(
    parameter                     BW = 9,
    parameter                     TIMER_BITS = 32,
    parameter [(TIMER_BITS-1):0]  CLOCKS_PER_BAUD = 868
  ) (
    input wire            clk,
    input wire            i_reset,

    output wire           led0_b,
    output wire           led3_r,

    output wire [(BW):0]  out_data,
    output wire [3:0]     out_bit_rx,
    output wire [3:0]     out_bit_tx,
    output wire           out_start_tx,

    input wire            uart_txd_in,
    output wire           uart_rxd_out
  );

  assign led0_b = uart_rxd_out;

  tx_uart #(BW, TIMER_BITS, CLOCKS_PER_BAUD) tx_uart_inst (
    clk,
    i_reset,
    out_start_tx,
    out_data,
    out_bit_tx,
    uart_rxd_out
  );

  rx_uart #(BW, TIMER_BITS, CLOCKS_PER_BAUD) rx_uart_inst (
    clk,
    i_reset,
    out_start_tx,
    led3_r,
    out_data,
    out_bit_rx,
    uart_txd_in
  );
endmodule
