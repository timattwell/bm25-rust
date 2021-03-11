pub fn bm25(n_q: f64, freq: f64, big_n: f64, dl: f64, avdl: f64, 
            k1: f64, b: f64, eps: f64, delt: f64) -> f64 {

    let idf = ( (big_n - n_q + 0.5) / (n_q + 0.5) + 1.0).ln().max(eps);
    
    let k = k1 * ( 1.0 - b + (b * (dl / avdl)) );

    idf * ( delt + ( freq * (k1 + 1.0) ) / 
                    ( freq + (k1 * k) ))
}
    
    