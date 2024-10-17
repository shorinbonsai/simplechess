use raylib::prelude::*;
use std::fmt;

// Piece representation
#[derive(Clone, Copy, PartialEq)]
enum Piece {
    Empty,
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

#[derive(Clone, Copy, PartialEq)]
enum Color {
    White,
    Black,
}

// 10x12 board representation
struct Board {
    squares: [Piece; 120],
    side_to_move: Color,
}

impl Board {
    fn new() -> Self {
        let mut board = Board {
            squares: [Piece::Empty; 120],
            side_to_move: Color::White,
        };
        board.init_position();
        board
    }

    fn piece_at(&self, rank: usize, file: usize) -> Piece {
        let index = rank * 10 + file + 21; // Convert 8x8 coordinates to 10x12 index
        self.squares[index]
    }

    fn init_position(&mut self) {
        // Initialize the starting position
        // TODO: Set up pieces on the board
    }

    fn make_move(&mut self, from: usize, to: usize) {
        // TODO: Implement move making logic
    }

    fn generate_moves(&self) -> Vec<(usize, usize)> {
        // TODO: Implement move generation
        Vec::new()
    }

    fn evaluate(&self) -> i32 {
        // TODO: Implement basic position evaluation
        0
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Implement board display
        Ok(())
    }
}

fn piece_to_char(piece: Piece) -> char {
    match piece {
        Piece::Empty => ' ',
        Piece::Pawn(Color::White) => 'P',
        Piece::Knight(Color::White) => 'N',
        Piece::Bishop(Color::White) => 'B',
        Piece::Rook(Color::White) => 'R',
        Piece::Queen(Color::White) => 'Q',
        Piece::King(Color::White) => 'K',
        Piece::Pawn(Color::Black) => 'p',
        Piece::Knight(Color::Black) => 'n',
        Piece::Bishop(Color::Black) => 'b',
        Piece::Rook(Color::Black) => 'r',
        Piece::Queen(Color::Black) => 'q',
        Piece::King(Color::Black) => 'k',
    }
}

struct ChessDisplay {
    board: Board,
}

impl ChessDisplay {
    fn new(board: Board) -> Self {
        ChessDisplay { board }
    }

    fn run(&self) {
        let (mut rl, thread) = raylib::init().size(480, 480).title("Chess").build();

        while !rl.window_should_close() {
            let mut d = rl.begin_drawing(&thread);

            d.clear_background(raylib::color::Color::WHITE);

            // Draw the chess board
            for rank in 0..8 {
                for file in 0..8 {
                    let x = file as i32 * 60;
                    let y = rank as i32 * 60;
                    let color = if (rank + file) % 2 == 0 {
                        raylib::color::Color::new(240, 217, 181, 255) // Light squares
                    } else {
                        raylib::color::Color::new(181, 136, 99, 255) // Dark squares
                    };

                    d.draw_rectangle(x, y, 60, 60, color);

                    // Draw the piece
                    let piece = self.board.piece_at(rank, file);
                    let piece_char = piece_to_char(piece);
                    if piece_char != ' ' {
                        d.draw_text(
                            &piece_char.to_string(),
                            x + 20,
                            y + 15,
                            40,
                            raylib::color::Color::BLACK,
                        );
                    }
                }
            }
        }
    }
}

struct ChessEngine {
    board: Board,
}

impl ChessEngine {
    fn new() -> Self {
        ChessEngine {
            board: Board::new(),
        }
    }

    fn search(&mut self, depth: u8) -> Option<(usize, usize)> {
        // TODO: Implement simple minimax search
        None
    }
}

fn main() {
    let mut engine = ChessEngine::new();
    println!("{}", engine.board);

    let board = Board::new(); // Assuming this initializes the board with starting position
    let display = ChessDisplay::new(board);
    display.run();
    // Main game loop
    // loop {
    // TODO: Implement game loop (get user move, make move, search and make engine move)
    // }
}
