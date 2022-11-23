
`timescale 1ns / 1ps

module ram #(
    parameter LOGD = 10
  ) (
    input wire                     clk,
    input wire                     i_reset,

    // from uart, code download
    input wire                     rx_valid,
    input wire [7:0]               rx_data,

    // verilator lint_off UNUSEDSIGNAL
    input wire  [31:0]             rd_addr,
    input wire  [31:0]             wr_addr,
    // verilator lint_on UNUSEDSIGNAL
    output wire [31:0]             rd_data,
    input wire  [31:0]             wr_data,
    input wire                     wr_valid
  );

  reg [31:0] mem [0:((1<<LOGD)-1)];
  reg [31:0] rx_addr;
  reg [1:0]  rx_byte;

  assign rd_data = mem[rd_addr];

  always @(posedge clk)
    if (wr_valid)
      mem[wr_addr] <= wr_data;

  // read from uart
  initial rx_addr = 0;
  initial rx_byte = 0;
  always @(posedge clk)
    if (i_reset)
    begin
      rx_addr <= 0;
      rx_byte <= 0;
    end
    else if (rx_valid)
    begin
      case (rx_byte)
        2'b00: mem[rx_addr] <= {24'b0, rx_data};
        2'b01: mem[rx_addr] <= {16'b0, rx_data, mem[rx_addr][7:0]};
        2'b10: mem[rx_addr] <= {8'b0, rx_data, mem[rx_addr][15:0]};
        2'b11: begin
          mem[rx_addr] <= {rx_data, mem[rx_addr][23:0]};
          rx_addr <= rx_addr + 1;
        end
      endcase
      rx_byte <= rx_byte + 1;
    end
endmodule
