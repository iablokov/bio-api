use rust_v2::seq::{Seq, Sequence, DNA, RNA, Alphabet};
use rust_v2::io::{Fasta, FastaRecord};

fn main() 
{
    let dna1 = Sequence::<DNA>::new("ATGC".to_owned()).unwrap();
    let dna2 = Sequence::<DNA>::new("CGAT".to_owned()).unwrap();
    let rna1 = dna1.to_rna();
    let rna2 = dna2.to_rna();

    println!("{} = {}", dna1.seq_type(), dna1.sequence());
    println!("{} = {}", dna2.seq_type(), dna2.sequence());
    println!("{} = {}", rna1.seq_type(), rna1.sequence());
    println!("{} = {}", rna2.seq_type(), rna2.sequence());
    println!("");

    let sequences = vec![dna1, dna2];

    let mut fasta = Fasta::<DNA>::new();

    for seq in sequences 
    {
        fasta.push(FastaRecord { head: "seq".to_owned(), body: seq });
    }
    fasta.save_to("f.fasta");

    println!("Reading from file stream...");
    let stream = Fasta::<DNA>::read_from("f.fasta");
    for record in stream
    {
        println!("H: {}", record.head);
        println!("B: {}", record.body.sequence());
    }
    println!("");
    println!("Loading from file...");
    let mut storage = Fasta::<DNA>::load_from("f.fasta");
    for record in storage.iter_mut()
    {
        println!("H: {}", record.head);
        println!("B: {}", record.body.sequence());
    }

    return;

}
