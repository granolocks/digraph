require 'chunky_png'

GRID_DIM = 256;

data = File.read("digraph.out").split("\n").map{|row| row.strip.split(" ").map(&:to_i)};

png = ChunkyPNG::Image.new(GRID_DIM, GRID_DIM, ChunkyPNG::Color::BLACK);
data.each_with_index{|row, row_id|
  row.each_with_index{|col, col_id|
    png[row_id, col_id] = ChunkyPNG::Color.rgba(col, col, col, 255);
  }
}
png.save('filename.png', :interlace => false)

