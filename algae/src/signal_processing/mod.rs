mod SignalProcessing {
    use crate::array::array::Array;

    pub fn is_linearly_dependent(mut signal_vec:Vec<Vec<f64>>) -> bool {
        fn extend(v:&mut Vec<Vec<f64>>, signal_vec:&mut Vec<Vec<f64>>) -> bool {
            let max_len = signal_vec.iter().map(|v| v.len()).max().unwrap_or(0);
            let mut counter = 0;
            loop {
                let mut temp = Vec::<f64>::with_capacity(signal_vec.len());
                counter = 0;
                for n in 0..signal_vec.len() {
                    if signal_vec[n].len() == max_len {
                        match signal_vec[n].pop() {
                            Some(s) => temp.push(s),
                            None => {
                                temp.push(0.0);
                                counter += 1;
                            }
                        }
                    } else {
                        temp.push(0.0);
                        counter += 1;
                    }
                }
                if !temp.iter().all(|e| *e == 0.0) {
                    v.push(temp);
                    break;
                } else if counter >= signal_vec.len() {
                    return false;
                }
            }
            true
        }
        if signal_vec.len() == 0 {
            true
        } else {
            let mut casorati_mat = Vec::<Vec<f64>>::with_capacity(signal_vec.len());
            while casorati_mat.len() < signal_vec.len() {
                extend(&mut casorati_mat, &mut signal_vec);
            }
            let mut counter = 0;
            while let Err(e) = Array::new_mat(casorati_mat.clone()).inv() {
                casorati_mat.swap_remove(counter % signal_vec.len());
                counter += 1;
                if !extend(&mut casorati_mat, &mut signal_vec) {
                    return false;
                }
            }
            true
        }
    }

    fn convolution_step(signal:&Vec<f64>, filter:&Vec<f64>, k:usize) -> f64 {
        let signal_arr = {
            let start = usize::saturating_sub(k, filter.len());
            let end = usize::min(k, signal.len());
            Array::new_vec((&signal[start..end]).to_vec())
        };
        let filter_arr = {
            let start = usize::saturating_sub(filter.len(), k);
            let end = start + signal_arr.size.1;
            Array::new_vec((&filter[start..end]).to_vec())
        };
        (signal_arr.transpose() * filter_arr)[(0, 0)]
    }

    pub fn convolve(signal:&Vec<f64>, filter:&Vec<f64>) -> Vec<f64> {
        let mut result = Vec::<f64>::with_capacity(signal.len() + filter.len());
        for k in 1..(signal.len() + filter.len()) {
            result.push(convolution_step(&signal, &filter, k));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::signal_processing::SignalProcessing::*;

    #[test]
    fn linear_independency_true() {
        let actual = {
            let signals = vec![
                vec![1.0, 2.0, 1.0],
                vec![0.0, 1.0, 2.0, 1.0],
                vec![0.0, 0.0, 1.0, 2.0, 1.0],
            ];
            is_linearly_dependent(signals)
        };
        assert!(actual);
    }

    #[test]
    fn linear_independency_false() {
        let actual = {
            let signals = vec![
                vec![1.0, 2.0, 1.0],
                vec![0.0, 1.0, 2.0, 1.0],
                vec![0.0, 1.0, 2.0, 1.0, 0.0, 0.0, 0.0],
            ];
            is_linearly_dependent(signals)
        };
        assert!(!actual);
    }

    #[test]
    fn convolution_one() {
        let signal = vec![1.0, 1.0, 1.0];
        let filter = vec![0.25, 0.5, 0.25];
        let expected = vec![0.25, 0.75, 1.0, 0.75, 0.25];
        let actual = convolve(&signal, &filter);

        assert_eq!(expected, actual);
    }

    #[test]
    fn convolution_two() {
        let signal = vec![1.0, 1.0];
        let filter = vec![0.25, 0.5, 0.25];
        let expected = vec![0.25, 0.75, 0.75, 0.25];
        let actual = convolve(&signal, &filter);

        assert_eq!(expected, actual);
    }
}