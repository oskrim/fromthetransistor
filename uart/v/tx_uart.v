`timescale 1ns / 1ps

module tx_uart #(
    parameter                     BW=9,
    parameter                     TIMER_BITS = 10,
    parameter [(TIMER_BITS-1):0]  CLOCKS_PER_BAUD = 868
  ) (
    input wire            clk,
    input wire            i_reset,
    input wire            i_start_tx,
    input wire  [BW:0]    i_data,

    output wire [3:0]     out_bit_tx,
    output wire           uart_rxd_out
  );

  reg [3:0]               r_bit_tx;
  reg                     r_out;
  reg [(TIMER_BITS-1):0]  clk_counter;

  assign out_bit_tx   = r_bit_tx;
  assign uart_rxd_out = r_out;

  always @(posedge clk)
    if (i_reset)
      r_bit_tx <= 15;
    else if (i_start_tx)
      r_bit_tx <= 0;
    else if (r_bit_tx < BW && clk_counter == 0)
      r_bit_tx <= r_bit_tx + 1;
    else if (r_bit_tx == BW && clk_counter == 0)
      r_bit_tx <= 15;

  always @(posedge clk)
    if (i_reset)
      r_out <= 1;
    else if (r_bit_tx != 15)
      r_out <= i_data[r_bit_tx];
    else
      r_out <= 1;

  always @(posedge clk)
  begin
    if (clk_counter == 0 || i_start_tx)
      clk_counter <= CLOCKS_PER_BAUD - 1;
    else
      clk_counter <= clk_counter - 1;
  end
endmodule
