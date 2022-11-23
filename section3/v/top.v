`timescale 1ns / 1ps

module top #() (
    input wire            clk,
    input wire		        i_reset,

    output wire           led0_b,
    output wire           led3_r,

    input wire            uart_txd_in,
    output wire           uart_rxd_out
  );
endmodule
