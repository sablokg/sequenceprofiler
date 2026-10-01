use crate::filestruct::Genomeiter;
use crate::filestruct::ProfileKmer;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
Gaurav Sablok,
gsablok@proton.me
*/

pub fn profilesseq(path: &str, kmer: &str) -> Result<String, Box<dyn Error>> {
    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut sequencevector: Vec<String> = Vec::new();
    let mut headervector: Vec<String> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("file not found");
        if line.starts_with(">") {
            headervector.push(line.replace(">", ""));
        } else if !line.starts_with(">") {
            sequencevector.push(line);
        }
    }
    let mut combinedinfo: Vec<Genomeiter> = Vec::new();
    for i in 0..headervector.len() {
        combinedinfo.push(Genomeiter {
            header: headervector[i].clone(),
            sequence: sequencevector[i].clone(),
        })
    }

    let mut seqbtreemap: Vec<(String, (String, Vec<String>))> = Vec::new();
    for i in combinedinfo.iter() {
        let windowkmer: Vec<_> = i
            .sequence
            .chars()
            .map(String::from)
            .collect::<Vec<_>>()
            .windows(kmer.parse::<usize>().unwrap())
            .map(|x| x.join("").to_string())
            .collect::<Vec<_>>();
        let sequencehash: Vec<String> = windowkmer
            .into_iter()
            .collect::<HashSet<String>>()
            .into_iter()
            .collect::<Vec<_>>()
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();
        seqbtreemap.push((i.header.clone(), (i.sequence.clone(), sequencehash)));
    }

    let mut newbase: Vec<ProfileKmer> = Vec::new();
    for i in 0..seqbtreemap.len() - 1 {
        let mut counterbase: Vec<usize> = Vec::new();
        for j in 0..seqbtreemap[i].1 .1.len() {
            let mut countkmer: usize = 0usize;
            let itervalue = seqbtreemap[i].1 .1[j].clone();
            if seqbtreemap[i + 1].1 .1.contains(&itervalue.to_string()) {
                countkmer += 1usize;
            }
            counterbase.push(countkmer);
        }
        let sharedvalue: usize = seqbtreemap[i].1 .1.len() + seqbtreemap[i + 1].1 .1.len();
        newbase.push(ProfileKmer {
            name: seqbtreemap[i].0.clone(),
            nextname: seqbtreemap[i + 1].0.clone(),
            sequence: seqbtreemap[i].1 .1.clone(),
            nextsequence: seqbtreemap[i + 1].1 .1.clone(),
            count: counterbase.len(),
            total: sharedvalue,
            ratio: counterbase.len() as f32 / sharedvalue as f32 * 100.0,
        });
    }
    let mut filewrite = File::create("sequence-clusters.fasta").expect("file not found");
    for i in newbase.iter() {
        writeln!(
            filewrite,
            "{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}",
            i.name, i.nextname, i.sequence, i.nextsequence, i.count, i.total, i.ratio
        )
        .expect("file not found");
    }

    let mut filewithoutkmer = File::create("sequence-cluster.txt").expect("file not found");
    for i in newbase.iter() {
        writeln!(
            filewithoutkmer,
            "{:?}\t{:?}\t{:?}\t{:?}\t{:?}",
            i.name, i.nextname, i.count, i.total, i.ratio
        )
        .expect("file not found");
    }
    Ok(
        "The sequence similarity scores and the cluster of the sequences have been written"
            .to_string(),
    )
}
