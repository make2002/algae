mod ml {
    use std::{error, process::Output};

    use crate::array::array::Array;
    
    #[derive(Clone, Copy)]
    pub enum ActivationFunction{
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

        fn perceptron_derivative(value:f64) -> f64 {
            0.
        }

        fn sigmoid(value:f64) -> f64 {
            1.0 / 1.0 + (-value).exp()
        }

        fn sigmoid_derivative(value:f64) -> f64 {
            (-value).exp() / ((-2. * value).exp() + 2. * (-value).exp() + 1.)
        }

        fn tanh(value:f64) -> f64 {
            value.tanh()
        }

        fn tanh_derivative(value:f64) -> f64 {
            1. - value.tanh().powf(2.)
        }

        fn relu(value:f64) -> f64 {
            value.max(0.)
        }

        fn relu_derivative(value:f64) -> f64 {
            if value > 0. {
                1.
            } else {
                0.
            }
        }

        fn leaky_relu(value:f64) -> f64 {
            if value < 0. {
                0.1 * value
            } else {
                value
            }
        }

        fn leaky_relu_derivative(value:f64) -> f64 {
            if value < 0.0 {
                0.1
            } else {
                1.
            }
        }

        fn elu(value:f64) -> f64 {
            if value < 0.0 {
                value.exp() - 1.0
            } else {
                value
            }
        }

        fn elu_derivative(value:f64) -> f64 {
            if value < 0.0 {
                value.exp()
            } else {
                1.
            }
        }

        fn softmax(value:f64, divisor:f64) -> f64 {
            value.exp() / divisor
        }

        fn softmax_derivative(value:f64, divisor:f64) -> f64 {
            value.exp() / divisor
        }

        fn softplus(value:f64) -> f64 {
            (1.0 + value.exp()).ln()
        }

        fn softplus_derivative(value:f64) -> f64 {
            value.exp() / (value.exp() + 1.)
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
    
    #[macro_export]
    macro_rules! derivative {
        ( $function:expr, $arr:expr ) => {
            match $function {
                ActivationFunction::Perceptron => {
                    for row in 0..$arr.size.1 {
                        for col in 0..$arr.size.0 {
                            $arr[(row, col)] = ActivationFunction::perceptron_derivative($arr[(row, col)]);
                        }
                    }
                    $arr
                },
                ActivationFunction::Sigmoid => {
                    for row in 0..$arr.size.1 {
                        for col in 0..$arr.size.0 {
                            $arr[(row, col)] = ActivationFunction::sigmoid_derivative($arr[(row, col)]);
                        }
                    }       
                    $arr         
                },
                ActivationFunction::Tanh => {
                    for row in 0..$arr.size.1 {
                        for col in 0..$arr.size.0 {
                            $arr[(row, col)] = ActivationFunction::tanh_derivative($arr[(row, col)]);
                        }
                    }    
                    $arr              
                },
                ActivationFunction::ReLU => {
                    for row in 0..$arr.size.1 {
                        for col in 0..$arr.size.0 {
                            $arr[(row, col)] = ActivationFunction::relu_derivative($arr[(row, col)]);
                        }
                    }   
                    $arr               
                },
                ActivationFunction::LeakyReLu => {
                    for row in 0..$arr.size.1 {
                        for col in 0..$arr.size.0 {
                            $arr[(row, col)] = ActivationFunction::leaky_relu_derivative($arr[(row, col)]);
                        }
                    }      
                    $arr            
                },
                ActivationFunction::Elu => {
                    for row in 0..$arr.size.1 {
                        for col in 0..$arr.size.0 {
                            $arr[(row, col)] = ActivationFunction::elu_derivative($arr[(row, col)]);
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
                            $arr[(row, col)] = ActivationFunction::softmax_derivative($arr[(row, col)], denominator);
                        }
                    }
                    $arr
                },
                ActivationFunction::Softplus => {
                    for row in 0..$arr.size.1 {
                        for col in 0..$arr.size.0 {
                            $arr[(row, col)] = ActivationFunction::softplus_derivative($arr[(row, col)]);
                        }
                    }    
                    $arr              
                },
            }
        };
    }

    pub struct NeuralNetwork {
        input:Array<f64>,
        output:Array<f64>,
        weights:Vec<Array<f64>>,
        biases:Vec<Array<f64>>,
        activation_function:ActivationFunction,
    }

    impl NeuralNetwork {
       pub fn new(
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

        pub fn get_input_size(&self) -> usize {
            self.input.size.1
        }

        pub fn set_input(&mut self, input:Vec<f64>) {
            if input.len() != self.input.size.1 {
                panic!("Error: Wrong input size, expected '{}', actually '{}'.", self.input.size.1, input.len());
            }
            self.input = Array::new_vec(input);
        }

        pub fn get_output(&self) -> Vec<f64> {
            let mut output = Vec::<f64>::with_capacity(self.output.size.1);
            for row in 0..self.output.size.1 {
                output.push(self.output[(row, 0)]);
            }
            output
        }

        pub fn propagate_forward(&mut self) {
            let mut temp = self.input.clone();
            for i in 0..self.weights.len() {
                temp = apply!(
                    self.activation_function,
                    self.weights[i].clone() * temp.clone() + self.biases[i].clone()
                );
            }
            self.output = temp;
        }

        pub fn propagate_backwards(&mut self, target:Vec<f64>, learning_rate:f64) -> Vec<f64> {
            let mut error = self.output.clone();
            for i in 0..error.size.1 {
                error[(0, i)] = self.output[(0, i)] - target[i];
            }

            for i in (0..self.weights.len()).rev() {
                let delta_weights = error.hadamard_product(derivative!(self.activation_function, self.output.clone()));
                let weights_gradient = delta_weights.clone().hadamard_product(self.weights[i].transpose());

                self.weights[i] = self.weights[i].clone() - weights_gradient * learning_rate;

                let biases_gradient = delta_weights.clone();

                self.biases[i] = self.biases[i].clone() - biases_gradient * learning_rate;

                error = delta_weights.hadamard_product(self.weights[i].clone());
            }
            let mut error_vec = Vec::<f64>::with_capacity(error.size.1);
            for n in 0..error.size.1 {
                error_vec.push(error[(n, 0)]);
            }
            error_vec
        }
    }

    pub struct RecurrentNeuralNetwork {
        neural_network:NeuralNetwork,
        hidden_layer_size:usize,
    } 

    impl RecurrentNeuralNetwork {
        pub fn new(
            input_size:usize, 
            output_size:usize, 
            hidden_layer_size:usize,
            depth:usize, 
            layers:usize, 
            activation_function:ActivationFunction
        ) -> Self {
            let neural_network = NeuralNetwork::new(
                input_size + hidden_layer_size, 
                output_size + hidden_layer_size, 
                depth, 
                layers, 
                activation_function,
            );
            RecurrentNeuralNetwork {
                neural_network,
                hidden_layer_size
            }
        }

        pub fn get_input_size(&self) -> usize {
            self.neural_network.get_input_size() - self.hidden_layer_size
        }

        pub fn set_input(&mut self, mut input:Vec<f64>) {
            input.append(&mut self.neural_network.get_output());
            self.neural_network.set_input(input)
        }

        pub fn get_hidden_layer_size(&self) -> usize {
            self.hidden_layer_size
        }

        pub fn get_hidden_layer(&self) -> Vec<f64> {
            let mut hidden_layer = self.neural_network.get_output();
            hidden_layer.split_off(hidden_layer.len() - self.hidden_layer_size)
        }

        pub fn manual_hidden_layer(&mut self, hidden_layer:Vec<f64>) {
            for n in (0..self.hidden_layer_size).rev() {
                let nn_index = self.neural_network.input.size.1 - n;
                let hidden_index = self.hidden_layer_size - n;
                self.neural_network.input[(nn_index, 0)] = hidden_layer[hidden_index];
            }
        }

        pub fn get_output(&self) -> Vec<f64> {
            let mut output = self.neural_network.get_output();
            output.truncate(output.len() - self.hidden_layer_size);
            output
        }

        pub fn propagate_forward(&mut self) {
            self.neural_network.propagate_forward()
        }

        pub fn propagate_backwards(&mut self, mut target:Vec<f64>, learning_rate:f64) -> Vec<f64> {
            while target.len() < self.neural_network.output.size.1 + self.hidden_layer_size {
                target.push(self.neural_network.output[(target.len(), 0)]);
            }
            self.propagate_backwards(target, learning_rate)
        }

        pub fn propagate_only_hidden(&mut self, mut target:Vec<f64>, learning_rate:f64) -> Vec<f64> {
            let target = {
                let mut output = self.neural_network.get_output();
                output.append(&mut target);
                output
            };
            self.propagate_backwards(target, learning_rate)
        }

        // Note that this implementation allows for the target to be larger than the output 
        //      thus also learning the hidden layer. 
        //      This isn't a bug, it's a feature ;)
        pub fn backward_pass(&mut self, target:Vec<f64>, learning_rate:f64, mut history:Vec<Vec<f64>>) {
            self.propagate_backwards(target, learning_rate);
            if let Some((target_out)) = history.pop() {
                self.backward_pass(target_out, learning_rate, history);
            }
        }        
    }
}

mod generall_intelligence;
