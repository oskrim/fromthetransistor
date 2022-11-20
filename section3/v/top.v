`timescale 1ns / 1ps

module top #(
    parameter             BW = 9
  ) (
    input wire            clk,
    input wire		        i_reset,

    output wire           led0_b,
    output wire           led3_r,

    input wire            uart_txd_in,
    output wire           uart_rxd_out
  );

  cpu #(BW, 32, 10416) cpui (
    clk,
    i_reset,
    led0_b,
    led3_r,
    fifo_data,
    out_bit_rx,
    out_bit_tx,
    out_start_tx,
    o_empty,
    out_state,
    out_wr_addr,
    out_rd_addr,
    uart_txd_in,
    uart_rxd_out
  );
endmodule
