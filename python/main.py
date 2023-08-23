from bio_api.sequence import DNA, RNA, Protein

dna = DNA("ATGCGA")
rna = dna.transcribe()

dna_complement = dna.complement()
rna_complement = rna.complement()

dna_reverse_complement = dna.reverse_complement()
rna_reverse_complement = rna.reverse_complement()

prt_from_dna = dna.translate()
prt_from_rna = rna.translate()

print(f"DNA: {dna.sequence}")
print(f"RNA: {rna.sequence}")
print(f"Protein from DNA: {prt_from_dna.sequence}")
print(f"Protein from RNA: {prt_from_rna.sequence}")
print(f"DNA complement: {dna_complement.sequence}")
print(f"RNA complement: {rna_complement.sequence}")
print(f"DNA reverse complement: {dna_reverse_complement.sequence}")
print(f"RNA reverse complement: {rna_reverse_complement.sequence}")


