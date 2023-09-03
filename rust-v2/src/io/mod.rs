// file = RecordsFile::load("q.fasta");
// stream = RecordsStream<FASTA<DNA>>::from("q.fasta")
// for record in stream {}

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use core::marker::PhantomData;

use crate::seq::{Sequence, Alphabet, Seq};


// Fasta: */Record/Reader/Writer

pub struct Fasta<A: Alphabet + 'static>
{
    records : Vec<FastaRecord<A>>
}

pub struct FastaReader<A>
{
    reader : BufReader<File>,
    buffer : String,
    _p     : PhantomData<A>
}
pub struct FastaWriter<A: Alphabet>
{
    writer : BufWriter<File>,
    _p     : PhantomData<A>
}

impl<A: Alphabet + Default + 'static> FastaWriter<A> 
{
    pub fn write(&mut self, record: &FastaRecord<A>) -> Result<(), ()>
    {
        self.writer.write_all(record.to_text().as_bytes());

        Ok(())
    }
}

impl<A: Alphabet + Default> Fasta<A>
{
    pub fn new() -> Fasta<A>
    {
        Fasta { records : Vec::new() }
    }
    
    pub fn read_from(path: &str) -> FastaReader<A>
    {
        let file   : File            = File::open(path).unwrap();
        let reader : BufReader<File> = BufReader::new(file);

        FastaReader { reader : reader, buffer : String::new(), _p : PhantomData }
    }

    pub fn write_to(path: &str) -> FastaWriter<A>
    {
        let file   : File            = File::create(path).unwrap();
        let writer : BufWriter<File> = BufWriter::new(file);

        FastaWriter { writer : writer, _p : PhantomData }
    }

    pub fn load_from(path: &str) -> Fasta<A>
    {
        Fasta { records : Self::read_from(path).collect() }
    }

    pub fn save_to(&self, path: &str) -> Result<(),()>
    {
        let mut writer = Self::write_to(path);
        self.records.iter().for_each(|r| { writer.write(r); });

        Ok(())
    }

    pub fn push(&mut self, record: FastaRecord<A>)
    {
        self.records.push(record);
    }

    pub fn iter(&self) -> std::slice::Iter<'_, FastaRecord<A>>
    {
        self.records.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, FastaRecord<A>>
    {
        self.records.iter_mut()
    }
}

impl<A: Alphabet + Default + 'static> Iterator for FastaReader<A>
{
    type Item = FastaRecord<A>;

    fn next(&mut self) -> Option<Self::Item>
    {
        self.buffer.clear();
        let mut header = String::new();
        let mut sequence = String::new();
        
        loop 
        {
            let bytes_read = self.reader.read_line(&mut self.buffer).unwrap();
            if bytes_read == 0 { break; }

            let line = self.buffer.trim_end().to_string();
            self.buffer.clear();

            if line.is_empty() { continue; }
            
            match line.as_bytes()[0] 
            {
                b'>' => 
                {
                    if !header.is_empty() 
                    { 
                        let record = FastaRecord { head : header, body : Sequence::<A>::new(sequence.clone()).unwrap() };
                        sequence.clear();
                        return Some( record ); 
                    }
                    header = line[1..].to_string();
                }
                _ => { sequence.push_str(&line); }
            }
        }

        if !header.is_empty() { return Some( FastaRecord { head : header, body : Sequence::<A>::new(sequence).unwrap() } ); }

        None
    }
}

pub struct FastaRecord<A: Alphabet + 'static>
{
    pub head : String,
    pub body : Box<Sequence<A>>
}

impl<A: Alphabet + Default + 'static> FastaRecord<A>
{
    pub fn to_text(&self) -> String
    {
        format!(">{}\n{}\n", self.head, self.body.sequence_ref())
    }
}
