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
}

#[cfg(test)]
mod tests {
    use crate::signal_processing::SignalProcessing::is_linearly_dependent;

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
}