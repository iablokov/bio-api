pub mod alphabet;

use crate::seq::alphabet::Alphabet;

pub struct Sequence<'a>
{
    pub sequence : String,
    alphabet : &'a Alphabet,
}

impl<'a> Sequence<'a>
{
    pub fn new(sequence: &str, alphbaet: &'a Alphabet) -> Result<Self, String>
    { 
        if !alphbaet.is_allowed_sequence(sequence)
        {
            return Err(format!("Sequence contains not allowed symbols"));
        }
        Ok( Sequence { sequence : sequence.to_owned(), alphabet : alphbaet } )
    }

    pub fn from(sequence: String, alphbaet: &'a Alphabet) -> Result<Self, String>
    { 
        if !alphbaet.is_allowed_sequence(&sequence)
        {
            return Err(format!("Sequence contains not allowed symbols"));
        }
        Ok ( Sequence { sequence : sequence, alphabet : alphbaet } )
    }

    // sequence methods

    pub fn len(&self) -> usize { self.sequence.len() }
    pub fn reverse(&self) -> Self { Self::from(self.sequence.chars().rev().collect(), self.alphabet).unwrap() }
    
    // can be improved with 2 pointers
    pub fn reverse_mut(&mut self) { self.sequence = self.sequence.chars().rev().collect(); }

    

}
