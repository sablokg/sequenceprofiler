#[derive(Debug, Default, Clone, PartialOrd, PartialEq)]

pub struct Genomeiter {
    pub header: String,
    pub sequence: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct CollectIter {
    pub name: String,
    pub namenext: String,
    pub id: String,
    pub idnext: String,
    pub count: usize,
    pub shared: usize,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct ProfileKmer {
    pub name: String,
    pub nextname: String,
    pub sequence: Vec<String>,
    pub nextsequence: Vec<String>,
    pub count: usize,
    pub total: usize,
    pub ratio: f32,
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CountIllumina {
    pub kmer: String,
    pub count: usize,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VecStore {
    pub id: String,
    pub numberstart: usize,
    pub numberend: usize,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VecStoreAnalyze {
    pub seqid: String,
    pub id: String,
    pub kmer: String,
    pub numberstart: usize,
    pub numberend: usize,
}
