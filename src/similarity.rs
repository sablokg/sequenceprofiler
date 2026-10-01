use crate::filestruct::CollectIter;
use crate::filestruct::Genomeiter;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
Gaurav Sablok,
codeprog@icloud.com
*/

pub fn profilesimilarity(path: &str, kmer: &str) -> Result<String, Box<dyn Error>> {
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
            .collect::<Vec<_>>();
        seqbtreemap.push((i.header.clone(), (i.sequence.clone(), sequencehash)));
    }

    let mut newbase: Vec<Vec<CollectIter>> = Vec::new();
    for i in 0..seqbtreemap.len() - 1 {
        let mut shared: Vec<CollectIter> = Vec::new();
        let vec = seqbtreemap[i].clone();
        let restvec: Vec<_> = seqbtreemap[i + 1usize..seqbtreemap.len()]
            .iter()
            .collect::<Vec<_>>();
        for restvectiter in 0..restvec.len() {
            let mut countkmer: usize = 0usize;
            for itercount in 0..vec.1 .1.len() {
                if restvec[restvectiter]
                    .1
                     .1
                    .contains(&vec.1 .1[itercount].to_string())
                {
                    countkmer += 1usize;
                }
            }
            shared.push(CollectIter {
                name: vec.0.clone(),
                namenext: restvec[restvectiter].0.clone(),
                id: vec.1 .0.clone(),
                idnext: restvec[restvectiter].1 .0.clone(),
                count: countkmer,
                shared: vec.1 .1.len() + restvec[restvectiter].1 .1.len(),
            });
        }
        newbase.push(shared);
    }

    let mut filewrite = File::create("sequence-clusters.fasta").expect("file not found");
    for i in newbase.iter() {
        for j in i.iter() {
            writeln!(
                filewrite,
                "{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}",
                j.name,
                j.namenext,
                j.id,
                j.idnext,
                j.count,
                j.shared,
                j.count as f32 / j.shared as f32 * 100.0
            )
            .expect("file not found");
        }
    }

    let mut filesecondwrite = File::create("frequencies-tab.txt").expect("file not present");
    for i in newbase.iter() {
        for j in i.iter() {
            writeln!(
                filesecondwrite,
                "{:?}\t{:?}\t{:?}\t{:?}\t{:?}",
                j.name,
                j.namenext,
                j.count,
                j.shared,
                j.count as f32 / j.shared as f32 * 100.0
            )
            .expect("file not found");
        }
    }
    Ok(
        "The sequence similarity scores and the cluster of the sequences have been written"
            .to_string(),
    )
}
