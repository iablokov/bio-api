use std::ops::Range;
use std::fmt;
use std::error::Error;

const ALPHABET_SIZE: usize = 128;

#[derive(Debug)]
struct AlphabetError;
impl Error for AlphabetError {}
impl fmt::Display for AlphabetError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "Invalid character")
    }
}
 
/// This macro creates a boolean map for a given alphabet
macro_rules! alphabet_map 
{
    ($($symbol:expr),*) => {
    {
        let mut arr = [false; ALPHABET_SIZE];
        $(
            arr[$symbol as usize] = true;
        )*
        arr
    }};
}


pub trait Alphabet
{ 
    fn seq_type(&self) -> &str;
    const CORRECT: [bool; ALPHABET_SIZE];
    const ALLOWED: [bool; ALPHABET_SIZE];

    fn is_correct(sequence: &str) -> bool
    {
        sequence.chars().all(|s| Self::CORRECT[s as usize])
    }
}


pub struct Sequence<A: Alphabet>
{
    sequence : String,
    alphabet : A,
}

#[derive(Default)]
pub struct DNA {}
impl Alphabet for DNA
{
    fn seq_type(&self) -> &str { "DNA" }
    const CORRECT: [bool; ALPHABET_SIZE] = alphabet_map!['A', 'T', 'G', 'C'];
    const ALLOWED: [bool; ALPHABET_SIZE] = alphabet_map!['N'];
}

#[derive(Default)]
pub struct RNA {}
impl Alphabet for RNA
{
    fn seq_type(&self) -> &str { "RNA" }
    const CORRECT: [bool; ALPHABET_SIZE] = alphabet_map!['A', 'U', 'G', 'C'];
    const ALLOWED: [bool; ALPHABET_SIZE] = alphabet_map!['N'];
}



pub trait Seq
{
    fn sequence(&self)     -> String;
    fn sequence_ref(&self) -> &str;
    fn seq_type(&self)     -> &str;
    fn reverse(&self)      -> Box<dyn Seq>;
}

impl<A: Alphabet + Default + 'static> Seq for Sequence<A>
{
    fn sequence(&self)     -> String { self.sequence.clone() }
    fn sequence_ref(&self) -> &str   { &self.sequence }
    fn seq_type(&self)     -> &str   { self.alphabet.seq_type() }
    fn reverse(&self)      -> Box<dyn Seq> 
    {
        Box::new( Sequence { sequence : self.sequence.chars().rev().collect(), alphabet : A::default() } )
    }
}


impl<A: Alphabet + Default> Sequence<A> 
{
    pub fn new(sequence: String) -> Option<Box<Self>>
    {
        if A::is_correct(&sequence)
        {
            Some(Box::new( Sequence { sequence : sequence, alphabet : A::default() } ))
        }
        else { None }
    }
}

/// DNA-specific methods
impl Sequence<DNA>
{
    pub fn to_rna(&self) -> Box<Sequence<RNA>>
    {
        Box::new(Sequence::<RNA> { sequence: self.sequence.replace("T", "U"), alphabet: RNA::default() })
    }
}

/// RNA-specific methods
impl Sequence<RNA>
{
    pub fn to_dna(&self) -> Box<Sequence<DNA>>
    {
        Box::new(Sequence::<DNA> { sequence: self.sequence.replace("U", "T"), alphabet: DNA::default() })
    }
}


// Principles:
// 1. tables and alphabets should be dynamic, i.e., objects
// 2. for each Alphabet, there should be possible to assign Translation/Transcription/Complement traits

// let DNA_ALPHABET = Alphabet::new(['A', 'T', 'G', 'C'], ['N']);
// let RNA_ALPHABET = Alphabet::new(['A', 'U', 'G', 'C'], ['N']);
// DNA_ALPHABET.translated_to(&PROTEIN_ALPHABET, HashMap::from(["ATG",'M']));
// DNA_ALPHABET.transcribed_to(&RNA_ALPHABET, HashMap::from(['T','U']));

// let dna_sequence = Sequence::new("ATGC".to_owned(), &DNA_ALPHABET).unwrap();

// dna_sequence.transcribe();

/* 
pub trait Trans
{
    // should return an instance of RNA
    fn to(&self) -> dyn Alphabet;
}
impl Trans for DNA 
{
    fn to(&self) -> { RNA::default() }
}

pub trait Transcription 
{
    fn transcribe(&self) -> Box<dyn Seq>;
}

impl<A: Alphabet + Trans + Default + 'static> Transcription for Sequence<A>
{
    fn transcribe(&self) -> Box<dyn Seq>
    {
        Box::new(Sequence::<A> { sequence: self.sequence.replace("T", "U"), alphabet: A::default() })
    }
}
*/

// Protein, TranslationTable, ...