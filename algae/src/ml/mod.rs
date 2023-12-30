mod ml {
    use crate::array::array::Array;
    
    enum ActivationFunction{
        Perceptron,
        Sigmoid,
        Tanh,
        ReLU,
        LeakyReLu,
        Elu,
        Softmax,
        Softplus,
    }

    impl ActivationFunction {
        fn perceptron(value:f64) -> f64 {
            f64::signum(value)
        }

        fn sigmoid(value:f64) -> f64 {
            1.0 / 1.0 + (-value).exp()
        }

        fn tanh(value:f64) -> f64 {
            value.tanh()
        }

        fn relu(value:f64) -> f64 {
            value.max(0.0)
        }

        fn leaky_relu(value:f64) -> f64 {
            if value < 0.0 {
                0.1 * value
            } else {
                value
            }
        }

        fn elu(value:f64) -> f64 {
            if value < 0.0 {
                value.exp() - 1.0
            } else {
                value
            }
        }

        fn softmax(value:f64, divisor:f64) -> f64 {
            value.exp() / divisor
        }

        fn softplus(value:f64) -> f64 {
            (1.0 + value.exp()).ln()
        }
    }

    #[macro_export]
    macro_rules! apply {
        ( $function:expr, $arr:expr ) => {
            match $function {
                ActivationFunction::Perceptron => {
                    for row in 0..$arr.size.1 {
                        for col in 0..$arr.size.0 {
                            $arr[(row, col)] = ActivationFunction::perceptron($arr[(row, col)]);
                        }
                    }
                    $arr
                },
                ActivationFunction::Sigmoid => {
                    for row in 0..$arr.size.1 {
                        for col in 0..$arr.size.0 {
                            $arr[(row, col)] = ActivationFunction::sigmoid($arr[(row, col)]);
                        }
                    }       
                    $arr         
                },
                ActivationFunction::Tanh => {
                    for row in 0..$arr.size.1 {
                        for col in 0..$arr.size.0 {
                            $arr[(row, col)] = ActivationFunction::tanh($arr[(row, col)]);
                        }
                    }    
                    $arr              
                },
                ActivationFunction::ReLU => {
                    for row in 0..$arr.size.1 {
                        for col in 0..$arr.size.0 {
                            $arr[(row, col)] = ActivationFunction::relu($arr[(row, col)]);
                        }
                    }   
                    $arr               
                },
                ActivationFunction::LeakyReLu => {
                    for row in 0..$arr.size.1 {
                        for col in 0..$arr.size.0 {
                            $arr[(row, col)] = ActivationFunction::leaky_relu($arr[(row, col)]);
                        }
                    }      
                    $arr            
                },
                ActivationFunction::Elu => {
                    for row in 0..$arr.size.1 {
                        for col in 0..$arr.size.0 {
                            $arr[(row, col)] = ActivationFunction::elu($arr[(row, col)]);
                        }
                    }
                    $arr
                },
                ActivationFunction::Softmax => {
                    let mut denominator = 0.0;
                    for row in 0..$arr.size.1 {
                        for col in 0..$arr.size.0 {
                            denominator = $arr[(row, col)].exp();
                        }
                    }
                    for row in 0..$arr.size.1 {
                        for col in 0..$arr.size.0 {
                            $arr[(row, col)] = ActivationFunction::softmax($arr[(row, col)], denominator);
                        }
                    }
                    $arr
                },
                ActivationFunction::Softplus => {
                    for row in 0..$arr.size.1 {
                        for col in 0..$arr.size.0 {
                            $arr[(row, col)] = ActivationFunction::softplus($arr[(row, col)]);
                        }
                    }    
                    $arr              
                },
            }
        };
    }

    struct NeuralNetwork {
        input:Array<f64>,
        output:Array<f64>,
        weights:Vec<Array<f64>>,
        biases:Vec<Array<f64>>,
        activation_function:ActivationFunction,
    }

    impl NeuralNetwork {
       fn new(
           input_size:usize, 
           output_size:usize, 
           depth:usize, 
           layers:usize, 
           activation_function:ActivationFunction
       ) -> Self {
            let input = Array::new_filled((1, input_size), 1.0);
            let output = Array::new_filled((1, output_size), 1.0);
            let mut weights = Vec::<Array<f64>>::with_capacity(layers + 2);
            weights.push(
                Array::new_filled((input_size, depth), 1.0)
            );
            while weights.len() < layers + 1 {
                weights.push(
                    Array::new_filled((depth, depth), 1.0)
                );
            }
            weights.push(
                Array::new_filled((depth, output_size), 1.0)
            );
            let mut biases = Vec::<Array<f64>>::with_capacity(layers + 2);
            while biases.len() < layers {
                biases.push(
                    Array::new_filled((1, depth), 1.0)
                );
            }
            biases.push(
                Array::new_filled((1, output_size), 1.0)
            );
            NeuralNetwork {
                input,
                output,
                weights,
                biases,
                activation_function,
            }
        }

        fn get_input_size(&self) -> usize {
            self.input.size.1
        }

        fn set_input(&mut self, input:Vec<f64>) {
            if input.len() != self.input.size.1 {
                panic!("Error: Wrong input size, expected '{}', actually '{}'.", self.input.size.1, input.len());
            }
            self.input = Array::new_vec(input);
        }

        fn get_output(&self) -> Vec<f64> {
            let mut output = Vec::<f64>::with_capacity(self.output.size.1);
            for row in 0..self.output.size.1 {
                output.push(self.output[(row, 0)]);
            }
            output
        }

        fn propagate(&mut self) {
            let mut temp = self.input.clone();
            for i in 0..self.weights.len() {
                temp = apply!(
                    self.activation_function,
                    self.weights[i].clone() * temp.clone() + self.biases[i].clone()
                );
            }
            self.output = temp;
        }
    }
}