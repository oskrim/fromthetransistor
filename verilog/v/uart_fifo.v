`timescale 1ns / 1ps

module uart_fifo #(
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

  wire       o_full;
  wire       o_empty;
  wire [7:0] fifo_data;
  wire       ready_tx;

  assign led0_b   = uart_rxd_out;
  assign ready_tx = out_bit_tx == 15;

  reg r_wr;
  reg r_rd;
  initial r_wr = 0;
  initial r_rd = 0;

  always @(posedge clk)
    if (r_wr)
      r_wr <= 0;
    else if (!r_wr && !o_full && out_start_tx)
      r_wr <= 1;

  always @(posedge clk)
    if (r_rd)
      r_rd <= 0;
    else if (!r_rd && !o_empty && ready_tx)
      r_rd <= 1;

  fifo #(8, 7) fifo_inst (
    clk,
    r_wr,
    r_rd,
    out_data[8:1],
    fifo_data,
    o_full,
    o_empty
  );

  tx_uart #(BW, TIMER_BITS, CLOCKS_PER_BAUD) tx_uart_inst (
    clk,
    i_reset,
    out_start_tx,
    {1'b1, fifo_data, 1'b0},
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
