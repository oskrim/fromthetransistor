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
    input  wire [31:0]             pc_data,
    output wire [31:0]             pc_addr,
    output wire                    wr_valid
    // verilator lint_on UNUSEDSIGNAL
    // verilator lint_on UNDRIVEN
  );

  assign pc_addr = pc;

  // verilator lint_off UNUSEDSIGNAL
  reg [31:0] pc;
  reg [31:0] r_rd_addr;
  reg [31:0] r_wr_addr;
  reg [31:0] regfile [15:0] /*verilator public_flat_rd*/;
  reg        r_wr_valid;

  // 0xE3A01041
  wire [31:0] insn;
  wire [11:0] op2;
  wire [3:0]  rd;
  wire [3:0]  rn;
  wire        op_s;
  wire [3:0]  opcode /* verilator public_flat_rd */;
  wire        op_i;
  wire [3:0]  cond;
  // verilator lint_off UNUSEDSIGNAL
  assign insn   = pc_data;
  assign op2    = insn[11:0];
  assign rd     = insn[15:12];
  assign rn     = insn[19:16];
  assign op_s   = insn[20];
  assign opcode = insn[24:21];
  assign op_i   = insn[25];
  assign cond   = insn[31:28];

  always @(posedge clk)
    if (r_wr_valid)
      r_wr_valid <= 0;

  always @(posedge clk)
    if (i_reset)
    begin
      r_rd_addr  <= 32'h00000000;
      r_wr_addr  <= 32'h00000000;
      pc         <= 32'h00000000;
      r_wr_valid <= 0;
    end
    else if (i_running)
      pc <= pc + 4;

  wire [31:0] str_value;
  wire [7:0] str_shift;
  assign str_value = op_i ? { 24'b0, op2[7:0] } : regfile[op2[3:0]];
  assign str_shift = op_i ? { 4'b0, op2[11:8] } : op2[11:4];

  always @(posedge clk)
    if (i_running)
    begin
      case (opcode)
        4'b1101: /* mov */ regfile[rd] <= str_value << str_shift;
        default: begin end
      endcase
    end
endmodule
