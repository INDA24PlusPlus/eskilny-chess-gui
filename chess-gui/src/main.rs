use ggez::*;
use graphics::Image;
use oscae_chess::Game;
use oscae_chess::Square;
use oscae_chess::PieceColor;
use oscae_chess::PieceType;
use oscae_chess::ChessResult;

const GRID_SIZE: i32 = 8;
const GRID_CELL_SIZE: i32 = 128;

const SCREEN_SIZE: (f32, f32) = (
    (GRID_SIZE * GRID_CELL_SIZE) as f32,
    (GRID_SIZE * GRID_CELL_SIZE) as f32,
);

#[derive(Debug, Clone, Copy)]
struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Rect {
    fn new(x: i32, y: i32) -> Rect {
        return Rect {x, y, w: GRID_CELL_SIZE, h: GRID_CELL_SIZE}
    }
}

fn x(file: i32) -> i32 {
    file * GRID_CELL_SIZE
}

fn y(rank: i32) -> i32 {
    rank * GRID_CELL_SIZE
}

impl From<Rect> for graphics::Rect {
    fn from(pos: Rect) -> Self {
        graphics::Rect::new_i32(
            pos.x,
            pos.y,
            pos.w,
            pos.h,
        )
    }
}

#[allow(non_snake_case)]
struct ChessState {
    chess_backend: Game,
    dt: std::time::Duration,
    b_image: Image,
    B_image: Image,
    k_image: Image,
    K_image: Image,
    n_image: Image,
    N_image: Image,
    p_image: Image,
    P_image: Image,
    q_image: Image,
    Q_image: Image,
    r_image: Image,
    R_image: Image,
    pos_selected: Option<i32>,
}

#[allow(non_snake_case)]
impl ChessState {
    fn new(ctx: &mut Context) -> GameResult<ChessState> {
        let chess_backend = Game::new();
        let dt = std::time::Duration::new(0, 0);

        let b_image = graphics::Image::from_path(ctx, "/Chess_bdt45.png")?;
        let B_image = graphics::Image::from_path(ctx, "/Chess_blt45.png")?;
        let k_image = graphics::Image::from_path(ctx, "/Chess_kdt45.png")?;
        let K_image = graphics::Image::from_path(ctx, "/Chess_klt45.png")?;
        let n_image = graphics::Image::from_path(ctx, "/Chess_ndt45.png")?;
        let N_image = graphics::Image::from_path(ctx, "/Chess_nlt45.png")?;
        let p_image = graphics::Image::from_path(ctx, "/Chess_pdt45.png")?;
        let P_image = graphics::Image::from_path(ctx, "/Chess_plt45.png")?;
        let q_image = graphics::Image::from_path(ctx, "/Chess_qdt45.png")?;
        let Q_image = graphics::Image::from_path(ctx, "/Chess_qlt45.png")?;
        let r_image = graphics::Image::from_path(ctx, "/Chess_rdt45.png")?;
        let R_image = graphics::Image::from_path(ctx, "/Chess_rlt45.png")?;

        Ok(ChessState {
            chess_backend,
            dt,
            b_image,
            B_image,
            k_image,
            K_image,
            n_image,
            N_image,
            p_image,
            P_image,
            q_image,
            Q_image,
            r_image,
            R_image,
            pos_selected: None,
        })
    }
}

impl ggez::event::EventHandler<GameError> for ChessState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx, graphics::Color::from([0.0, 0.0, 0.0, 1.0])
        );
        
        let board_state = self.chess_backend.get_board_state();
        
        /* Iterate over every index, draw:
            the background
                checkerboard pattern
                green if selected piece can move there
                red if selected piece can capture there
            the piece
        */
        for idx in 0..64 {
            let file = idx % 8;
            let rank = idx / 8;
            
            // draw background
            let rect = Rect::new(x(file), y(rank));
            let color: [f32; 4];
            if (file + rank) % 2 == 0 {
                color = [1.0, 0.808, 0.620, 1.0]
            } else {
                color = [0.820, 0.545, 0.278, 1.0]
            }
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(rect.clone().into())
                    .color(color),
            );

            // draw piece
            let square = Square {x: file as i8, y: rank as i8};
            match board_state.get(&square) {
                Some(piece) => {
                    canvas.draw(
                        match piece.color {
                            PieceColor::White => match piece.piece_type {
                                PieceType::King => &self.K_image,
                                PieceType::Queen => &self.Q_image,
                                PieceType::Bishop => &self.B_image,
                                PieceType::Knight => &self.N_image,
                                PieceType::Rook => &self.R_image,
                                PieceType::Pawn => &self.P_image,
                            },
                            PieceColor::Black => match piece.piece_type {
                                PieceType::King => &self.k_image,
                                PieceType::Queen => &self.q_image,
                                PieceType::Bishop => &self.b_image,
                                PieceType::Knight => &self.n_image,
                                PieceType::Rook => &self.r_image,
                                PieceType::Pawn => &self.p_image,
                            }
                        },
                        graphics::DrawParam::new()
                            .dest_rect(rect.clone().into()),
                    );
                },
                None => (), // do not draw when there is no piece
            }
        }
        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() {

    let window_mode = conf::WindowMode {
        width: SCREEN_SIZE.0,
        height: SCREEN_SIZE.1,
        maximized: false,
        fullscreen_type: conf::FullscreenType::Windowed,
        borderless: false,
        min_width: 1.0,
        max_width: 0.0,
        min_height: 1.0,
        max_height: 0.0,
        resizable: false,
        visible: true,
        transparent: false,
        resize_on_scale_factor_change: false,
        logical_size: None,
    };
    let c = conf::Conf::new().window_mode(window_mode);
    let (mut ctx, event_loop) = ContextBuilder::new("Chess", "eskilny")
        .add_resource_path("./resources/png/")
        .default_conf(c)
        .build()
        .unwrap();

    let state = ChessState::new(&mut ctx).unwrap();

    event::run(ctx, event_loop, state);
}