// old code, look into the ./src/* for new stuff

mod bm25;
use bm25::bum::BM25;

fn main() {

    let corpus = vec![
        "Hello world. george How are you's today? World. world world",
        "I have a friend named george and you should say hello!"
    ];

    let bm25 = BM25::new(&corpus);

    let queries = vec![
        "george super",
        "Hello World"
    ];
    
    let res = bm25.run(&queries);
    
    for (c, r) in res.iter().enumerate() {
        for (a, b) in r.iter() {
            println!("search term: {}, docid: {}, score: {}", &queries[c], &a, &b);
        }
    }





    //let (idx, dlt) = ds::build_data_structures(corpus);
    //let doc = "The “brown” fox can't jump 32.3 feet, right?";
    //let tokenizer = RegexpTokenizer::default();
    //let doc: Vec<String> = tokenizer.tokenize(doc)
    //    .map(|word| word.to_string())
    //    .collect();

    //bm25.idx.print();

    //bm25.dlt.print();

    //println!("Total words in IF: {}", inv_file.total_words());
}
