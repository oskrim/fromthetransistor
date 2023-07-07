`timescale 1ns / 1ps

module top #() (
    input wire            clk,
    input wire		        i_reset,

    input wire            uart_txd_in,
    output wire           uart_rxd_out
  );

  cputest cputesti (
    clk,
    i_reset,
    uart_txd_in,
    uart_rxd_out
  );
endmodule
