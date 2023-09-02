use std::collections::HashSet;
use std::collections::HashMap;

use crate::seq::Sequence;

pub struct Alphabet 
{
    correct: HashSet<char>,
    allowed: HashSet<char>,
}

pub struct Transcriptase<'a>
{
    src_alphabet : &'a Alphabet,
    dst_alphabet : &'a Alphabet,
    table        : HashMap<char, char>,
}

pub struct Polymerase<'a>
{
    alphabet : &'a Alphabet,
    table    : HashMap<char, char>,
}

pub struct Ribosome<'a>
{
    src_alphabet : &'a Alphabet,
    dst_alphabet : &'a Alphabet,
    size         : usize,
    table        : HashMap<&'a str, char>,
    stop_codons  : HashSet<&'a str>,
}

pub struct AlphabetError {}

// ===

impl Alphabet 
{
    pub fn new(correct: Vec<char>, allowed: Vec<char>) -> Self 
    {
        Alphabet 
        {
            correct: correct.into_iter().collect(),
            allowed: allowed.into_iter().collect(),
        }
    }

    pub fn is_allowed_symbol(&self, character: char) -> bool 
    {
        self.allowed.contains(&character) || self.correct.contains(&character)
    }

    pub fn is_correct_symbol(&self, character: char) -> bool 
    {
        self.correct.contains(&character)
    }

    pub fn is_allowed_sequence(&self, sequence: &str) -> bool 
    {
        sequence.chars().all(|c| self.is_allowed_symbol(c))
    }

    pub fn is_correct_sequence(&self, sequence: &str) -> bool 
    {
        sequence.chars().all(|c| self.is_correct_symbol(c))
    }

    pub fn from(&self, sequence: &str) -> Result<Sequence, String>
    {
        Sequence::new(sequence, self)
    }
}


impl<'a> Polymerase<'a>
{
    pub fn new(alphabet: &'a Alphabet, table: HashMap<char, char>) -> Result<Self, String>
    {
        Ok( Polymerase { alphabet : alphabet, table : table } )
    }

    fn _complement(&self, sequence: &Sequence, reverse: bool) -> Result<String, String>
    {
        // check that alphabets match
        if !std::ptr::eq(self.alphabet, sequence.alphabet)
        {
            return Err("Alphabet does not match".to_owned());
        }
        
        let mut cmp_sequence = 
            if reverse
            {
                sequence.sequence
                    .chars()
                    .rev()
                    .map(|c| self.table.get(&c).cloned().unwrap_or(c))
                    .collect::<String>()
            }
            else
            {
                sequence.sequence
                    .chars()
                    .map(|c| self.table.get(&c).cloned().unwrap_or(c))
                    .collect::<String>()
            };

        Ok( cmp_sequence )
    }

    pub fn complement(&self, sequence: &Sequence) -> Result<Sequence, String>
    {
        let cmp_sequence = self._complement(sequence, false)?;
        Sequence::from(cmp_sequence, self.alphabet)
    }

    pub fn reverse_complement(&self, sequence: &Sequence) -> Result<Sequence, String>
    {
        let mut cmp_sequence = self._complement(sequence, true)?;
        Sequence::from(cmp_sequence, self.alphabet)
    }
}

impl<'a> Transcriptase<'a>
{
    pub fn new(src_alphabet: &'a Alphabet, dst_alphabet: &'a Alphabet, table: HashMap<char,char>) -> Result<Self, String>
    {
        // check that missing keys and value are present in both alphabets!!!
        
        //check that table is consistent with the source alphabet
        if !table.keys().all(|k| src_alphabet.is_correct_symbol(*k))
        {
            return Err("Table keys contain symbols that are not CORRECT symbols of the source alphabet".to_owned());
        }

        //check that table is consistent with the destination alphabet
        if !table.values().all(|v| dst_alphabet.is_correct_symbol(*v))
        {
            return Err("Table values contain symbols that are not CORRECT symbols of the destination alphabet".to_owned());
        }

        Ok( Self { src_alphabet : src_alphabet, dst_alphabet : dst_alphabet, table : table } )
    }

    pub fn transcribe(&self, sequence: &Sequence) -> Result<Sequence, String>
    {
        // check that source alphabets match
        if ! std::ptr::eq(self.src_alphabet, sequence.alphabet)
        {
            return Err("Source alphabet does not match".to_owned());
        }

        let mut tsc_sequence = sequence.sequence
            .chars()
            .map(|c| self.table.get(&c).cloned().unwrap_or(c))
            .collect::<String>();

        Sequence::from(tsc_sequence, self.dst_alphabet)
    }
}

impl<'a> Ribosome<'a>
{
    pub fn new(src_alphabet: &'a Alphabet, dst_alphabet: &'a Alphabet, 
               table: HashMap<&'a str, char>, stop_codons: HashSet<&'a str>) -> Result<Self, String>
    {
        let size = table.keys().next().unwrap().len();

        // all codons should have the same size
        if !table.keys().all(|k| k.len() == size) || !stop_codons.iter().all(|k| k.len() == size)
        {
            return Err("Codons have different size".to_owned());
        }

        // codons' symbols should be correct symbols of the source alphabet
        if !table.keys().all(|k| k.chars().all(|c| src_alphabet.is_correct_symbol(c))) ||
           !stop_codons.iter().all(|k| k.chars().all(|c| src_alphabet.is_correct_symbol(c)))
        {
            return Err("Codons contain symbols that are not CORRECT symbols of the source alphabet".to_owned());
        }

        // check that table is consistent with the destination alphabet
        if !table.values().all(|v| dst_alphabet.is_correct_symbol(*v))
        {
            return Err("Table contain symbols that are not CORRECT symbols of the destination alphabet".to_owned());
        }

        Ok( Self { src_alphabet : src_alphabet, dst_alphabet : dst_alphabet, 
                   table : table, size : size, stop_codons : stop_codons } )
    }

    pub fn translate(&self, sequence: &Sequence) -> Result<Sequence, String>
    {
        // check that source alphabets match
        if ! std::ptr::eq(self.src_alphabet, sequence.alphabet)
        {
            return Err("Source alphabet does not match".to_owned());
        }

        // check that sequence length is multiple of codon size
        if sequence.sequence.len() % self.size != 0
        {
            return Err("Sequence length is not multiple of codon size".to_owned());
        }

        let mut trl_sequence = String::new();
        for i in (0..sequence.sequence.len()).step_by(self.size)
        {
            let codon = &sequence.sequence[i..i+self.size];
            if self.stop_codons.contains(codon) { break; }
            trl_sequence.push(*self.table.get(codon).ok_or("Codon not found")?);
        }

        // unnecessary check of the destination alphabet? think how to fix it
        Sequence::from(trl_sequence, self.dst_alphabet)
    }
}

// add tests for typical proteins from NCBI