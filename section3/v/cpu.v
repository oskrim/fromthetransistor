`timescale 1ns / 1ps

module cpu (
    // verilator lint_off UNUSEDSIGNAL
    // verilator lint_off UNDRIVEN
    input wire                     clk,
    input wire                     i_reset,
    input wire                     i_running,
    input wire  [31:0]             rd_data,
    output wire [31:0]             rd_addr,
    output wire [31:0]             wr_data,
    output wire [31:0]             wr_addr,
    output wire                    wr_valid
    // verilator lint_on UNUSEDSIGNAL
    // verilator lint_on UNDRIVEN
  );

  // verilator lint_off UNUSEDSIGNAL
  reg [31:0] r_rd_addr;
  reg [31:0] r_wr_addr;
  // verilator lint_on UNUSEDSIGNAL
  reg        r_wr_valid;

  initial begin
    r_rd_addr = 0;
    r_wr_addr = 0;
  end

  always @(posedge clk)
    if (r_wr_valid)
      r_wr_valid <= 0;

  always @(posedge clk)
    if (i_reset)
    begin
      r_rd_addr <= 32'h00000000;
      r_wr_addr <= 32'h00000000;
      r_wr_valid <= 1'b0;
    end
    else if (i_running)
    begin
      r_rd_addr <= rd_addr + 32'h00000004;
      r_wr_addr <= wr_addr + 32'h10000000;  // write to uart
      r_wr_valid <= i_running;
    end
endmodule
