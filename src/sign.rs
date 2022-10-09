/// Get sign.
use bytes::{Bytes, BytesMut};
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

fn bytes_to_bin(bytes: Bytes) -> Bytes {
    let len = bytes.len() * 8;
    let mut output = BytesMut::with_capacity(len);

    for v in bytes {
        output.extend([
            (v & 128) >> 7,
            (v & 64) >> 6,
            (v & 32) >> 5,
            (v & 16) >> 4,
            (v & 8) >> 3,
            (v & 4) >> 2,
            (v & 2) >> 1,
            v & 1,
        ])
    }

    output.freeze()
}

fn bin_to_bytes(bytes: Bytes) -> Bytes {
    let len = bytes.len() / 8;
    let mut output = BytesMut::with_capacity(len);

    output.resize(len, 0);

    for j in 0..len {
        output[j] = bytes[j * 8] << 7
            | bytes[j * 8 + 1] << 6
            | bytes[j * 8 + 2] << 5
            | bytes[j * 8 + 3] << 4
            | bytes[j * 8 + 4] << 3
            | bytes[j * 8 + 5] << 2
            | bytes[j * 8 + 6] << 1
            | bytes[j * 8 + 7]
    }

    output.freeze()
}

fn sub_10ea4(input: &[u8]) -> Bytes {
    let table = [
        [0, 0],
        [1, 4],
        [2, 61],
        [3, 15],
        [4, 56],
        [5, 40],
        [6, 6],
        [7, 59],
        [8, 62],
        [9, 58],
        [10, 17],
        [11, 2],
        [12, 12],
        [13, 8],
        [14, 32],
        [15, 60],
        [16, 13],
        [17, 45],
        [18, 34],
        [19, 14],
        [20, 36],
        [21, 21],
        [22, 22],
        [23, 39],
        [24, 23],
        [25, 25],
        [26, 26],
        [27, 20],
        [28, 1],
        [29, 33],
        [30, 46],
        [31, 55],
        [32, 35],
        [33, 24],
        [34, 57],
        [35, 19],
        [36, 53],
        [37, 37],
        [38, 38],
        [39, 5],
        [40, 30],
        [41, 41],
        [42, 42],
        [43, 18],
        [44, 47],
        [45, 27],
        [46, 9],
        [47, 44],
        [48, 51],
        [49, 7],
        [50, 49],
        [51, 63],
        [52, 28],
        [53, 43],
        [54, 54],
        [55, 52],
        [56, 31],
        [57, 10],
        [58, 29],
        [59, 11],
        [60, 3],
        [61, 16],
        [62, 50],
        [63, 48],
    ];
    let arr = bytes_to_bin(Bytes::copy_from_slice(input));

    let mut output = BytesMut::with_capacity(arr.len());
    output.resize(arr.len(), 0);

    for i in 0..table.len() {
        output[table[i][1]] = arr[table[i][0]];
    }
    bin_to_bytes(output.freeze())
}

fn sub_4b7c(input: Bytes) -> Bytes {
    let table = [
        [0, 6, 0, 1],
        [1, 4, 1, 0],
        [2, 5, 0, 1],
        [3, 0, 0, 1],
        [4, 2, 0, 1],
        [5, 3, 0, 1],
        [6, 1, 1, 0],
        [7, 7, 0, 1],
    ];

    let bytes = bytes_to_bin(input);

    let len = 8;
    let mut output = BytesMut::with_capacity(len);
    output.resize(len, 0);

    for i in 0..len {
        if bytes[i] == 0 {
            output[table[i][1]] = table[i][2] as u8;
        } else {
            output[table[i][1]] = table[i][3] as u8;
        }
    }

    bin_to_bytes(output.freeze())
}

fn sub_10d70(input: Bytes) -> Bytes {
    match input.len() {
        1 => sub_4b7c(input),
        _ => Bytes::from(""),
    }
}

fn sub_12510(input: Bytes) -> Bytes {
    let len = input.len() / 8;

    let mut output = BytesMut::with_capacity(input.len());

    for i in 0..len {
        output.extend(sub_10ea4(&input[i * 8..(i + 1) * 8]));
    }

    output.extend(sub_10d70(Bytes::copy_from_slice(
        &input[input.len() - (input.len() % 8)..input.len()],
    )));

    output.freeze()
}

fn sub_12ecc(input: Bytes) -> Bytes {
    let arr = [
        0x37, 0x92, 0x44, 0x68, 0xA5, 0x3D, 0xCC, 0x7F, 0xBB, 0xF, 0xD9, 0x88, 0xEE, 0x9A, 0xE9,
        0x5A,
    ];
    let key = b"80306f4370b39fd5630ad0529f77adb6";
    let mut output = BytesMut::with_capacity(input.len());
    output.resize(input.len(), 0);

    for i in 0..input.len() {
        let mut r0 = input[i] as u16;
        let mut r2 = arr[i & 0xf];
        let r4 = key[i & 7] as u16;
        r0 ^= r2;
        r0 ^= r4;
        r0 += r2;
        r2 ^= r0;
        let r1 = key[i & 7] as u16;
        r2 ^= r1;
        output[i] = (r2 & 0xff) as u8;
    }
    output.freeze()
}

pub fn get_sign_sv(function_id: &str, body: &str, sv: Option<u32>) -> String {
    let st = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    const CHARSET: &[u8] = b"0123456789qwertyuiopasdfghjklzxcvbnm";
    let mut rng = rand::thread_rng();

    let sv = sv.unwrap_or(120);

    let client = "apple";
    let client_version = "11.2.8";

    let uuid: String = (0..32)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    let input = format!("functionId={function_id}&body={body}&uuid={uuid}&client={client}&clientVersion={client_version}&st={st}&sv={sv}");

    let output = input.clone();
    let bytes = match sv {
        120 => sub_12ecc(Bytes::from(input)),
        _ => sub_12510(Bytes::from(input)),
    };

    let b64encode = base64::encode_config(bytes, base64::STANDARD);

    format!("{}&sign={:x}", output, md5::compute(b64encode))
}

pub fn get_sign(function_id: &str, body: &str) -> String {
    get_sign_sv(function_id, body, Some(120))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign() {
        let input = "{\"type\":\"\",\"version\":18,\"channel\":1,\"babelChannel\":\"121\"}";
        let sign = get_sign("waterGoodForFarm", input);
        println!("{:?}", sign);
    }
}
