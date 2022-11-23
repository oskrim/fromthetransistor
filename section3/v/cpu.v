`timescale 1ns / 1ps

module cpu (
    // verilator lint_off UNUSEDSIGNAL
    // verilator lint_off UNDRIVEN
    input wire                     clk,
    input wire                     i_reset,
    input wire  [31:0]             rd_data,
    output wire [31:0]             rd_addr,
    output wire [31:0]             wr_data,
    output wire [31:0]             wr_addr,
    output wire                    rd_valid
    // verilator lint_on UNUSEDSIGNAL
    // verilator lint_on UNDRIVEN
  );
endmodule
