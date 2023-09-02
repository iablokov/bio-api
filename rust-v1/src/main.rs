use std::collections::{HashMap, HashSet};

use rust_v1::seq::alphabet::{Alphabet, Transcriptase, Polymerase, Ribosome};


fn main() 
{
    let DNA: Alphabet = Alphabet::new(vec!['A', 'T', 'G', 'C'], vec!['N']);
    let RNA: Alphabet = Alphabet::new(vec!['A', 'U', 'G', 'C'], vec!['N']);
    let PRT: Alphabet = Alphabet::new(vec!['A', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'K', 'L', 'M',
                                                    'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'Y'],vec![]);

    let DNA_TO_RNA = 
        Transcriptase::new(&DNA, &RNA, HashMap::from([('T', 'U')])).unwrap();
    let RNA_TO_DNA = 
        Transcriptase::new(&RNA, &DNA, HashMap::from([('U', 'T')])).unwrap();
    
    let DNA_COMP = Polymerase::new(&DNA, HashMap::from([('A', 'T'), ('T', 'A'), ('G', 'C'), ('C', 'G')])).unwrap();

    let DNA_TO_PROT = Ribosome::new(&DNA, &PRT,
            HashMap::from([("TTT", 'F'), ("TTC", 'F'), ("TTA", 'L'), ("TTG", 'L'), ("TCT", 'S'), 
                           ("TCC", 'S'), ("TCA", 'S'), ("TCG", 'S'), ("TAT", 'Y'), ("TAC", 'Y'), 
                           ("TGT", 'C'), ("TGC", 'C'), ("GGA", 'G'), ("GGG", 'G'), ("GGC", 'G'),
                           ("TGG", 'W'), ("CTT", 'L'), ("CTC", 'L'), ("CTA", 'L'), ("CTG", 'L'), 
                           ("CCT", 'P'), ("CCC", 'P'), ("CCA", 'P'), ("CCG", 'P'), ("CAT", 'H'), 
                           ("CAC", 'H'), ("CAA", 'Q'), ("CAG", 'Q'), ("CGT", 'R'), ("CGC", 'R'), 
                           ("CGA", 'R'), ("CGG", 'R'), ("ATT", 'I'), ("ATC", 'I'), ("ATA", 'I'), 
                           ("ATG", 'M'), ("ACT", 'T'), ("ACC", 'T'), ("ACA", 'T'), ("ACG", 'T'), 
                           ("AAT", 'N'), ("AAC", 'N'), ("AAA", 'K'), ("AAG", 'K'), ("AGT", 'S'), 
                           ("AGC", 'S'), ("AGA", 'R'), ("AGG", 'R'), ("GTT", 'V'), ("GTC", 'V'), 
                           ("GTA", 'V'), ("GTG", 'V'), ("GCT", 'A'), ("GCC", 'A'), ("GCA", 'A'), 
                           ("GCG", 'A'), ("GAT", 'D'), ("GAC", 'D'), ("GAA", 'E'), ("GAG", 'E'), 
                           ("GGT", 'G')]), HashSet::from(["TAA", "TAG", "TGA"])).unwrap();

    let dna = DNA.from("ATGCTGATCGGGCGCGTG").unwrap();
    let rna = DNA_TO_RNA.transcribe(&dna).unwrap();
    let dna_c = DNA_COMP.complement(&dna).unwrap();
    let dna_rc = DNA_COMP.reverse_complement(&dna).unwrap();
    let prt = DNA_TO_PROT.translate(&dna).unwrap();

    println!("DNA    : {}", dna.sequence);
    println!("RNA    : {}", rna.sequence);
    println!("DNA C  : {}", dna_c.sequence);
    println!("DNA RC : {}", dna_rc.sequence);
    println!("PRT    : {}", prt.sequence);

}