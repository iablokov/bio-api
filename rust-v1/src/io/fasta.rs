use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

pub struct FastaReader<'a>
{
    alphabet : &'a Alphabet,
}

impl<'a> FastaReader<'a>
{
    pub fn new(alphabet: &'a Alphabet) -> Self
    {
        FastaReader { alphabet : alphabet }
    }

}