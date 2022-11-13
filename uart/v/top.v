`timescale 1ns / 1ps

module top #(
    parameter             BW=9
  ) (
    input wire            clk,
    input wire		        i_reset,

    output wire           led0_b,
    output wire           led3_r,

    input wire            uart_txd_in,
    output wire           uart_rxd_out
  );

  uart #(BW) uart_inst (
    clk,
    i_reset,
    led0_b,
    led3_r,
    out_data,
    out_bit_rx,
    out_bit_tx,
    out_start_tx,
    uart_txd_in,
    uart_rxd_out
  );
endmodule
