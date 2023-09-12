use crate::constant::S_BOX;

fn sub_bytes<S, T>(state: S) -> Vec<Vec<u8>>
where
	S: AsRef<[T]>,
	T: AsRef<[u8]>,
{
	state.as_ref().iter().map(|row|
		row.as_ref().iter().map(|value| {
			let row = (value >> 4) & 0b1111;
			let column = value & 0b1111;
			S_BOX[row as usize][column as usize]
		}).collect()
	).collect()
}

fn shift_rows<S, T>(state: S) -> Vec<Vec<u8>>
where
	S: AsRef<[T]>,
	T: AsRef<[u8]>,
{
	let state = state.as_ref();

    state.as_ref().iter().enumerate().map(|(i, row)| {
        let row = row.as_ref();
        let mut new_row = vec![0; row.len()];

        for (j, value) in row.iter().enumerate() {
            let new_index = (j + row.len() - i) % row.len();
            new_row[new_index] = *value;
        }

        new_row
    }).collect()
}

fn mix_columns<S, T>(state: S) -> Vec<Vec<u8>>
where
	S: AsRef<[T]>,
	T: AsRef<[u8]>,
{
    todo!()
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn cipher_test() {
        let init_state = [
            [0x19, 0xa0, 0x9a, 0xe9],
            [0x3d, 0xf4, 0xc6, 0xf8],
            [0xe3, 0xe2, 0x8d, 0x48],
            [0xbe, 0x2b, 0x2a, 0x08],
        ];

		let state = sub_bytes(init_state);
        let sub = [
            [0xd4, 0xe0, 0xb8, 0x1e],
            [0x27, 0xbf, 0xb4, 0x41],
            [0x11, 0x98, 0x5d, 0x52],
            [0xae, 0xf1, 0xe5, 0x30],
        ];
		assert_eq!(state, sub);

		let state = shift_rows(state);
        let shift = [
            [0xd4, 0xe0, 0xb8, 0x1e],
            [0xbf, 0xb4, 0x41, 0x27],
            [0x5d, 0x52, 0x11, 0x98],
            [0x30, 0xae, 0xf1, 0xe5],
        ];
		assert_eq!(state, shift);

        let state = mix_columns(state);
        let mix = [
            [0x04, 0xe0, 0x48, 0x28],
            [0x66, 0xcb, 0xf8, 0x06],
            [0x81, 0x19, 0xd3, 0x26],
            [0xe5, 0x9a, 0x7a, 0x4c],
        ];
        assert_eq!(state, mix);

        let round = [
            [0xa4, 0x68, 0x6b, 0x02],
            [0x9c, 0x9f, 0x5b, 0x6a],
            [0x7f, 0x35, 0xea, 0x50],
            [0xf2, 0x2b, 0x43, 0x49],
        ];
    }
}
