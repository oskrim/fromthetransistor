`timescale 1ns / 1ps

module ram #(
    parameter LOGD = 10
  ) (
    input wire                     clk,
    input wire                     i_reset,
    output wire                    cpu_running,

    // from uart, code download
    input wire                     rx_valid,
    input wire [7:0]               rx_data/* verilator public_flat_rd */,

    // verilator lint_off UNUSEDSIGNAL
    input wire  [31:0]             rd_addr,
    input wire  [31:0]             wr_addr,
    input wire  [31:0]             pc_addr,
    // verilator lint_on UNUSEDSIGNAL
    output wire [31:0]             rd_data,
    input wire  [31:0]             wr_data,
    output wire [31:0]             pc_data,
    input wire                     wr_valid
  );

  reg [31:0]  mem [0:((1<<LOGD)-1)] /* verilator public_flat_rd */;
  reg [31:0]  rx_addr;
  reg         r_running;

  wire [1:0]  rx_byte;
  wire [(LOGD-1):0] rx_word;

  assign rx_byte = rx_addr[1:0];
  assign rx_word = rx_addr[(LOGD+1):2];
  assign rd_data = mem[rd_addr >> 2];
  assign pc_data = mem[pc_addr >> 2];

  always @(posedge clk)
    if (wr_valid && !wr_addr[31])
      mem[wr_addr] <= wr_data;

  // read from uart
  always @(posedge clk)
    if (i_reset)
    begin
      rx_addr <= 0;
      r_running <= 0;
    end
    else if (rx_valid)
    begin
      case (rx_byte)
        2'b00: mem[rx_word] <= {24'b0, rx_data};
        2'b01: mem[rx_word] <= {16'b0, rx_data, mem[rx_word][7:0]};
        2'b10: mem[rx_word] <= {8'b0, rx_data, mem[rx_word][15:0]};
        2'b11: mem[rx_word] <= {rx_data, mem[rx_word][23:0]};
      endcase
      rx_addr <= rx_addr + 1;
    end

  assign cpu_running = r_running;
  always @(posedge clk)
    if (pc_data == 32'hffffffff)
      r_running <= 1'b0;
    else if (mem[rx_word-1] == 32'hffffffff && pc_addr == 0)
      r_running <= 1'b1;
endmodule
