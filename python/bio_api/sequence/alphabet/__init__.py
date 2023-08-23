class Alphabet:
    allowed = set()
    correct = set()

    def __init__(self, correct: list[str], allowed: list[str]):
        self.allowed = set(allowed)
        self.correct = set(correct)
    
    def is_allowed(self, char: str) -> bool:
        return char in self.allowed or char in self.correct

    def is_correct(self, char: str) -> bool:
        return char in self.correct
    
class TranscriptionTable:
    src   : Alphabet
    dst   : Alphabet
    table : dict[str, str]

    def __init__(self, src: Alphabet, dst: Alphabet, table: dict[str, str]):
        self.src = src
        self.dst = dst

        # check if table is valid
        for key in table.keys():
            if not src.is_allowed(key):
                raise ValueError(f"Invalid key in transcription table: {key}")
            if not dst.is_allowed(table[key]):
                raise ValueError(f"Invalid value in transcription table: {table[key]}")

        self.table = table

    def transcribe(self, sequence: str) -> str:
        return ''.join([self.table.get(char,char) for char in sequence])

# add option to translate to N/./- if a codon was not found
class TranslationTable:
    src   : Alphabet
    dst   : Alphabet
    size  : int
    table : dict[str, str]

    def __init__(self, src: Alphabet, dst: Alphabet, table: dict[str, str]):
        self.src = src
        self.dst = dst

        self.size = len(next(iter(table.keys())))

        # check if table is valid (todo later)

        self.table = table

    def translate(self, sequence: str) -> str:
        return ''.join(self.table.get(sequence[i:i+self.size], '-') for i in range(0, len(sequence), self.size))
    
class ComplementTable:
    alphabet  : Alphabet
    table     : dict[str, str]

    def __init__(self, alphabet: Alphabet, table: dict[str, str]):
        self.alphabet = alphabet

        # check if table is valid
        for key in table.keys():
            if not alphabet.is_allowed(key):
                raise ValueError(f"Invalid key in complement table: {key}")
            if not alphabet.is_allowed(table[key]):
                raise ValueError(f"Invalid value in complement table: {table[key]}")
        
        if len(table.keys()) != len(table.values()):
            raise ValueError("Complement table is not a bijection")
        
        if len(table.keys()) != len(alphabet.correct):
            raise ValueError(f"Complement table does not cover all correct characters {alphabet.correct}")
        
        self.table = table

    def complement(self, sequence: str) -> str:
        return ''.join([self.table[char] for char in sequence])
    

DNA_ALPHABET = Alphabet(['A', 'C', 'G', 'T'], ['N'])
RNA_ALPHABET = Alphabet(['A', 'C', 'G', 'U'], ['N'])
PRT_ALPHABET = Alphabet(['A', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'K', 'L', 'M'], [])

DNA_TO_RNA = TranscriptionTable(DNA_ALPHABET, RNA_ALPHABET, {'T' : 'U'})
RNA_TO_DNA = TranscriptionTable(RNA_ALPHABET, DNA_ALPHABET, {'U' : 'T'})

DNA_TO_PRT = TranslationTable(DNA_ALPHABET, PRT_ALPHABET, {'ATG' : 'M'})
RNA_TO_PRT = TranslationTable(RNA_ALPHABET, PRT_ALPHABET, {'AUG' : 'M'})

DNA_TO_DNA = ComplementTable(DNA_ALPHABET, {'A' : 'T', 'T' : 'A', 'G' : 'C', 'C' : 'G'})
RNA_TO_RNA = ComplementTable(RNA_ALPHABET, {'A' : 'U', 'U' : 'A', 'G' : 'C', 'C' : 'G'})
