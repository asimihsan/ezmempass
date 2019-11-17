pub struct DeltaEncoder<'a> {
    data: &'a Vec<u32>,
}

impl<'a> DeltaEncoder<'a> {
    /// Creates a new encoder
    pub fn new(data: &'a Vec<u32>) -> DeltaEncoder<'a> {
        DeltaEncoder { data }
    }

    /// Encode
    pub fn encode(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let mut last_number = 0;
        let mut gapless_run: u32 = 0;
        for &number in self.data {
            if number == 0 {
                panic!("Input cannot include 0.");
            }
            if last_number >= number {
                panic!("Input must be monotonic strictly increasing sequence")
            }
            let mut delta: u32 = number - last_number - 1;
            /*println!(
                "number: {}, last_number: {}, delta: {}",
                number, last_number, delta
            );*/
            last_number = number;
            if delta == 0 {
                gapless_run += 1;
                continue;
            }
            while gapless_run > 0 {
                let gapless_upper: u8 = 0x80;
                let gapless_lower: u8 = (gapless_run & 0x7F) as u8;
                let byte: u8 = gapless_upper | gapless_lower;
                /*println!("encoding gapless run first: {:X}", byte);*/
                result.push(byte);
                gapless_run >>= 7;
            }
            while delta > 0 {
                let delta_upper: u8 = if delta < 0x20 { 0x20 } else { 0x40 };
                let delta_lower: u8 = (delta & 0x1F) as u8;
                /*println!(
                    "delta upper: {:X}, delta lower: {:X}",
                    delta_upper, delta_lower
                );*/
                let byte: u8 = delta_upper | delta_lower;
                /*println!("encoding delta: {:X}", byte);*/
                result.push(byte);
                delta >>= 5;
            }
        }
        /*println!("encoding ended with gapless_run {}", gapless_run);*/
        while gapless_run > 0 {
            let gapless_upper: u8 = 0x80;
            let gapless_lower: u8 = (gapless_run & 0x7F) as u8;
            let byte: u8 = gapless_upper | gapless_lower;
            /*println!("encoding gapless run second: {:X}", byte);*/
            result.push(byte);
            gapless_run >>= 7;
        }
        result
    }
}

pub struct DeltaDecoder<'a> {
    data: &'a Vec<u8>,
}

impl<'a> DeltaDecoder<'a> {
    /// Creates a new encoder
    pub fn new(data: &'a Vec<u8>) -> DeltaDecoder<'a> {
        DeltaDecoder { data }
    }

    /// Encode
    pub fn decode(&self) -> Vec<u32> {
        let mut result = Vec::new();
        let encoded_chars = self.data;
        let mut encoded_index: usize = 0;
        let mut last_number: u32 = 0;
        let mut gapless_run: u32 = 0;
        while encoded_index < encoded_chars.len() || gapless_run > 0 {
            let mut delta_index: usize = 0;
            let mut delta: u32 = 0;
            if gapless_run > 0 {
                /*println!("gapless_run is decrementing from {}", gapless_run);*/
                gapless_run -= 1;
            } else {
                let value: u32 = u32::from(encoded_chars[encoded_index]);
                /*println!("decode value: {:X?}", value);*/
                if value & 0x80 != 0 {
                    /*println!(
                        "gapless marker. encoded_index {}, delta_index {}",
                        encoded_index, delta_index
                    );*/
                    loop {
                        /*println!("decoding gapless. encoded_index {}, delta_index {}, gapless_run: {}", encoded_index, delta_index, gapless_run);*/
                        if encoded_index + delta_index >= encoded_chars.len() {
                            break;
                        }
                        let value: u32 = u32::from(encoded_chars[encoded_index + delta_index]);
                        /*println!("gapless value: {:X}", value);*/
                        if value & 0x80 == 0 {
                            delta_index -= 1;
                            break;
                        }
                        let gapless_chunk: u32 = (value & 0x7F) << ((delta_index * 7) as u32);
                        gapless_run |= gapless_chunk;
                        delta_index += 1;
                    }
                    gapless_run -= 1;
                    delta_index += 1;
                } else {
                    /*println!("not gapless marker");*/
                    loop {
                        let value: u32 = u32::from(encoded_chars[encoded_index + delta_index]);
                        //                            println!("value for regular: {}", value);
                        let delta_chunk: u32 = (value & 0x1F) << ((delta_index * 5) as u32);
                        delta |= delta_chunk;
                        delta_index += 1;
                        if value & 0x40 == 0 {
                            break;
                        }
                    }
                }
            }
            /*println!(
                "encoded_index: {}, delta_index: {}, delta: {}",
                encoded_index, delta_index, delta
            );*/
            encoded_index += delta_index;
            let number: u32 = last_number + delta + 1;
            /*println!("decoded number as: {}", number);*/
            result.push(number);
            last_number = number;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::convert::TryInto;

    use rand::Rng;

    use super::*;

    #[test]
    fn empty_list() {
        execute_encode_decode_test(&Vec::new());
    }

    #[test]
    fn single_one() {
        execute_encode_decode_test(&vec![1]);
    }

    #[test]
    #[should_panic]
    fn zero_input_is_rejected() {
        execute_encode_decode_test(&vec![0]);
    }

    #[test]
    #[should_panic]
    fn out_of_order_input_is_rejected() {
        execute_encode_decode_test(&vec![5, 4, 3, 2, 1]);
    }

    #[test]
    #[should_panic]
    fn dupes_are_rejected() {
        execute_encode_decode_test(&vec![1, 2, 3, 4, 5, 5]);
    }

    #[test]
    fn single_run() {
        execute_encode_decode_test(&vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn start_run_end_run() {
        execute_encode_decode_test(&vec![1, 2, 3, 4, 5, 8, 11, 12, 13, 14, 15]);
    }

    #[test]
    fn no_runs() {
        execute_encode_decode_test(&vec![5, 10, 15, 20]);
    }

    #[test]
    fn big_jump() {
        execute_encode_decode_test(&vec![1, 100_000, 1_000_000, 10_000_001]);
    }

    #[test]
    fn large_single_run_01() {
        let mut xs: Vec<u32> = Vec::with_capacity(33);
        for i in 1..33 {
            xs.push(i);
        }
        execute_encode_decode_test(&xs);
    }

    #[test]
    fn large_single_run_02() {
        let mut xs: Vec<u32> = Vec::with_capacity(35);
        for i in 1..35 {
            xs.push(i);
        }
        execute_encode_decode_test(&xs);
    }

    #[test]
    fn two_large_runs() {
        let mut xs: Vec<u32> = Vec::with_capacity(200);
        for i in 1..100 {
            xs.push(i);
        }
        for i in 101..200 {
            xs.push(i);
        }
        execute_encode_decode_test(&xs);
    }

    #[test]
    fn one_million_random_integers() {
        let mut rng = rand::thread_rng();
        let xs = get_random_vector(
            1_000_000,  /*minimum_vector_length*/
            1_000_001,  /*maximum_vector_length*/
            1,          /*minimum_value */
            10_000_000, /*maximum_value*/
            &mut rng,
        );
        execute_encode_decode_test(&xs);
    }

    #[test]
    fn one_million_integer_run() {
        let mut xs: Vec<u32> = Vec::with_capacity(1_000_000);
        for i in 1..1_000_000 {
            xs.push(i);
        }
        execute_encode_decode_test(&xs);
    }

    /// Intentially do not use a fixed seed. That way each time this test runs more of the
    /// space of inputs is explored.
    #[test]
    fn random_tests() {
        const NUMBER_OF_RUNS: u32 = 1000;
        const MINIMUM_VECTOR_LENGTH: u32 = 0;
        const MAXIMUM_VECTOR_LENGTH: u32 = 1000;
        const MINIMUM_VALUE: u32 = 1;
        const MAXIMUM_VALUE: u32 = 10_000_000;
        let mut rng = rand::thread_rng();
        for _ in 0..NUMBER_OF_RUNS {
            let input = get_random_vector(
                MINIMUM_VECTOR_LENGTH,
                MAXIMUM_VECTOR_LENGTH,
                MINIMUM_VALUE,
                MAXIMUM_VALUE,
                &mut rng,
            );
            execute_encode_decode_test(&input);
        }
    }

    fn get_random_vector(
        minimum_vector_length: u32,
        maximum_vector_length: u32,
        minimum_value: u32,
        maximum_value: u32,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Vec<u32> {
        let length: usize = rng
            .gen_range(minimum_vector_length, maximum_vector_length)
            .try_into()
            .unwrap();
        let mut values_to_use: BTreeSet<u32> = BTreeSet::new();
        while values_to_use.len() < length {
            values_to_use.insert(rng.gen_range(minimum_value, maximum_value));
        }
        let mut result: Vec<u32> = Vec::with_capacity(length);
        for i in values_to_use.iter() {
            result.push(*i);
        }
        result
    }

    #[test]
    fn it_works() {
        execute_encode_decode_test(&vec![1, 2, 3, 5, 8, 13]);
    }

    fn execute_encode_decode_test(input: &Vec<u32>) {
        /*println!("input: {:?}", input);*/
        let encoder = DeltaEncoder::new(input);
        let encoded = encoder.encode();
        let decoder = DeltaDecoder::new(&encoded);
        let decoded = decoder.decode();
        assert_eq!(&decoded, input);
    }
}
