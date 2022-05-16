#[derive(Debug)]
pub enum ParamMode {
    Immediate(i32),
    Position(usize),
}

impl ParamMode {
    pub fn get_params(param: &[i32]) -> Vec<ParamMode> {
        let mut result = vec![];

        let mut instruction = param[0] / 100;

        for item in param.iter().skip(1) {
            let param_mode = instruction % 10;
            instruction /= 10;

            if param_mode == 0 {
                result.push(Self::Position(*item as usize));
            } else if param_mode == 1 {
                result.push(Self::Immediate(*item));
            }
        }

        result
    }
}
