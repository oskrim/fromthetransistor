`timescale 1ns / 1ps

module uart (
    input wire                     clk,
    input wire                     i_reset,
    input wire                     wr_valid,
    // verilator lint_off UNUSEDSIGNAL
    input wire  [31:0]             wr_data,
    input wire  [31:0]             wr_addr,
    // verilator lint_on UNUSEDSIGNAL
    output wire                    uart_rxd_out
  );

  wire   tx_valid;
  assign tx_valid = wr_valid && wr_addr[31];

  tx txi (
    clk,
    i_reset,
    tx_valid,
    wr_data[7:0],
    uart_rxd_out
  );
endmodule
