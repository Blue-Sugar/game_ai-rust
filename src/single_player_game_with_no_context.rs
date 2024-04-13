use rand::*;

#[allow(unused)]
#[derive(Clone, Copy)]
struct Player {
    // player の状態を表す。
}
#[allow(unused)]
impl Player {
    pub fn new(seed: u64) -> Self {
        let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(seed);
        // 初期化された player を返す関数
        Self {

        }
    }

    fn transition(&mut self, seed: u64) {
        let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(seed);
        // player の情報を一部更新する関数。
    }
}

#[derive(Clone, Copy)]
#[allow(unused)]
struct Board {
    // board の状態を表す。
}
#[allow(unused)]
impl Board {
    pub fn new(seed: u64) -> Self {
        let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(seed);
        // 初期化された盤面を返す関数。
        Self {

        }
    }
}

#[allow(unused)]
#[derive(Clone, Copy)]
enum Action {
    // player がし得る action 全体。
}
// Action のなかで player が取り得る action　全体。
#[allow(unused)]
const ACTIONS: [Action; 0] = [];
#[allow(unused)]
const INF: isize = std::isize::MAX / 2;

#[derive(Clone, Copy)]
#[allow(unused)]
struct GameState {
    board: Board,
    initial_board: Board,
    turn: usize,
    player: Player,
    game_score: isize,
    evaliated_score: isize,
    end_turn: usize,
}
#[allow(unused)]
impl GameState {
    pub fn new(seed: u64) -> Self {
        // GameState をシード値を用いて作成する。
        let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(seed);
        let board = Board::new(0);
        let player = Player::new(0);
        let end_turn = rng.gen::<usize>();
        Self {
            board: board,
            initial_board: board.clone(),
            turn: 0,
            player: player,
            game_score: 0,
            evaliated_score: 0,
            end_turn: end_turn,
        }
    }
    pub fn build(board: Board, player: Player, end_turn: usize) -> Self {
        // GameState を情報が与えられた前提で作成する。
        Self {
            board: board,
            initial_board: board.clone(),
            turn: 0,
            player: player,
            game_score: 0,
            evaliated_score: 0,
            end_turn: end_turn,
        }
    }

    pub fn init(&mut self) {
        // player の状態をランダムに初期化する。
        let mut rng = rand::thread_rng();
        self.player = Player::new(rng.gen::<u64>());
    }

    pub fn transition(&mut self) {
        // player の状態を一つ遷移させ、盤面を初期化する。
        let mut rng = rand::thread_rng();
        self.board = self.initial_board.clone();
        self.turn = 0;
        self.player.transition(rng.gen::<u64>());
        self.game_score = 0;
        self.evaliated_score = 0;
    }

    fn is_done(&self) -> bool {
        // game が終了しているかどうかを返す。
        self.turn == self.end_turn
    }

    fn advance(&mut self) {
        // game が一手進む関数。
    }

    fn get_score(&mut self) -> isize {
        while !self.is_done() {
            self.advance();
        }
        self.game_score
    }
}
#[allow(unused)]
fn random(mut state: GameState) -> GameState {
    state.init();
    state
}
#[allow(unused)]
fn hill_climb(mut state: GameState, number: usize) -> GameState {
    state.init();
    let mut best_score = state.get_score();
    for _ in 0..number {
        let mut now_state = state.clone();
        now_state.transition();
        let now_score = now_state.get_score();
        if now_score > best_score {
            best_score = now_score;
            std::mem::swap(&mut state, &mut now_state);
        }
    }
    state
}
#[allow(unused)]
fn simulated_annealing(mut state: GameState, number: usize, start_temp: f64, end_temp: f64) -> GameState {
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(0);
    state.init();
    let mut best_score = state.get_score();
    for i in 0..number {
        let mut now_state = state.clone();
        now_state.transition();
        let now_score = now_state.get_score();
        let temp = start_temp + (end_temp - start_temp) * (i as f64 / number as f64);
        let probability = ((now_score - best_score) as f64 / temp).exp();
        if now_score > best_score || probability > rng.gen::<u8>() as f64 / std::u8::MAX as f64 {
            best_score = now_score;
            std::mem::swap(&mut state, &mut now_state);
        }
    }
    state
}

#[allow(unused)]
pub fn play_game(seed: u64) -> isize {
    let _state = GameState::new(seed);
    let mut state = random(_state);
    state.get_score()
}
#[allow(unused)]
pub fn test_ai_score(game_number: usize) -> f64 {
    let mut rng = rand::thread_rng();
    let mut score_sum = 0;
    for _ in 0..game_number {
        score_sum += play_game(rng.gen::<u64>());
    }
    score_sum as f64 / game_number as f64
}
