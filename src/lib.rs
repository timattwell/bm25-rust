/* libbm25 (0.1.0)
 * Author - Tim Attwell 
 * 
 * Desc -
 * An implementation of the BM25 algorithm in rust, exported as a python library.
 * It was created to speed up computation on the DMINR project in assessing 
 * retrieved term relevance to an intial search query. All the conversion from 
 * python to rust types and vice versa are done as soon as the data enters and
 * leaves the function respectively, and so there should be maximum performance
 * gains as all computation is done in native rust.
 * 
 * The program is structured based on the influence of several BM25 libraries,
 * mostly originally written in Python. I wanted a fast, variable-based input
 * function, rather than a file based input.
 * 
 * ToDo -
 * Improve the configurability of the BM25 weights, and investigate the
 * performance hit from making these variables unknown at compile time.
 * 
 * Lots of testing, particularly with larger corpus sets
 * 
 * Parallelise some of the search and processing functions and investigate performance 
 * gains. While the data structures use the highly optimised hashbrown::HashMap, that I 
 * believe already implements rayon parallisations, the results are returned in a std::ops::HM,
 * to enable compatibility with the python wrapper.
 * 
 */

// Include the data structure and scoring modules from the local directory.
mod ds;
mod score;

// Import the two fast HashMap based data structs
use ds::{ InvertedIndex , DocumentLengthTable };
// Use std::...::HashMap for results to keep compatible with pyO3
use std::collections::HashMap;
use vtext::tokenize::*;
use rayon::prelude::*;
use std::time::Instant;

// Import python wrapping
use pyo3::prelude::*;
use pyo3::types::*;

// Create the python module. In this case only a single "class" is made available to
// avoid as much data conversion overhead as possible.
#[pymodule]
fn bm25_roost(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<BM25>()?;
    Ok(())
}

/* The BM25 struct that is to be wrapped as a python class. The data in the struct consists
 * of the high speed inverted index and document length table, along with a Regexp tokenizer. 
 * Having the tokenizer within the struct allows it to be reused in any function without 
 * needing to re-initialise it.
 */
#[pyclass]
struct BM25{
    idx: InvertedIndex,
    dlt: DocumentLengthTable,
    tok: RegexpTokenizer,
    k1: f64,
    b: f64,
    eps: f64,
    delt: f64,
}

// Python facing methods in BM25 struct 
#[pymethods]
impl BM25 {
    /* BM25::new() function becomes python class __init__. I found implementations where the
     * constructor takes and processes the corpus much more elegant, and so that is what I have
     * implemented here. This will also eventually include runtime tweakable weights for the 
     * ranking algorithm.
     */
    #[new]
    fn new(corpus: &PyList, k1: &PyFloat, b: &PyFloat, eps: &PyFloat, delt: &PyFloat) -> PyResult<BM25> {
        let k1: f64 = k1.extract()?;
        let b: f64 = b.extract()?;
        let eps: f64 = eps.extract()?;
        let delt: f64 = delt.extract()?;
        // Extracts rust Vec<&str> from the input python list
        let corpus: Vec<&str> = corpus.extract()?; 
        // Making everything lowercase also converts &str -> String (might not be necessary)
        
        let corpus: Vec<String> = corpus.par_iter()
            .map(|s| s.to_lowercase())
            .collect();
        // Need to convert back String -> &str for tokenizer
        let corpus: Vec<&str> = corpus.par_iter()
            .map(|s| &**s)
            .collect();
        
        // Initialise tokenizer and the output vec of vecs
        let tokenizer = RegexpTokenizer::default();
        // Loop through docs in corpus and tokenize each doc, &str -> Vec<&str>
        let tokenized_corpus = corpus.iter()
            .map(|doc| tokenizer.tokenize(doc).collect())
            .collect();

        // Initialise the data structs, filling them with the tokenized corpus
        let (idx, dlt) = ds::build_data_structures(tokenized_corpus);
        
        // Return struct within PyResult<_> in case of failed input data conversion.
        Ok(
            BM25 {
                idx: idx,
                dlt: dlt,
                tok: tokenizer,
                k1: k1,
                b: b,
                eps: eps,
                delt: delt,
            }
        )
    }
    
    /* BM25::query() method is essentially the python facing wrapper for the 
     * BM25::eval() method with the additional functionality that it can take in
     * several query terms in a vec, and then par_iterate through and return their 
     * rankings as a rust vec of hashmaps -> python list of dictionaries.
     */
    fn query(&self, query_set: &PyList) -> PyResult<Vec<HashMap<u32, f64>>> {
        let query_set: Vec<&str> = query_set.extract()?;
        Ok(
            query_set.iter()
                .map(|query| self.eval(query))
                .collect()
        )
    }

    /* bm25::av_query() is exactly the same as the above, except the 
     * results scores returned for each query are averaged out to give
     * a single value.
     */
    fn av_query(&self, query_set: &PyList) -> PyResult<Vec<f64>> {
        let query_set: Vec<&str> = query_set.extract()?;
        Ok(
            query_set.iter()
                .map(|query| {
                    let x = self.eval(query).values().sum::<f64>() /
                           (self.eval(query).values().len() as f64);
                    match x.is_nan() {
                        true => 0.0,
                        _  => x,
                    } 
                })
                .collect()
        )
    }

    // Rudimentary printing of the data structures to get an idea of whats
    // inside them
    fn print(&self) {
        self.idx.print();
        self.dlt.print();
    }
}

// Internal rust methods for BM25 struct
impl BM25 {
    /* BM25::eval() takes a query string, tokenising it if necessary, and then 
     * scores them based on the BM25+ function
     *
     *
     * 
     */
    fn eval(&self, query: &str) -> HashMap<u32, f64> {
        let mut query_result: HashMap<u32, f64> = HashMap::new();
        let tok_query: Vec<&str> = self.tok.tokenize(query).collect();
        let avdl = self.dlt.get_mean_len();
        let big_n = self.dlt.table.len() as f64;
        for term in tok_query {
            if let Some(doc_hash) = self.idx.index.get(term) {
                let num_docs = doc_hash.len() as f64;
                for (docid, freq) in doc_hash.iter() {
                    let score = score::bm25(
                        num_docs,                          // n = number of documents containing q_i
                        *freq as f64,                       // f = term frequency in document
                        big_n,        // N = total number of documents in collection
                        self.dlt.get_length(docid).unwrap() as f64, // dl = length of document D in words
                        avdl,     // avdl = average document length
                        self.k1,
                        self.b,
                        self.eps,
                        self.delt
                    );
                    if let Some(result) = query_result.get_mut(docid) {
                        *result += score;
                    } else {
                        query_result.insert(*docid, score);
                    }
                }
            }
        }
        query_result
    }
}