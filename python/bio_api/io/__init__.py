from bio_api.sequence import Sequence
from typing import Callable, Iterator, TypeVar

# positional metadata?
# interval metadata?

class SequenceRecord:
    sequence : Sequence
    def __init__(self, sequence: Sequence) -> None:
        self.sequence = sequence

class FastaRecord(SequenceRecord):
    header : str
    def __init__(self, header: str, sequence: Sequence) -> None:
        super().__init__(sequence)
        self.header = header

class FastaIO:
    factory : Callable[[str], Sequence]
    def __init__(self, factory: Callable[[str], Sequence]) -> None:
        self.factory = factory
    
    def read(self, path_fasta: str) -> Iterator[FastaRecord]:
        with open(path_fasta, "r") as f_fasta:
            header   = None
            sequence = ""
            for line in f_fasta:
                if line.startswith(">"):
                    if header is not None:
                        yield FastaRecord(header, self.factory(sequence))
                    header   = line[1:].strip()
                    sequence = ""
                else:
                    sequence += line.strip()
            if header is not None:
                yield FastaRecord(header, self.factory(sequence))

    def load(self, path_fasta: str) -> "SequenceFile":
        fasta = SequenceFile()
        for record in self.read(path_fasta):
            fasta.records.append(record)
        return fasta

class SequenceFile:
    records : list[SequenceRecord]
    def __init__(self) -> None:
        self.records = []

class SequenceSample:
    pass

class SequenceSampleSingle(SequenceSample):
    file_single  : SequenceFile

class SequenceSamplePaired(SequenceSample):
    file_forward : SequenceFile
    file_reverse : SequenceFile
