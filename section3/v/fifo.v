`timescale 1ns / 1ps

module fifo #(
    parameter W    = 8,
    parameter LOGD = 7
  ) (
    input wire           clk,
    input wire           i_reset,
    input wire           i_wr,
    input wire           i_rd,
    input wire [(W-1):0] i_data,

    output wire [(W-1):0] o_data,
    output wire           o_full,
    output wire           o_empty
  );

  reg [(W-1):0]    mem [0:((1<<LOGD)-1)];
  reg [(LOGD-1):0] wr_addr;
  reg [(LOGD-1):0] rd_addr;

  wire w_rd;
  wire w_wr;

  assign o_data  = mem[rd_addr];
  assign o_full  = wr_addr == (rd_addr - 1);
  assign o_empty = wr_addr == rd_addr;
  assign w_wr   = i_wr && !o_full;
  assign w_rd   = i_rd && !o_empty;

  // reads
  initial rd_addr = 0;
  always @(posedge clk)
    if (i_reset)
      rd_addr <= 0;
    else if (w_rd)
      rd_addr <= rd_addr + 1;

  // writes
  initial wr_addr = 0;
  always @(posedge clk)
    if (i_reset)
      wr_addr <= 0;
    else if (w_wr)
      wr_addr <= wr_addr + 1;

  always @(posedge clk)
    if (w_wr)
      mem[wr_addr] <= i_data;
endmodule
