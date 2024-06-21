// S-Box fur sub_bytes
const SBOX: [[u8; 16]; 16] = [
    [0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76],
    [0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0],
    [0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15],
    [0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75],
    [0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84],
    [0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf],
    [0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8],
    [0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2],
    [0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73],
    [0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb],
    [0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79],
    [0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08],
    [0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a],
    [0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e],
    [0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf],
    [0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16],
];

// inverse S-Box for inv_sub_bytes
const INV_SBOX: [[u8; 16]; 16] = [
    [0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb],
    [0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb],
    [0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e],
    [0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25],
    [0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92],
    [0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84],
    [0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06],
    [0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b],
    [0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73],
    [0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e],
    [0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b],
    [0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4],
    [0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f],
    [0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef],
    [0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61],
    [0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d],
];

//type definition - 4x4 boxes/tables
type AESTable = [[u8; 4]; 4];
type RoundKey = [[u8; 4]; 4];


pub enum AESKey {
    Key128([u8; 16]),
    Key192([u8; 24]),
    Key256([u8; 32]),
}

impl AESKey {
    fn len(&self) -> usize {
        match self {
            AESKey::Key128(_) => 16,
            AESKey::Key192(_) => 24,
            AESKey::Key256(_) => 32,
        }
    }

    fn round_count(&self) -> usize {
        match self {
            AESKey::Key128(_) => 10,
            AESKey::Key192(_) => 12,
            AESKey::Key256(_) => 14,
        }
    }

    fn to_vec(&self) -> Vec<u8> {
        match self {
            AESKey::Key128(key) => key.to_vec(),
            AESKey::Key192(key) => key.to_vec(),
            AESKey::Key256(key) => key.to_vec(),
        }
    }

    fn key_expansion(&self) -> Vec<RoundKey> {
        let rcon = vec![
            0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36, 0x6c, 0xd8, 0xab, 0x4d, 0x9a, 0x2f,
        ];

        let rounds = self.round_count() + 1; // Vorrunde
        let mut round_keys: Vec<RoundKey> = Vec::with_capacity(rounds);

        let mut w: Vec<[u8; 4]> = Vec::with_capacity(4 * (rounds + 1));

        let key_cols: Vec<[u8; 4]> = self.to_vec().chunks(4).map(|chunk| {
            let mut col = [0; 4];
            col.copy_from_slice(chunk);
            col
        }).collect();

        for i in 0..4 {
            w.push(key_cols[i]);
        }
        
        for i in 4..(4 * (rounds + 1)) {
            let mut temp = w[i - 1];
            
            if i % 4 == 0 {
                temp.rotate_left(1);
                for j in 0..4 {
                    let row = (temp[j] >> 4) as usize;
                    let col = (temp[j] << 4 >> 4) as usize;
                    temp[j] = SBOX[row][col];
                }
                

                let rcon_val: u8 = rcon[(i / 4) - 1];
                temp = temp.xor([rcon_val, 0, 0, 0]);
            }
            
            let xored = w[i - 4].xor(temp);
            w.push(xored);
        }

        for i in 0..rounds {
            let mut round_key: RoundKey = [[0; 4]; 4];
            for j in 0..4 {
                round_key[0][j] = w[i * 4 + j][0];
                round_key[1][j] = w[i * 4 + j][1];
                round_key[2][j] = w[i * 4 + j][2];
                round_key[3][j] = w[i * 4 + j][3];
            }
            round_keys.push(round_key);
        }

        round_keys
    }
}

pub struct AESCipher {
    table: AESTable,
    round_keys: Vec<RoundKey>,
}

impl AESCipher {
    pub fn new(key: AESKey) -> Self {
        let table = [[0; 4]; 4];
        AESCipher { table, round_keys: key.key_expansion() }
    }



    fn sub_bytes(&mut self) {
        for i in 0..4 {
            for j in 0..4 {
                let row = (self.table[i][j] >> 4) as usize;
                let col = (self.table[i][j] << 4 >> 4) as usize;
                self.table[i][j] = SBOX[row][col];
            }
        }
    }

    fn inv_sub_bytes(&mut self) {
        for i in 0..4 {
            for j in 0..4 {
                let row = (self.table[i][j] >> 4) as usize;
                let col = (self.table[i][j] << 4 >> 4) as usize;
                self.table[i][j] = INV_SBOX[row][col];
            }
        }
    }

    fn rotate_rows(&mut self) {
        self.table[1].rotate_left(1);
        self.table[2].rotate_left(2);
        self.table[3].rotate_left(3);
    }

    fn inv_rotate_rows(&mut self) {
        self.table[1].rotate_right(1);
        self.table[2].rotate_right(2);
        self.table[3].rotate_right(3);
    }

    //https://dkblackley.github.io/posts/rust-aes/ (angepasst)
    fn mix_columns(&mut self) {
        let mds = [
            [0x02, 0x03, 0x01, 0x01],
            [0x01, 0x02, 0x03, 0x01],
            [0x01, 0x01, 0x02, 0x03],
            [0x03, 0x01, 0x01, 0x02],
        ];

        for row_idx in 0..4 {
            let col = [
                self.table[0][row_idx],
                self.table[1][row_idx],
                self.table[2][row_idx],
                self.table[3][row_idx],
            ];
                
            let mut new_word: [u8; 4] = [0, 0, 0, 0];
            for i in 0..4 {
                let mds_row = mds[i];
    
                let mut result: u8 = multiply_gf(mds_row[0], col[0].clone());
                for j in 1..4 {
                    let multiple = multiply_gf(mds_row[j], col[j].clone());
                    result = multiple ^ result;
                    
                }
                new_word[i] = result;
            }

            self.table[0][row_idx] = new_word[0];
            self.table[1][row_idx] = new_word[1];
            self.table[2][row_idx] = new_word[2];
            self.table[3][row_idx] = new_word[3];

        }
    }

    //https://dkblackley.github.io/posts/rust-aes/ (angepasst)
    fn inv_mix_columns(&mut self) {
        let mds = [
            [14, 11, 13, 9],
            [9, 14, 11, 13],
            [13, 9, 14, 11],
            [11, 13, 9, 14],
        ];

        for row_idx in 0..4 {
            let col = [
                self.table[0][row_idx],
                self.table[1][row_idx],
                self.table[2][row_idx],
                self.table[3][row_idx],
            ];
                
            let mut new_word: [u8; 4] = [0, 0, 0, 0];
            for i in 0..4 {
                let mds_row = mds[i];
    
                let mut result: u8 = multiply_gf(mds_row[0], col[0].clone());
                for j in 1..4 {
                    let multiple = multiply_gf(mds_row[j], col[j].clone());
                    result = multiple ^ result;
                    
                }
                new_word[i] = result;
            }

            self.table[0][row_idx] = new_word[0];
            self.table[1][row_idx] = new_word[1];
            self.table[2][row_idx] = new_word[2];
            self.table[3][row_idx] = new_word[3];

        }
    }

    fn add_roundkey(&mut self, round: usize) {
        for i in 0..4 {
            for j in 0..4 {
                self.table[i][j] ^= self.round_keys[round][i][j];
            }
        }
    }

    pub fn encrypt(&mut self, mut data: Vec<u8>) -> Vec<u8> {
        let rounds = self.round_keys.len();
        let mut encrypted_data = Vec::new();
    
        for chunk in data.chunks_mut(16) {
            let mut chunk = chunk.to_vec();
    
            if chunk.len() < 16 {
                chunk.resize(16, 0);
            }
    
            self.table = [
                [chunk[0], chunk[4], chunk[8], chunk[12]],
                [chunk[1], chunk[5], chunk[9], chunk[13]],
                [chunk[2], chunk[6], chunk[10], chunk[14]],
                [chunk[3], chunk[7], chunk[11], chunk[15]],
            ];
            
            self.add_roundkey(0);
            
            for round in 1..rounds {

                self.sub_bytes();

                self.rotate_rows();

                if round != rounds - 1 {
                    self.mix_columns();
                }

                self.add_roundkey(round);

            }

            for i in 0..4 {
                for j in 0..4 {
                    encrypted_data.push(self.table[j][i]);
                }
            }
        }
        
        return encrypted_data
    }

    pub fn decrypt(&mut self, mut data: Vec<u8>) -> Vec<u8> {
        let rounds = self.round_keys.len();

        let mut decrypted_data = Vec::new();

        println!("data len {}", data.len());

        for chunk in data.chunks_mut(16) {
            let mut chunk = chunk.to_vec();
            if chunk.len() < 16 {
                chunk.resize(16, 0);
            }

            self.table = [
                [chunk[0], chunk[4], chunk[8], chunk[12]],
                [chunk[1], chunk[5], chunk[9], chunk[13]],
                [chunk[2], chunk[6], chunk[10], chunk[14]],
                [chunk[3], chunk[7], chunk[11], chunk[15]],
            ];
            
            for round in (1..rounds).rev() {

                self.add_roundkey(round);

                if round != (rounds - 1) {
                    self.inv_mix_columns();
                }

                self.inv_sub_bytes();

                self.inv_rotate_rows();

            }

            self.add_roundkey(0);

            println!("womp {}", self.table.as_hex());

            for i in 0..4 {
                for j in 0..4 {
                    decrypted_data.push(self.table[j][i]);
                }
            }
        }
        
        return decrypted_data
    }
}

// https://dkblackley.github.io/posts/rust-aes/
fn multiply_gf(mut a: u8, mut b: u8) -> u8 {
    let mut p = 0x00;

    for _ in 0..8 {
        if 0x01 & b != 0 { // if the rightmost bit is set
            p = p ^ a; // p + a
        }
        b = b >> 0x01;
        let carry = 0x80 & a; // x^7
        a = a << 1;
        if carry != 0 {
            a = a ^ 0x1b;
        }
    }
    return p;
}

// additional utility-method for Tables
trait TableExt {
    fn as_hex(&self) -> String;
}

impl TableExt for AESTable {
    fn as_hex(&self) -> String {
        let mut result = String::new();
        for i in 0..4 {
            for j in 0..4 {
                result.push_str(&format!("{:02x} ", self[j][i]));
            }
        }
        return result
    }
}

// additional utility-methods for [u8;4]-Arrays
trait ArrayExt {
    fn rotate_left(&mut self, n: usize);
    fn rotate_right(&mut self, n: usize);
    fn xor(self, other: [u8; 4]) -> [u8; 4];
    fn to_u32(&self) -> u32;
}

impl ArrayExt for [u8; 4] {
    fn rotate_left(&mut self, n: usize) {
        for _ in 0..n {
            let temp = self[0];
            self[0] = self[1];
            self[1] = self[2];
            self[2] = self[3];
            self[3] = temp;
        }
    }

    fn rotate_right(&mut self, n: usize) {
        for _ in 0..n {
            let temp = self[3];
            self[3] = self[2];
            self[2] = self[1];
            self[1] = self[0];
            self[0] = temp;
        }
    }

    fn xor(self, other: [u8; 4]) -> [u8; 4] {
        let mut result = [0; 4];
        for i in 0..4 {
            result[i] = self[i] ^ other[i];
        }
        return result
    }

    fn to_u32(&self) -> u32 {
        let mut result = 0;
        for i in 0..4 {
            result = result << 8 | self[i] as u32;
        }
        return result
    }
}
