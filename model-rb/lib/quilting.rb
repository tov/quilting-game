module Quilting
  class Square # <width, height>
    # x : Natural | x < width
    attr_reader :x
    # y : Natural | y < height
    attr_reader :y
  end

  class Piece
    # width : Number
    attr_reader :width
    # height : Number
    attr_reader :height
    # squares : [Square<width, height>]
    # INVARIANT: unique
    attr_reader :squares
  end

  class QuiltBoard
    # width : Natural
    attr_reader :width
    # height : Natural
    attr_reader :height
    # pieces : [[Boolean; width]; height]
    attr_reader :pieces
  end
end
