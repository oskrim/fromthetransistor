`timescale 1ns / 1ps

module rx_uart #(
    parameter                      BW=9,
    parameter                      TIMER_BITS = 32,
    parameter  [(TIMER_BITS-1):0]  CLOCKS_PER_BAUD = 868,
    localparam [(TIMER_BITS-1):0]  HALF_PER_BAUD = CLOCKS_PER_BAUD / 2
  ) (
    input wire            clk,
    input wire            i_reset,

    output wire           out_start_tx,
    output wire           out_led,
    output wire [BW:0]    out_data,
    output wire [3:0]     out_bit_rx,

    input wire            uart_txd_in
  );

  reg q_uart, qq_uart, ck_uart;

  reg [(BW):0]            r_data_in;
  reg [(BW):0]            r_data_out;
  reg [3:0]               r_bit_rx;
  reg [(TIMER_BITS-1):0]  clk_counter;

  reg                     r_start_rx;
  reg                     r_start_tx;
  reg                     r_prev_in;
  reg                     r_debug;

  assign out_start_tx   = r_start_tx;
  assign out_bit_rx     = r_bit_rx;
  assign out_data       = r_data_out;
  assign out_led        = r_debug;

  always @(posedge clk)
    r_prev_in <= ck_uart;

  always @(posedge clk)
    if (i_reset)
      r_bit_rx <= 15;
    else if (r_start_rx)
      r_bit_rx <= 0;
    else if (r_bit_rx < BW && clk_counter == 0)
      r_bit_rx <= r_bit_rx + 1;
    else if (r_bit_rx == BW && clk_counter == 0)
      r_bit_rx <= 15;

  always @(posedge clk)
    if (i_reset || r_start_rx)
      r_data_in <= 10'b1111111111;
    else if (clk_counter == HALF_PER_BAUD)
      r_data_in[r_bit_rx] <= ck_uart;

  always @(posedge clk)
    if (r_start_tx)
      r_data_out <= r_data_in;

  always @(posedge clk)
    if (i_reset || r_start_rx)
      r_start_rx <= 0;
    else if (r_bit_rx == 15 && !ck_uart && r_prev_in)
      r_start_rx <= 1;

  always @(posedge clk)
    if (i_reset)
      r_start_tx <= 1;
    else if (r_start_tx)
      r_start_tx <= 0;
    else if (r_bit_rx == BW && clk_counter == 0)
      r_start_tx <= 1;

  always @(posedge clk)
    if (clk_counter == 0 || r_start_rx)
      clk_counter <= CLOCKS_PER_BAUD - 1;
    else
      clk_counter <= clk_counter - 1;

  reg [31:0] debug_counter;
  reg transition;
  initial debug_counter = 0;
  initial transition = 1;
  always @(posedge clk)
    if (i_reset)
    begin
      debug_counter <= 32'hffffffff;
      transition <= 1;
    end
    else if (debug_counter[31] && !ck_uart)
    begin
      debug_counter <= 0;
      transition <= 0;
    end
    else if (!debug_counter[31])
    begin
      if (!transition)
        debug_counter <= debug_counter + 1;
      if (ck_uart && !transition)
        transition <= 1;
    end

  always @(posedge clk)
    r_debug <= transition && (debug_counter > 9474) && (debug_counter < 11457);

  // 2-step sync for metastability
  initial q_uart  = 1'b0;
  initial qq_uart = 1'b0;
  initial ck_uart = 1'b0;
  always @(posedge clk)
  begin
    q_uart  <= uart_txd_in;
    qq_uart <= q_uart;
    ck_uart <= qq_uart;
  end
endmodule
