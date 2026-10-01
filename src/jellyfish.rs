use crate::filestruct::CountIllumina;
use crate::filestruct::VecStore;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
Gaurav Sablok,
gsablok@proton.me
*/

pub fn kmer_jellyfish(path: &str, kmer: usize) -> Result<String, Box<dyn Error>> {
    let f = File::open(path).expect("file not present");
    let read = BufReader::new(f);
    for i in read.lines() {
        let line = i.expect("line not present");
        let mut header: Vec<&str> = vec![];
        let mut sequence: Vec<&str> = vec![];
        if line.starts_with("@") {
            header.push(&line)
        }
        if line.starts_with("A")
            || line.starts_with("T")
            || line.starts_with("G")
            || line.starts_with("C")
        {
            sequence.push(&line)
        }
        let mut sequence_iter: Vec<&str> = vec![];
        for i in 0..sequence.len() {
            let i = sequence[i];
            for j in 0..i.len() - kmer {
                sequence_iter.push(&i[j..j + kmer])
            }
            let mut fileall = File::create("allkmerunique.txt").expect("file not present");
            for i in sequence_iter.iter() {
                writeln!(fileall, "{}", i).expect("file not found");
            }
            let hash_kmer: HashSet<_> = sequence_iter.iter().collect();
            let mut finalvec: Vec<&str> = Vec::new();
            for i in hash_kmer.into_iter() {
                finalvec.push(i)
            }
            let mut path = File::create("kmerunique.txt").expect("file not present");
            for i in finalvec.iter() {
                writeln!(path, "{}", i).expect("file not found");
            }
            let mut mutunique = Vec::new();
            let uniquehold = File::open("kmerunique.txt").expect("file not present");
            let uniqueread = BufReader::new(uniquehold);
            for i in uniqueread.lines() {
                let appendline = i.expect("line not present");
                mutunique.push(appendline)
            }
            let mut allkmer = Vec::new();
            let allopen = File::open("allkmerunique.txt").expect("file not present");
            let allread = BufReader::new(allopen);
            for i in allread.lines() {
                let appendline = i.expect("line not present");
                allkmer.push(appendline)
            }
            let mut mutunique = Vec::new();
            let uniquehold = File::open("kmerunique.txt").expect("file not present");
            let uniqueread = BufReader::new(uniquehold);
            for i in uniqueread.lines() {
                let appendline = i.expect("line not present");
                mutunique.push(appendline)
            }

            let mut unique_count = Vec::new();
            for i in mutunique.iter() {
                let count_add = allkmer.iter().filter(|&x| *x == *i).count();
                unique_count.push(CountIllumina {
                    kmer: i.to_string(),
                    count: count_add,
                })
            }
            let mut unique_file = File::create("histogram-count.txt").expect("file not present");
            for i in unique_count.iter() {
                writeln!(unique_file, "{}\t{}", i.kmer, i.count).expect("file not found");
            }
            let mut indexstorestart = Vec::new();
            for i in sequence.iter() {
                for j in mutunique.iter() {
                    let indexout = i.find(j).unwrap();
                    let indexoutend = i.find(j).unwrap() + j.len();
                    indexstorestart.push(VecStore {
                        id: i.to_string(),
                        numberstart: indexout,
                        numberend: indexoutend,
                    });
                }
            }
        }
    }
    Ok("The result for the jellyfish have been written".to_string())
}
