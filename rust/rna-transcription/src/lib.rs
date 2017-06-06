#[derive(Debug)]
pub struct RibonucleicAcid {
    rna: String
}

#[derive(Debug)]
pub struct DeoxyribonucleicAcid {
    dna: String
}

impl RibonucleicAcid {
    pub fn new(rna: &str) -> RibonucleicAcid {
        RibonucleicAcid { rna: rna.to_string() }
    }
}

impl PartialEq for RibonucleicAcid {
    fn eq(&self, other: &RibonucleicAcid) -> bool {
        self.rna == other.rna
    }
}

impl DeoxyribonucleicAcid {
    pub fn new(dna: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid { dna: dna.to_string() }
    }

    pub fn to_rna(&self) -> Result<RibonucleicAcid, &'static str> {
        self.dna
            .chars()
            .map(|c|
                 match c {
                     'G' => Ok('C'),
                     'C' => Ok('G'),
                     'T' => Ok('A'),
                     'A' => Ok('U'),
                     _ => Err("Bad Input")
                 })
            .collect::<Result<String,&'static str>>()
            .map(|rna| RibonucleicAcid::new(&rna))
    }
}
