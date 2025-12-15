/*!
 * \author Vinícius Manoel
 * \copyright MIT License
 *
 * \brief Class that calculates match, mismatch and gap cost
 */

use once_cell::sync::Lazy;
use std::sync::Mutex;

pub struct Cost;

static COST_MATRIX: Lazy<Mutex<[[i32; 256]; 256]>> = Lazy::new(|| Mutex::new([[0; 256]; 256]));
static GAP_COST: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(30));
static GAP_GAP: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(30));

impl Cost {
    pub fn get_gap_cost() -> i32 {
        *GAP_COST.lock().unwrap()
    }

    pub fn get_gap_gap() -> i32 {
        *GAP_GAP.lock().unwrap()
    }

    pub fn set_cost_pam250() {
        let mut matrix = COST_MATRIX.lock().unwrap();
        let mut gap = GAP_COST.lock().unwrap();
        let mut gap_gap = GAP_GAP.lock().unwrap();

        // Initialize all to 0
        for row in matrix.iter_mut() {
            row.fill(0);
        }

        // Set PAM250 costs (complete matrix from C++ code)
        // C costs
        matrix[b'C' as usize][b'C' as usize] = 5;
        matrix[b'C' as usize][b'S' as usize] = 17; matrix[b'S' as usize][b'C' as usize] = 17;
        matrix[b'C' as usize][b'T' as usize] = 19; matrix[b'T' as usize][b'C' as usize] = 19;
        matrix[b'C' as usize][b'P' as usize] = 20; matrix[b'P' as usize][b'C' as usize] = 20;
        matrix[b'C' as usize][b'A' as usize] = 19; matrix[b'A' as usize][b'C' as usize] = 19;
        matrix[b'C' as usize][b'G' as usize] = 20; matrix[b'G' as usize][b'C' as usize] = 20;
        matrix[b'C' as usize][b'N' as usize] = 21; matrix[b'N' as usize][b'C' as usize] = 21;
        matrix[b'C' as usize][b'D' as usize] = 22; matrix[b'D' as usize][b'C' as usize] = 22;
        matrix[b'C' as usize][b'E' as usize] = 22; matrix[b'E' as usize][b'C' as usize] = 22;
        matrix[b'C' as usize][b'Q' as usize] = 22; matrix[b'Q' as usize][b'C' as usize] = 22;
        matrix[b'C' as usize][b'H' as usize] = 20; matrix[b'H' as usize][b'C' as usize] = 20;
        matrix[b'C' as usize][b'R' as usize] = 21; matrix[b'R' as usize][b'C' as usize] = 21;
        matrix[b'C' as usize][b'K' as usize] = 22; matrix[b'K' as usize][b'C' as usize] = 22;
        matrix[b'C' as usize][b'M' as usize] = 22; matrix[b'M' as usize][b'C' as usize] = 22;
        matrix[b'C' as usize][b'I' as usize] = 19; matrix[b'I' as usize][b'C' as usize] = 19;
        matrix[b'C' as usize][b'L' as usize] = 23; matrix[b'L' as usize][b'C' as usize] = 23;
        matrix[b'C' as usize][b'V' as usize] = 19; matrix[b'V' as usize][b'C' as usize] = 19;
        matrix[b'C' as usize][b'F' as usize] = 21; matrix[b'F' as usize][b'C' as usize] = 21;
        matrix[b'C' as usize][b'Y' as usize] = 17; matrix[b'Y' as usize][b'C' as usize] = 17;
        matrix[b'C' as usize][b'W' as usize] = 25; matrix[b'W' as usize][b'C' as usize] = 25;

        // S costs
        matrix[b'S' as usize][b'S' as usize] = 15;
        matrix[b'S' as usize][b'T' as usize] = 16; matrix[b'T' as usize][b'S' as usize] = 16;
        matrix[b'S' as usize][b'P' as usize] = 16; matrix[b'P' as usize][b'S' as usize] = 16;
        matrix[b'S' as usize][b'A' as usize] = 16; matrix[b'A' as usize][b'S' as usize] = 16;
        matrix[b'S' as usize][b'G' as usize] = 16; matrix[b'G' as usize][b'S' as usize] = 16;
        matrix[b'S' as usize][b'N' as usize] = 16; matrix[b'N' as usize][b'S' as usize] = 16;
        matrix[b'S' as usize][b'D' as usize] = 17; matrix[b'D' as usize][b'S' as usize] = 17;
        matrix[b'S' as usize][b'E' as usize] = 17; matrix[b'E' as usize][b'S' as usize] = 17;
        matrix[b'S' as usize][b'Q' as usize] = 18; matrix[b'Q' as usize][b'S' as usize] = 18;
        matrix[b'S' as usize][b'H' as usize] = 18; matrix[b'H' as usize][b'S' as usize] = 18;
        matrix[b'S' as usize][b'R' as usize] = 17; matrix[b'R' as usize][b'S' as usize] = 17;
        matrix[b'S' as usize][b'K' as usize] = 17; matrix[b'K' as usize][b'S' as usize] = 17;
        matrix[b'S' as usize][b'M' as usize] = 19; matrix[b'M' as usize][b'S' as usize] = 19;
        matrix[b'S' as usize][b'I' as usize] = 18; matrix[b'I' as usize][b'S' as usize] = 18;
        matrix[b'S' as usize][b'L' as usize] = 20; matrix[b'L' as usize][b'S' as usize] = 20;
        matrix[b'S' as usize][b'V' as usize] = 18; matrix[b'V' as usize][b'S' as usize] = 18;
        matrix[b'S' as usize][b'F' as usize] = 20; matrix[b'F' as usize][b'S' as usize] = 20;
        matrix[b'S' as usize][b'Y' as usize] = 20; matrix[b'Y' as usize][b'S' as usize] = 20;
        matrix[b'S' as usize][b'W' as usize] = 19; matrix[b'W' as usize][b'S' as usize] = 19;

        // T costs
        matrix[b'T' as usize][b'T' as usize] = 14;
        matrix[b'T' as usize][b'P' as usize] = 17; matrix[b'P' as usize][b'T' as usize] = 17;
        matrix[b'T' as usize][b'A' as usize] = 16; matrix[b'A' as usize][b'T' as usize] = 16;
        matrix[b'T' as usize][b'G' as usize] = 17; matrix[b'G' as usize][b'T' as usize] = 17;
        matrix[b'T' as usize][b'N' as usize] = 17; matrix[b'N' as usize][b'T' as usize] = 17;
        matrix[b'T' as usize][b'D' as usize] = 17; matrix[b'D' as usize][b'T' as usize] = 17;
        matrix[b'T' as usize][b'E' as usize] = 17; matrix[b'E' as usize][b'T' as usize] = 17;
        matrix[b'T' as usize][b'Q' as usize] = 18; matrix[b'Q' as usize][b'T' as usize] = 18;
        matrix[b'T' as usize][b'H' as usize] = 18; matrix[b'H' as usize][b'T' as usize] = 18;
        matrix[b'T' as usize][b'R' as usize] = 18; matrix[b'R' as usize][b'T' as usize] = 18;
        matrix[b'T' as usize][b'K' as usize] = 17; matrix[b'K' as usize][b'T' as usize] = 17;
        matrix[b'T' as usize][b'M' as usize] = 18; matrix[b'M' as usize][b'T' as usize] = 18;
        matrix[b'T' as usize][b'I' as usize] = 17; matrix[b'I' as usize][b'T' as usize] = 17;
        matrix[b'T' as usize][b'L' as usize] = 19; matrix[b'L' as usize][b'T' as usize] = 19;
        matrix[b'T' as usize][b'V' as usize] = 17; matrix[b'V' as usize][b'T' as usize] = 17;
        matrix[b'T' as usize][b'F' as usize] = 20; matrix[b'F' as usize][b'T' as usize] = 20;
        matrix[b'T' as usize][b'Y' as usize] = 20; matrix[b'Y' as usize][b'T' as usize] = 20;
        matrix[b'T' as usize][b'W' as usize] = 22; matrix[b'W' as usize][b'T' as usize] = 22;

        // P costs
        matrix[b'P' as usize][b'P' as usize] = 11;
        matrix[b'P' as usize][b'A' as usize] = 16; matrix[b'A' as usize][b'P' as usize] = 16;
        matrix[b'P' as usize][b'G' as usize] = 18; matrix[b'G' as usize][b'P' as usize] = 18;
        matrix[b'P' as usize][b'N' as usize] = 18; matrix[b'N' as usize][b'P' as usize] = 18;
        matrix[b'P' as usize][b'D' as usize] = 18; matrix[b'D' as usize][b'P' as usize] = 18;
        matrix[b'P' as usize][b'E' as usize] = 18; matrix[b'E' as usize][b'P' as usize] = 18;
        matrix[b'P' as usize][b'Q' as usize] = 17; matrix[b'Q' as usize][b'P' as usize] = 17;
        matrix[b'P' as usize][b'H' as usize] = 17; matrix[b'H' as usize][b'P' as usize] = 17;
        matrix[b'P' as usize][b'R' as usize] = 17; matrix[b'R' as usize][b'P' as usize] = 17;
        matrix[b'P' as usize][b'K' as usize] = 18; matrix[b'K' as usize][b'P' as usize] = 18;
        matrix[b'P' as usize][b'M' as usize] = 19; matrix[b'M' as usize][b'P' as usize] = 19;
        matrix[b'P' as usize][b'I' as usize] = 19; matrix[b'I' as usize][b'P' as usize] = 19;
        matrix[b'P' as usize][b'L' as usize] = 20; matrix[b'L' as usize][b'P' as usize] = 20;
        matrix[b'P' as usize][b'V' as usize] = 18; matrix[b'V' as usize][b'P' as usize] = 18;
        matrix[b'P' as usize][b'F' as usize] = 22; matrix[b'F' as usize][b'P' as usize] = 22;
        matrix[b'P' as usize][b'Y' as usize] = 22; matrix[b'Y' as usize][b'P' as usize] = 22;
        matrix[b'P' as usize][b'W' as usize] = 23; matrix[b'W' as usize][b'P' as usize] = 23;

        // A costs
        matrix[b'A' as usize][b'A' as usize] = 15;
        matrix[b'A' as usize][b'G' as usize] = 16; matrix[b'G' as usize][b'A' as usize] = 16;
        matrix[b'A' as usize][b'N' as usize] = 17; matrix[b'N' as usize][b'A' as usize] = 17;
        matrix[b'A' as usize][b'D' as usize] = 17; matrix[b'D' as usize][b'A' as usize] = 17;
        matrix[b'A' as usize][b'E' as usize] = 17; matrix[b'E' as usize][b'A' as usize] = 17;
        matrix[b'A' as usize][b'Q' as usize] = 17; matrix[b'Q' as usize][b'A' as usize] = 17;
        matrix[b'A' as usize][b'H' as usize] = 18; matrix[b'H' as usize][b'A' as usize] = 18;
        matrix[b'A' as usize][b'R' as usize] = 19; matrix[b'R' as usize][b'A' as usize] = 19;
        matrix[b'A' as usize][b'K' as usize] = 18; matrix[b'K' as usize][b'A' as usize] = 18;
        matrix[b'A' as usize][b'M' as usize] = 18; matrix[b'M' as usize][b'A' as usize] = 18;
        matrix[b'A' as usize][b'I' as usize] = 18; matrix[b'I' as usize][b'A' as usize] = 18;
        matrix[b'A' as usize][b'L' as usize] = 19; matrix[b'L' as usize][b'A' as usize] = 19;
        matrix[b'A' as usize][b'V' as usize] = 17; matrix[b'V' as usize][b'A' as usize] = 17;
        matrix[b'A' as usize][b'F' as usize] = 21; matrix[b'F' as usize][b'A' as usize] = 21;
        matrix[b'A' as usize][b'Y' as usize] = 20; matrix[b'Y' as usize][b'A' as usize] = 20;
        matrix[b'A' as usize][b'W' as usize] = 23; matrix[b'W' as usize][b'A' as usize] = 23;

        // G costs
        matrix[b'G' as usize][b'G' as usize] = 12;
        matrix[b'G' as usize][b'N' as usize] = 17; matrix[b'N' as usize][b'G' as usize] = 17;
        matrix[b'G' as usize][b'D' as usize] = 16; matrix[b'D' as usize][b'G' as usize] = 16;
        matrix[b'G' as usize][b'E' as usize] = 17; matrix[b'E' as usize][b'G' as usize] = 17;
        matrix[b'G' as usize][b'Q' as usize] = 18; matrix[b'Q' as usize][b'G' as usize] = 18;
        matrix[b'G' as usize][b'H' as usize] = 19; matrix[b'H' as usize][b'G' as usize] = 19;
        matrix[b'G' as usize][b'R' as usize] = 20; matrix[b'R' as usize][b'G' as usize] = 20;
        matrix[b'G' as usize][b'K' as usize] = 19; matrix[b'K' as usize][b'G' as usize] = 19;
        matrix[b'G' as usize][b'M' as usize] = 20; matrix[b'M' as usize][b'G' as usize] = 20;
        matrix[b'G' as usize][b'I' as usize] = 20; matrix[b'I' as usize][b'G' as usize] = 20;
        matrix[b'G' as usize][b'L' as usize] = 21; matrix[b'L' as usize][b'G' as usize] = 21;
        matrix[b'G' as usize][b'V' as usize] = 18; matrix[b'V' as usize][b'G' as usize] = 18;
        matrix[b'G' as usize][b'F' as usize] = 22; matrix[b'F' as usize][b'G' as usize] = 22;
        matrix[b'G' as usize][b'Y' as usize] = 22; matrix[b'Y' as usize][b'G' as usize] = 22;
        matrix[b'G' as usize][b'W' as usize] = 24; matrix[b'W' as usize][b'G' as usize] = 24;

        // N costs
        matrix[b'N' as usize][b'N' as usize] = 15;
        matrix[b'N' as usize][b'D' as usize] = 15; matrix[b'D' as usize][b'N' as usize] = 15;
        matrix[b'N' as usize][b'E' as usize] = 16; matrix[b'E' as usize][b'N' as usize] = 16;
        matrix[b'N' as usize][b'Q' as usize] = 16; matrix[b'Q' as usize][b'N' as usize] = 16;
        matrix[b'N' as usize][b'H' as usize] = 15; matrix[b'H' as usize][b'N' as usize] = 15;
        matrix[b'N' as usize][b'R' as usize] = 17; matrix[b'R' as usize][b'N' as usize] = 17;
        matrix[b'N' as usize][b'K' as usize] = 16; matrix[b'K' as usize][b'N' as usize] = 16;
        matrix[b'N' as usize][b'M' as usize] = 19; matrix[b'M' as usize][b'N' as usize] = 19;
        matrix[b'N' as usize][b'I' as usize] = 19; matrix[b'I' as usize][b'N' as usize] = 19;
        matrix[b'N' as usize][b'L' as usize] = 20; matrix[b'L' as usize][b'N' as usize] = 20;
        matrix[b'N' as usize][b'V' as usize] = 19; matrix[b'V' as usize][b'N' as usize] = 19;
        matrix[b'N' as usize][b'F' as usize] = 21; matrix[b'F' as usize][b'N' as usize] = 21;
        matrix[b'N' as usize][b'Y' as usize] = 19; matrix[b'Y' as usize][b'N' as usize] = 19;
        matrix[b'N' as usize][b'W' as usize] = 21; matrix[b'W' as usize][b'N' as usize] = 21;

        // D costs
        matrix[b'D' as usize][b'D' as usize] = 13;
        matrix[b'D' as usize][b'E' as usize] = 14; matrix[b'E' as usize][b'D' as usize] = 14;
        matrix[b'D' as usize][b'Q' as usize] = 15; matrix[b'Q' as usize][b'D' as usize] = 15;
        matrix[b'D' as usize][b'H' as usize] = 16; matrix[b'H' as usize][b'D' as usize] = 16;
        matrix[b'D' as usize][b'R' as usize] = 18; matrix[b'R' as usize][b'D' as usize] = 18;
        matrix[b'D' as usize][b'K' as usize] = 17; matrix[b'K' as usize][b'D' as usize] = 17;
        matrix[b'D' as usize][b'M' as usize] = 20; matrix[b'M' as usize][b'D' as usize] = 20;
        matrix[b'D' as usize][b'I' as usize] = 19; matrix[b'I' as usize][b'D' as usize] = 19;
        matrix[b'D' as usize][b'L' as usize] = 21; matrix[b'L' as usize][b'D' as usize] = 21;
        matrix[b'D' as usize][b'V' as usize] = 19; matrix[b'V' as usize][b'D' as usize] = 19;
        matrix[b'D' as usize][b'F' as usize] = 23; matrix[b'F' as usize][b'D' as usize] = 23;
        matrix[b'D' as usize][b'Y' as usize] = 21; matrix[b'Y' as usize][b'D' as usize] = 21;
        matrix[b'D' as usize][b'W' as usize] = 24; matrix[b'W' as usize][b'D' as usize] = 24;

        // E costs
        matrix[b'E' as usize][b'E' as usize] = 13;
        matrix[b'E' as usize][b'Q' as usize] = 15; matrix[b'Q' as usize][b'E' as usize] = 15;
        matrix[b'E' as usize][b'H' as usize] = 16; matrix[b'H' as usize][b'E' as usize] = 16;
        matrix[b'E' as usize][b'R' as usize] = 18; matrix[b'R' as usize][b'E' as usize] = 18;
        matrix[b'E' as usize][b'K' as usize] = 17; matrix[b'K' as usize][b'E' as usize] = 17;
        matrix[b'E' as usize][b'M' as usize] = 19; matrix[b'M' as usize][b'E' as usize] = 19;
        matrix[b'E' as usize][b'I' as usize] = 19; matrix[b'I' as usize][b'E' as usize] = 19;
        matrix[b'E' as usize][b'L' as usize] = 20; matrix[b'L' as usize][b'E' as usize] = 20;
        matrix[b'E' as usize][b'V' as usize] = 19; matrix[b'V' as usize][b'E' as usize] = 19;
        matrix[b'E' as usize][b'F' as usize] = 22; matrix[b'F' as usize][b'E' as usize] = 22;
        matrix[b'E' as usize][b'Y' as usize] = 21; matrix[b'Y' as usize][b'E' as usize] = 21;
        matrix[b'E' as usize][b'W' as usize] = 24; matrix[b'W' as usize][b'E' as usize] = 24;

        // Q costs
        matrix[b'Q' as usize][b'Q' as usize] = 13;
        matrix[b'Q' as usize][b'H' as usize] = 14; matrix[b'H' as usize][b'Q' as usize] = 14;
        matrix[b'Q' as usize][b'R' as usize] = 16; matrix[b'R' as usize][b'Q' as usize] = 16;
        matrix[b'Q' as usize][b'K' as usize] = 16; matrix[b'K' as usize][b'Q' as usize] = 16;
        matrix[b'Q' as usize][b'M' as usize] = 18; matrix[b'M' as usize][b'Q' as usize] = 18;
        matrix[b'Q' as usize][b'I' as usize] = 19; matrix[b'I' as usize][b'Q' as usize] = 19;
        matrix[b'Q' as usize][b'L' as usize] = 19; matrix[b'L' as usize][b'Q' as usize] = 19;
        matrix[b'Q' as usize][b'V' as usize] = 19; matrix[b'V' as usize][b'Q' as usize] = 19;
        matrix[b'Q' as usize][b'F' as usize] = 22; matrix[b'F' as usize][b'Q' as usize] = 22;
        matrix[b'Q' as usize][b'Y' as usize] = 21; matrix[b'Y' as usize][b'Q' as usize] = 21;
        matrix[b'Q' as usize][b'W' as usize] = 22; matrix[b'W' as usize][b'Q' as usize] = 22;

        // H costs
        matrix[b'H' as usize][b'H' as usize] = 11;
        matrix[b'H' as usize][b'R' as usize] = 15; matrix[b'R' as usize][b'H' as usize] = 15;
        matrix[b'H' as usize][b'K' as usize] = 17; matrix[b'K' as usize][b'H' as usize] = 17;
        matrix[b'H' as usize][b'M' as usize] = 19; matrix[b'M' as usize][b'H' as usize] = 19;
        matrix[b'H' as usize][b'I' as usize] = 19; matrix[b'I' as usize][b'H' as usize] = 19;
        matrix[b'H' as usize][b'L' as usize] = 19; matrix[b'L' as usize][b'H' as usize] = 19;
        matrix[b'H' as usize][b'V' as usize] = 19; matrix[b'V' as usize][b'H' as usize] = 19;
        matrix[b'H' as usize][b'F' as usize] = 19; matrix[b'F' as usize][b'H' as usize] = 19;
        matrix[b'H' as usize][b'Y' as usize] = 17; matrix[b'Y' as usize][b'H' as usize] = 17;
        matrix[b'H' as usize][b'W' as usize] = 20; matrix[b'W' as usize][b'H' as usize] = 20;

        // R costs
        matrix[b'R' as usize][b'R' as usize] = 11;
        matrix[b'R' as usize][b'K' as usize] = 14; matrix[b'K' as usize][b'R' as usize] = 14;
        matrix[b'R' as usize][b'M' as usize] = 17; matrix[b'M' as usize][b'R' as usize] = 17;
        matrix[b'R' as usize][b'I' as usize] = 19; matrix[b'I' as usize][b'R' as usize] = 19;
        matrix[b'R' as usize][b'L' as usize] = 20; matrix[b'L' as usize][b'R' as usize] = 20;
        matrix[b'R' as usize][b'V' as usize] = 19; matrix[b'V' as usize][b'R' as usize] = 19;
        matrix[b'R' as usize][b'F' as usize] = 21; matrix[b'F' as usize][b'R' as usize] = 21;
        matrix[b'R' as usize][b'Y' as usize] = 21; matrix[b'Y' as usize][b'R' as usize] = 21;
        matrix[b'R' as usize][b'W' as usize] = 15; matrix[b'W' as usize][b'R' as usize] = 15;

        // K costs
        matrix[b'K' as usize][b'K' as usize] = 12;
        matrix[b'K' as usize][b'M' as usize] = 17; matrix[b'M' as usize][b'K' as usize] = 17;
        matrix[b'K' as usize][b'I' as usize] = 19; matrix[b'I' as usize][b'K' as usize] = 19;
        matrix[b'K' as usize][b'L' as usize] = 20; matrix[b'L' as usize][b'K' as usize] = 20;
        matrix[b'K' as usize][b'V' as usize] = 19; matrix[b'V' as usize][b'K' as usize] = 19;
        matrix[b'K' as usize][b'F' as usize] = 22; matrix[b'F' as usize][b'K' as usize] = 22;
        matrix[b'K' as usize][b'Y' as usize] = 21; matrix[b'Y' as usize][b'K' as usize] = 21;
        matrix[b'K' as usize][b'W' as usize] = 20; matrix[b'W' as usize][b'K' as usize] = 20;

        // M costs
        matrix[b'M' as usize][b'M' as usize] = 11;
        matrix[b'M' as usize][b'I' as usize] = 15; matrix[b'I' as usize][b'M' as usize] = 15;
        matrix[b'M' as usize][b'L' as usize] = 13; matrix[b'L' as usize][b'M' as usize] = 13;
        matrix[b'M' as usize][b'V' as usize] = 15; matrix[b'V' as usize][b'M' as usize] = 15;
        matrix[b'M' as usize][b'F' as usize] = 17; matrix[b'F' as usize][b'M' as usize] = 17;
        matrix[b'M' as usize][b'Y' as usize] = 19; matrix[b'Y' as usize][b'M' as usize] = 19;
        matrix[b'M' as usize][b'W' as usize] = 21; matrix[b'W' as usize][b'M' as usize] = 21;

        // I costs
        matrix[b'I' as usize][b'I' as usize] = 12;
        matrix[b'I' as usize][b'L' as usize] = 15; matrix[b'L' as usize][b'I' as usize] = 15;
        matrix[b'I' as usize][b'V' as usize] = 13; matrix[b'V' as usize][b'I' as usize] = 13;
        matrix[b'I' as usize][b'F' as usize] = 16; matrix[b'F' as usize][b'I' as usize] = 16;
        matrix[b'I' as usize][b'Y' as usize] = 18; matrix[b'Y' as usize][b'I' as usize] = 18;
        matrix[b'I' as usize][b'W' as usize] = 22; matrix[b'W' as usize][b'I' as usize] = 22;

        // L costs
        matrix[b'L' as usize][b'L' as usize] = 11;
        matrix[b'L' as usize][b'V' as usize] = 15; matrix[b'V' as usize][b'L' as usize] = 15;
        matrix[b'L' as usize][b'F' as usize] = 15; matrix[b'F' as usize][b'L' as usize] = 15;
        matrix[b'L' as usize][b'Y' as usize] = 18; matrix[b'Y' as usize][b'L' as usize] = 18;
        matrix[b'L' as usize][b'W' as usize] = 19; matrix[b'W' as usize][b'L' as usize] = 19;

        // V costs
        matrix[b'V' as usize][b'V' as usize] = 13;
        matrix[b'V' as usize][b'F' as usize] = 18; matrix[b'F' as usize][b'V' as usize] = 18;
        matrix[b'V' as usize][b'Y' as usize] = 19; matrix[b'Y' as usize][b'V' as usize] = 19;
        matrix[b'V' as usize][b'W' as usize] = 23; matrix[b'W' as usize][b'V' as usize] = 23;

        // F costs
        matrix[b'F' as usize][b'F' as usize] = 8;
        matrix[b'F' as usize][b'Y' as usize] = 10; matrix[b'Y' as usize][b'F' as usize] = 10;
        matrix[b'F' as usize][b'W' as usize] = 17; matrix[b'W' as usize][b'F' as usize] = 17;

        // Y costs
        matrix[b'Y' as usize][b'Y' as usize] = 7;
        matrix[b'Y' as usize][b'W' as usize] = 17; matrix[b'W' as usize][b'Y' as usize] = 17;

        // W costs
        matrix[b'W' as usize][b'W' as usize] = 0;

        *gap = 30;
        *gap_gap = 30;
    }

    pub fn set_cost_nuc() {
        let mut matrix = COST_MATRIX.lock().unwrap();
        let mut gap = GAP_COST.lock().unwrap();
        let mut gap_gap = GAP_GAP.lock().unwrap();

        // Initialize all to 0
        for row in matrix.iter_mut() {
            row.fill(0);
        }

        // Nucleotide costs
        matrix[b'A' as usize][b'A' as usize] = 0;
        matrix[b'A' as usize][b'C' as usize] = 1; matrix[b'C' as usize][b'A' as usize] = 1;
        matrix[b'A' as usize][b'G' as usize] = 1; matrix[b'G' as usize][b'A' as usize] = 1;
        matrix[b'A' as usize][b'T' as usize] = 1; matrix[b'T' as usize][b'A' as usize] = 1;
        matrix[b'A' as usize][b'U' as usize] = 1; matrix[b'U' as usize][b'A' as usize] = 1;

        matrix[b'C' as usize][b'C' as usize] = 0;
        matrix[b'C' as usize][b'G' as usize] = 1; matrix[b'G' as usize][b'C' as usize] = 1;
        matrix[b'C' as usize][b'T' as usize] = 1; matrix[b'T' as usize][b'C' as usize] = 1;
        matrix[b'C' as usize][b'U' as usize] = 1; matrix[b'U' as usize][b'C' as usize] = 1;

        matrix[b'G' as usize][b'G' as usize] = 0;
        matrix[b'G' as usize][b'T' as usize] = 1; matrix[b'T' as usize][b'G' as usize] = 1;
        matrix[b'G' as usize][b'U' as usize] = 1; matrix[b'U' as usize][b'G' as usize] = 1;

        matrix[b'T' as usize][b'T' as usize] = 0;
        matrix[b'T' as usize][b'U' as usize] = 0; matrix[b'U' as usize][b'T' as usize] = 0;

        matrix[b'U' as usize][b'U' as usize] = 0;

        *gap = 2;
        *gap_gap = 2;
    }

    pub fn cost(r: u8, l: u8) -> i32 {
        let matrix = COST_MATRIX.lock().unwrap();
        matrix[r as usize][l as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cost_initialization() {
        Cost::set_cost_nuc();
        assert_eq!(Cost::cost(b'A', b'A'), 0);
        assert_eq!(Cost::get_gap_cost(), 5);
    }

    #[test]
    fn test_pam250() {
        Cost::set_cost_pam250();
        assert_eq!(Cost::cost(b'A', b'A'), 0);
        assert_eq!(Cost::get_gap_cost(), 8);
    }
}
