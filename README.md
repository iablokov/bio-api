# Bio API. 
This is an experimental repository aimed for testing different strategies on how to design the API for a biological library in different languages.
One of the purposes of this research is to make a flexible API that is universal throughout languages that follow different paradigms, such as Rust and Python.
The ultimate goal of this project is to develop a biological library in Rust that can be easily wrapped into a corresponding Python's package with the shared codebase (in Rust).

## Python version

The main idea behind the appraoch used here consists in providing a significant amount of flexibility when working with biological sequences. This is achieved by allowing a user to define their own alphabets (`class Alphabet`) and alphabet transformations:
* Transcription, i.e., a map from one alphabet to another.
* Translation, i.e., a map from k-mers of one alphabet to symbols of another.
* Complement, i.e., an automorphism of an alphabet (a bijection of an alphabet to itself).

Each `Sequence` is constructed not only with the `sequence` string itself, but also requires the specification of respective 'alphabet'. Its child class `NucSequence` provides additional functionality for nucleotide sequences through the specification of respective transformation tables, i.e., `TranscriptionTable`, `TranslationTable`, and `ComplementTable`. When `.transcribe()`, `.translate()` or `.complement()` method of `NucSequence` is called, the `sequence` string of `Sequence` is being transformed into a string of target alphabet (specified in the respective transformation table). This transformed `sequence` should then be wrapped into a `NucSequence` class together not only with the target `Alphabet` but also with the transformations of this alphabet. For example, when a DNA sequence is transcribed into its corresponding RNA sequence, the latter should be constructed with, e.g., `TranscriptionTable` that allows to reverse transcribe it back to the original DNA.

In order to hide all this complexity and provide a simple API for working with standard biological sequences, such as DNA, RNA, and Proteins, the corresponding factories are used. They are designed to be simple functions of only one string argument (i.e., `sequence`). The rest, e.g., assignment of an `Alphabet` and its transformations, happends "under the hood". These factories themeselves are created using another factory `make_triple(...)` out of RNA/DNA/Protein alphabets and their transformations. For popular alphabets (e.g., standard, IUPAC, etc), the corresponding factories are already instantiated, e.g., `DNA`, `RNA`, and `Protein`. Their main purpose is to simplify the creation of `Sequence` objects (such as `NucSequence`) from `sequence` strings, e.g.
* when applying transformations (`transcribe/translate/complement`) on existing `Sequences` objects
* when reading sequences from data streams (e.g., a file storage) using `FastaIO` and/or `FastqIO` classes.
