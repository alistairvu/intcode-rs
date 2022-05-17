#[derive(Debug)]
pub enum Param {
    Immediate(i64),
    Position(usize),
    Relative(i64),
}

impl Param {
    pub fn get_params(param: &[i64]) -> Vec<Param> {
        let mut result = vec![];

        let mut instruction = param[0] / 100;

        for item in param.iter().skip(1) {
            let param_mode = instruction % 10;
            instruction /= 10;

            if param_mode == 0 {
                result.push(Self::Position(*item as usize));
            } else if param_mode == 1 {
                result.push(Self::Immediate(*item));
            } else if param_mode == 2 {
                result.push(Self::Relative(*item));
            }
        }

        if result.len() != param.len() {
            panic!("Invalid param mode found: {}", param[0]);
        }

        result
    }
}
