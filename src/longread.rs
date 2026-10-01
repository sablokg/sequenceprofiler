use crate::filestruct::Genomeiter;
use crate::filestruct::VecStoreAnalyze;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
Gaurav Sablok,
gsablok@proton.me
*/

pub fn kmer_fasta(fastqfile: String, kmer: usize) -> Result<String, Box<dyn Error>> {
    let f = File::open(fastqfile).expect("file not present");
    let read = BufReader::new(f);
    let mut header: Vec<String> = vec![];
    let mut sequence: Vec<String> = vec![];
    for i in read.lines() {
        let line = i.expect("line not present");
        if line.starts_with(">") {
            header.push(line.clone().replace(">", ""))
        }
        if !line.starts_with(">") {
            sequence.push(line.clone())
        }
    }

    let mut genomeiter: Vec<Genomeiter> = Vec::new();
    for i in 0..header.len() {
        genomeiter.push(Genomeiter {
            header: header[i].clone(),
            sequence: sequence[i].clone(),
        })
    }

    let mut sequence_iter: Vec<String> = vec![];
    for i in 0..sequence.len() {
        let i = sequence[i].clone();
        for j in 0..i.len() - kmer {
            sequence_iter.push(i[j..j + kmer].to_string())
        }
    }

    let finalvec = sequence_iter
        .iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();

    let mut path = File::create("kmeruniquefasta.txt").expect("file not present");
    for i in finalvec.iter() {
        writeln!(path, "{}", i).expect("file not found");
    }

    // added a None value check so that it dont reverse iterate

    let mut indexstorestart = Vec::new();
    for i in genomeiter.iter() {
        for j in finalvec.iter() {
            let indexout = i.sequence.find(&j.to_string());
            let indexoutend = i.sequence.find(&j.to_string());
            if indexout == None || indexoutend == None {
                continue;
            } else if indexout == None || indexoutend != None {
                indexstorestart.push(VecStoreAnalyze {
                    seqid: i.header.clone(),
                    id: i.sequence.to_string(),
                    kmer: j.to_string(),
                    numberstart: indexout.unwrap(),
                    numberend: indexoutend.unwrap() + j.len(),
                });
            }
        }
    }

    for line in indexstorestart.iter() {
        println!(
            "{}\t{}\t{}\t{}\t{}",
            line.seqid, line.id, line.kmer, line.numberstart, line.numberend
        )
    }

    let mut filewrite = File::create("sequenceorigin.txt").expect("file not found");
    for i in indexstorestart.iter() {
        writeln!(
            filewrite,
            "{}\t{}\t{}\t{}\t{}",
            i.seqid, i.id, i.kmer, i.numberstart, i.numberend
        )
        .expect("line not present");
    }

    Ok("The kmer file has been written".to_string())
}
