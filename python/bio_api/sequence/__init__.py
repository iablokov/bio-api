from typing import Callable, Tuple, Optional
from .alphabet import *

# 1. add sequence slices

NucFactory = Callable[[str], "NucSequence"]
PrtFactory = Callable[[str], "PrtSequence"]

class TranscriptionData:
    table   : TranscriptionTable
    factory : Optional[NucFactory]

    def __init__(self, table: dict[str, str], factory: Optional[NucFactory] = None):
        self.table   = table
        self.factory = factory

class TranslationData:
    table   : TranslationTable
    factory : Optional[PrtFactory]

    def __init__(self, table: dict[str, str], factory: Optional[PrtFactory] = None):
        self.table   = table
        self.factory = factory

class ComplementData:
    table   : ComplementTable
    factory : Optional[NucFactory]

    def __init__(self, table: dict[str, str], factory: Optional[NucFactory] = None):
        self.table   = table
        self.factory = factory

class Sequence:
    sequence : str
    alphabet : Alphabet

    def __init__(self, sequence: str, alphabet: Alphabet):
        self.sequence = sequence
        self.alphabet = alphabet

class NucSequence(Sequence):
    tsc_data : TranscriptionData
    tla_data : TranslationData
    cmp_data : ComplementData

    def __init__(self, sequence : str, 
                       alphabet : Alphabet,
                       tsc_data : TranscriptionData, 
                       tla_data : TranslationData,
                       cmp_data : ComplementData):
        
        super().__init__(sequence, alphabet)
        self.tsc_data = tsc_data
        self.tla_data = tla_data
        self.cmp_data = cmp_data

    def transcribe(self) -> "NucSequence":
        sequence = self.tsc_data.table.transcribe(self.sequence)
        return self.tsc_data.factory(sequence)

    def translate(self) -> "PrtSequence":
        sequence = self.tla_data.table.translate(self.sequence)
        return self.tla_data.factory(sequence)

    def complement(self) -> "NucSequence":
        sequence = self.cmp_data.table.complement(self.sequence)
        return self.cmp_data.factory(sequence)
    
    def reverse(self) -> "NucSequence":
        sequence = self.sequence[::-1]
        return self.cmp_data.factory(sequence)

    def reverse_complement(self) -> "NucSequence":
        sequence = self.cmp_data.table.complement(self.sequence[::-1])
        return self.cmp_data.factory(sequence)

class PrtSequence(Sequence):

    def __init__(self, sequence: str, alphabet: Alphabet):
        super().__init__(sequence, alphabet)


#def make_protein(prt_alphabet : Alphabet) -> PrtFactory:
    
def make_triple(dna_alphabet : Alphabet, 
                rna_alphabet : Alphabet,
                prt_alphabet : Alphabet,
                dna_to_rna   : Optional[TranscriptionTable],
                rna_to_dna   : Optional[TranscriptionTable],
                dna_to_prt   : Optional[TranslationTable],
                rna_to_prt   : Optional[TranslationTable],
                dna_to_dna   : Optional[ComplementTable], 
                rna_to_rna   : Optional[ComplementTable]
               ) -> Tuple[NucFactory, NucFactory, PrtFactory]:
    
    dna_tsc_data = TranscriptionData(dna_to_rna) if dna_to_rna is not None else None
    rna_tsc_data = TranscriptionData(rna_to_dna) if rna_to_dna is not None else None
    dna_tla_data = TranslationData(dna_to_prt)   if dna_to_prt is not None else None
    rna_tla_data = TranslationData(rna_to_prt)   if rna_to_prt is not None else None 
    dna_cmp_data = ComplementData(dna_to_dna)    if dna_to_dna is not None else None
    rna_cmp_data = ComplementData(rna_to_rna)    if rna_to_rna is not None else None

    DNA: NucFactory = lambda sequence: NucSequence(sequence, dna_alphabet, dna_tsc_data, dna_tla_data, dna_cmp_data)
    RNA: NucFactory = lambda sequence: NucSequence(sequence, rna_alphabet, rna_tsc_data, rna_tla_data, rna_cmp_data)
    PRT: PrtFactory = lambda sequence: PrtSequence(sequence, prt_alphabet)

    dna_tsc_data.factory = RNA
    rna_tsc_data.factory = DNA

    dna_tla_data.factory = PRT
    rna_tla_data.factory = PRT

    dna_cmp_data.factory = DNA
    rna_cmp_data.factory = RNA

    return DNA, RNA, PRT

DNA, RNA, Protein = make_triple(DNA_ALPHABET, RNA_ALPHABET, PRT_ALPHABET,
                                DNA_TO_RNA, RNA_TO_DNA,
                                DNA_TO_PRT, RNA_TO_PRT,
                                DNA_TO_DNA, RNA_TO_RNA)
