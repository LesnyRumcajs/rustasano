use crate::crypto_utils::hamming_distance;
use crate::xor_cipher::crack_single_byte_xor;
use crate::xor_cipher::evaluate_english;

pub fn repeating_key_xor(input: &[u8], key: &[u8]) -> Vec<u8> {
    input
        .iter()
        .zip(key.iter().cycle())
        .map(|x| x.0 ^ x.1)
        .collect()
}

pub fn repeating_key_xor_break(input: &[u8]) -> Vec<u8> {
    let candidates = create_candidate_keysizes(&input);
    let candidate_keys = create_candidate_keys(input, candidates);
    let candidate_plaintexts = create_candidate_plaintexts(&input, candidate_keys);

    get_best_scored_plaintext(candidate_plaintexts)
}

fn get_best_scored_plaintext(candidate_plaintexts: Vec<(Vec<u8>, i32)>) -> Vec<u8> {
    candidate_plaintexts
        .iter()
        .max_by_key(|x| x.1)
        .unwrap()
        .0
        .clone()
}

fn create_candidate_plaintexts(input: &&[u8], candidate_keys: Vec<Vec<u8>>) -> Vec<(Vec<u8>, i32)> {
    let mut candidate_plaintexts: Vec<(Vec<u8>, i32)> = Vec::new();
    for key in candidate_keys {
        let plaintext = repeating_key_xor(&input, &key);
        let score = evaluate_english(&plaintext);
        candidate_plaintexts.push((plaintext, score));
    }
    candidate_plaintexts
}

fn create_candidate_keys(input: &[u8], candidates: Vec<usize>) -> Vec<Vec<u8>> {
    let mut candidate_keys: Vec<Vec<u8>> = Vec::new();
    // transpose
    for key_size in candidates {
        let mut transposed = vec![Vec::new(); key_size];

        for (i, byte) in input.iter().enumerate() {
            transposed[i % key_size].push(*byte);
        }

        let mut candidate_key = Vec::new();
        for block in transposed {
            candidate_key.push(crack_single_byte_xor(&block).2);
        }
        candidate_keys.push(candidate_key);
    }
    candidate_keys
}

fn create_candidate_keysizes(input: &&[u8]) -> Vec<usize> {
    let mut key_size: Vec<(usize, f32)> = (2..40)
        .map(|key_size| {
            (
                key_size,
                (0..3)
                    .map(|i| {
                        hamming_distance(
                            &input[i * key_size..(2 * i + 1) * key_size],
                            &input[(2 * i + 1) * key_size..(2 * i + 2) * key_size],
                        )
                    })
                    .sum::<u32>() as f32
                    / key_size as f32,
            )
        })
        .collect();

    //workaround because Ord is not implemented for f32 / f64
    key_size.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let candidates: Vec<_> = key_size.iter().take(3).map(|x| x.0).collect();
    candidates
}
