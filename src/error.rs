use std::{ fmt, error };

pub enum TetraError {
    #[allow(non_camel_case_types)]
    ARGUMENTS_SYNTAX,
    #[allow(non_camel_case_types)]
    INPUT_FILE_SYNTAX,
}

impl error::Error for TetraError {}

impl fmt::Display for TetraError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            TetraError::ARGUMENTS_SYNTAX => "Wrong syntax of command. Usage: tetra_solve [input_file].",
            TetraError::INPUT_FILE_SYNTAX => "Wrong syntax of input file.",
        })
    }
}

impl fmt::Debug for TetraError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <TetraError as fmt::Display>::fmt(self, f)
    }
}
