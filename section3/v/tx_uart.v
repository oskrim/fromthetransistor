`timescale 1ns / 1ps

module tx_uart #(
    parameter                     BW = 9,
    parameter                     TIMER_BITS = 32,
    parameter [(TIMER_BITS-1):0]  CLOCKS_PER_BAUD = 868
  ) (
    input wire            clk,
    input wire            i_reset,
    input wire            i_start_tx,
    input wire [(BW-2):0] i_data,

    output wire [3:0]     out_bit_tx,
    output wire           uart_rxd_out
  );

  reg [BW:0]              r_data;
  reg [3:0]               r_bit_tx;
  reg                     r_out;
  reg [(TIMER_BITS-1):0]  clk_counter;

  assign out_bit_tx     = r_bit_tx;
  assign uart_rxd_out   = r_out;

  always @(posedge clk)
    if (i_reset)
    begin
      r_out <= 1;
      r_bit_tx <= 15;
    end
    else if (i_start_tx)
    begin
      r_bit_tx <= 0;
      r_data <= { 1'b1, i_data, 1'b0 };
    end
    else if (clk_counter == 0)
    begin
      if (r_bit_tx != BW && r_bit_tx != 15)
      begin
        r_bit_tx <= r_bit_tx + 1;
        { r_out, r_data } <= { r_data[0], { 1'b1, r_data[BW:1] } };
      end
      else
      begin
        r_out <= 1;
        r_bit_tx <= 15;
      end
    end

  always @(posedge clk)
  begin
    if (clk_counter == 0 || i_start_tx)
      clk_counter <= CLOCKS_PER_BAUD - 1;
    else
      clk_counter <= clk_counter - 1;
  end
endmodule
