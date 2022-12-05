`timescale 1ns / 1ps

module cpu (
    // verilator lint_off UNUSEDSIGNAL
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
  );

  // verilator lint_off UNUSEDSIGNAL
  reg [31:0] pc;
  reg [31:0] r_rd_addr;
  reg [31:0] r_wr_addr;
  reg [31:0] r_wr_data;
  reg        r_wr_valid;
  reg [31:0] regfile [15:0] /*verilator public_flat_rd*/;


  wire [31:0] insn;
  wire [11:0] root;
  wire [3:0]  rd;
  wire [3:0]  rn;
  wire [3:0]  opcode;
  wire [3:0]  cond;
  // verilator lint_on UNUSEDSIGNAL

  assign pc_addr  = pc;
  assign insn     = pc_data;
  assign rd_addr  = r_wr_addr;
  assign wr_addr  = r_wr_addr;
  assign wr_data  = r_wr_data;
  assign wr_valid = r_wr_valid;

  assign root   = insn[11:0];
  assign rd     = insn[15:12];
  assign rn     = insn[19:16];
  assign opcode = insn[24:21];
  assign cond   = insn[31:28];

  always @(posedge clk)
    if (r_wr_valid)
      r_wr_valid <= 0;

  always @(posedge clk)
    if (i_reset)
    begin
      r_rd_addr  <= 32'h00000000;
      r_wr_addr  <= 32'h00000000;
      r_wr_valid <= 0;
    end

  always @(posedge clk)
    if (i_reset)
    begin
      pc <= 32'h00000000;
    end
    else if (i_running)
      pc <= pc + 4;

  // 4.5 data processing
  wire [31:0] str_value;
  wire [31:0] str_shift;
  assign str_value = insn[25] ? { 24'b0, root[7:0] } : regfile[root[3:0]];
  assign str_shift = insn[25] ? { 28'b0, root[11:8] } : { 24'b0, root[11:4] };

  always @(posedge clk)
    if (i_running && insn[27:26] == 2'b00)
    begin
      case (opcode)
        4'b1101: /* mov */ regfile[rd] <= str_value << str_shift;
        default: begin
          $display("Error [cpu.v]: Unknown opcode 0x%b", opcode); $fatal();
        end
      endcase
    end

  // 4.9 single  data transfer
  wire [31:0] addr;
  wire [31:0] offset;
  assign offset = { 20'b0, root };
  assign addr = insn[23] ? regfile[rn] + offset : regfile[rn] - offset;

  always @(posedge clk)
    if (i_running && insn[27:26] == 2'b01)
    begin
      if (insn[20]) // load
      begin
        r_rd_addr <= addr;
        regfile[rd] <= rd_data;
      end
      else // store
      begin
        r_wr_addr <= addr;
        r_wr_data <= regfile[rd];
        r_wr_valid <= 1;
      end
    end
endmodule
