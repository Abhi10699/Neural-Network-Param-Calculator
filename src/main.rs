use std::io::{self};

struct NeuralNetConfig {
    in_features: i128,
    hidden_neurons: Vec<i128>,
    output_size: i128,
}

impl NeuralNetConfig {
    fn new() -> Self {
        println!("Enter Number of Input Features: ");
        let mut in_features = String::new();
        io::stdin()
            .read_line(&mut in_features)
            .expect("Error reading input Features");

        println!("Enter Number of Hidden Layers: ");
        let mut hidden_layers = String::new();
        io::stdin()
            .read_line(&mut hidden_layers)
            .expect("Error reading hidden layers");

        // this will be a loop
        println!("Enter Number of Hidden Neurons in Each Layer (comma separated): ");
        let mut hidden_size = String::new();
        io::stdin()
            .read_line(&mut hidden_size)
            .expect("Error reading hidden neuron size");

        println!("Enter Number of Output in Each Layer: ");
        let mut out_features = String::new();
        io::stdin()
            .read_line(&mut out_features)
            .expect("Error reading output neurons");

        let hidden_size_vec = hidden_size.split(",").collect::<Vec<&str>>();

        if hidden_size_vec.len() != hidden_layers.trim().parse().unwrap() {
            panic!("Hidden Neurons length does not match hidden layer size..");
        }

        Self {
            in_features: in_features
                .trim()
                .parse()
                .expect("Error parsing in_features"),
            output_size: out_features
                .trim()
                .parse()
                .expect("Error parsing output features"),
            hidden_neurons: hidden_size_vec
                .into_iter()
                .map(|x| return x.trim().parse::<i128>().expect("Error parsing neurons"))
                .collect::<Vec<i128>>(),
        }
    }

    fn calculate_params(&self) -> i128 {
        let mut learnable_params: i128;
        let mut hidden_neurons_iter = self.hidden_neurons.iter();

        // calculate for the first layer
        let first_layer_neurons = hidden_neurons_iter.next().expect("element does not exist");
        println!("First layer neuron: {}", first_layer_neurons);
        learnable_params = (self.in_features * first_layer_neurons) + first_layer_neurons;

        let mut last_layer_output_size = first_layer_neurons;
        // calculate hidden layers
        for neurons in hidden_neurons_iter {
            learnable_params += (last_layer_output_size * neurons) + neurons;
            last_layer_output_size = neurons;
        }

        // calculate for output node
        println!("{:#?}", last_layer_output_size);
        learnable_params += (last_layer_output_size * self.output_size) + self.output_size;
        learnable_params
    }
}

fn main() {
    let net = NeuralNetConfig::new();
    let net_params = net.calculate_params();
    println!("Neural Netowrk Parameters: {}", net_params);
}
