
// Author Albin Dalbert

const EMPTY: i8 = 0;
const PAWN: i8 = 1;
const ROOK: i8 = 2;
const KNIGHT: i8 = 3;
const QUEEN: i8 = 5;
const KING: i8 = 6;
const BISHOP: i8 = 4;

static BOARD: Vec<i32> = Vec::new();

fn if_enemy(origin: i8, b: i8) -> bool {
    if BOARD.at(origin) > 0 && BOARD.at(b) < 0 {
        return true;
    }
    if BOARD.at(origin) < 0 && BOARD.at(b) > 0 {
        return true;
    }
    else {
        return false;
    }
}

// return -1 outside of bound
// return 0 is edge
// return 1 inside but not edge

fn is_edge(pos: i8) -> i8{
    if pos < 0 || pos > 63 {
        return -1;
    }

    else if pos < 9 || pos > 54 {
        return 0;
    }

    else if (pos % 8) == 0 || (pos % 8) == 7 {
        return 0;
    }
    
    else {
        return 1;
    }
}


fn pawn_movement(pos_list: Vec<i8>, origin: i8) { 
    //checks if origin is a white pawn
    if BOARD.at(origin) > 0 {

        if (origin - 8) > 0 {
            if BOARD.at(origin - 8) == EMPTY {
              pos_list.push(origin - 8);
            } 
        }

        if origin > 47 && origin < 56 {
            if BOARD.at(origin - 16) == EMPTY && BOARD.at(origin - 8) == EMPTY{
              pos_list.push(origin - 16);
            }
        }

        if (origin - 7) > 0 {
            if if_enemy(origin, origin - 7) {
              pos_list.push(origin - 7);
          
        }

        if (origin - 9) > 0 {
            if if_enemy(origin, origin - 9) {
              pos_list.push(origin - 9);
            }
        }    
      }
    }
    //checks if origin is a black pawn
    if BOARD.at(origin) < 0 {

        if (origin + 8) < 65 {
            if BOARD.at(origin + 8) == EMPTY {
              pos_list.push(origin + 8);
            }
        }

        if origin > 7 && origin < 16 {
            if BOARD.at(origin + 16) == EMPTY && BOARD.at(origin + 8) == EMPTY {
              pos_list.push(origin + 16);
            } 
        }

        if (origin + 7) < 65 {
            if if_enemy(origin, origin + 7) {
              pos_list.push(origin + 7);
            }   
        }

        if (origin + 9) < 65 {
            if if_enemy(origin, origin + 9) {
              pos_list.push(origin + 9);
            } 
        }
    }
}

fn rook_movement(pos_list: Vec<i8>, origin: i8) {

    for i in (origin + 8..63).step_by(8)
    {
        if i < 0 || i > 63 {
            break;
        } 
        if BOARD.at(i) == EMPTY { 
            pos_list.push(i); 
        } 
        else if if_enemy(origin, i) { 
            pos_list.push(i); 
            break; 
        } 
        else { 
            break; 
        }
    }

    
    for i in (origin + 56..0).step_by(8)
    {
        if i < 0 || i > 63 {
            break;
        } 
        if BOARD.at(i) == EMPTY { 
            pos_list.push(i); 
        } 
        else if if_enemy(origin, i) { 
            pos_list.push(i); 
            break; 
        } 
        else { 
            break; 
        }
    }


    for i in origin + 1..63 
    {
        if i % 8 <= 0 {
            break;
        }
        if i < 0 || i > 63 {
            break;
        } 
        if BOARD.at(i) == EMPTY { 
            pos_list.push(i); 
        } 
        else if if_enemy(origin, i) { 
            pos_list.push(i); 
            break; 
        } 
        else { 
            break; 
        }        
    }


    for i in origin + 63..1 
    {
        if i % 8 >= 7 {
            break;
        }
        if i < 0 || i > 63 {
            break;
        } 
        if BOARD.at(i) == EMPTY { 
            pos_list.push(i); 
        } 
        else if if_enemy(origin, i) { 
            pos_list.push(i); 
            break; 
        } 
        else { 
            break; 
        }        
    }

}


fn knight_movement(pos_list: Vec<i8>, origin: i8) {
    let knight_offsets: [i8; 8] = [-17, -15, -10,-6, 6, 10, 15, 17];
    let pos: i8;

    for i in knight_offsets
    {
        pos = origin + i;
        if pos >= 0 && pos < 64 && (BOARD.at(pos) == EMPTY || if_enemy(origin, pos)) {

            if i8::abs(i8::abs(origin % 8) - i8::abs((origin + i) % 8)) < 3 {
                pos_list.push(pos);
            }
            
        }
    }
}

fn bishop_movement(pos_list: Vec<i8>, origin: i8) {

    for i in (origin + 9..63).step_by(9) {

        if i % 8 <= 0 {
            break;
        }
        
        if BOARD.at(i) == EMPTY {
            pos_list.push(i);
        }
        else if if_enemy(origin, i) {
            pos_list.push(i);
            break;
        }
        else {
            break;
        }

    }

    for i in (origin + 7..63).step_by(7) {
        if i % 8 >= 7 {
            break;
        }

        if BOARD.at(i) == EMPTY {
            pos_list.push(i);
        }
        else if if_enemy(origin, i) {
            pos_list.push(i);
            break;
        }
        else {
            break;
        }
    }

    for i in (origin - 63..9).step_by(9) {
        if i % 8 >= 7 {
            break;
        }
    
        if BOARD.at(i) == EMPTY {
            pos_list.push(i);
        }
        else if if_enemy(origin, i) {
            pos_list.push(i);
            break;
        }
        else {
            break;
        }
    }

    for i in (origin - 63..7).step_by(7) {
        if i % 8 <= 0 {
            break;
        }

        if BOARD.at(i) == EMPTY {
            pos_list.push(i);
        }
        else if if_enemy(origin, i) {
            pos_list.push(i);
            break;
        }
        else {
            break;
        }
    }
}

fn queen_movement(pos_list: Vec<i8>, origin: i8) {
    rook_movement(pos_list, origin);
    bishop_movement(pos_list, origin);
}

fn king_movement(pos_list: Vec<i8>, origin: i8) {

    let king_offsets: [i8; 8] = [ -9, -8, -7,-1, 1, 7, 8, 9 ];
    let pos: i8;

    for i in king_offsets
    {
        pos = origin + i;
        if pos >= 0 && pos < 64 && (BOARD.at(pos) == EMPTY || if_enemy(origin, pos)) {
            pos_list.push(pos);
        }
    }
}

fn valid_moves_from(in_board: Vec<32>, origin: i8) -> Vec<i8>{
    BOARD = in_board;

    let mut pos_list: Vec<i8> = Vec::new();

    for &input in &[EMPTY, PAWN, ROOK, KNIGHT, BISHOP, QUEEN, KING] {
        match input {
            PAWN => pawn_movement(pos_list, origin),
            ROOK => rook_movement(pos_list, origin),
            KNIGHT => knight_movement(pos_list, origin),
            BISHOP => bishop_movement(pos_list, origin),
            QUEEN => queen_movement(pos_list, origin),
            KING => king_movement(pos_list, origin),
            _ => break,
        }
    }

    pos_list
}