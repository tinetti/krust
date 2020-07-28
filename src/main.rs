use clap::{App, Arg, SubCommand};

fn main() {
    let broker_arg = Arg::with_name("broker")
        .short("b")
        .long("broker")
        .value_name("broker")
        .help("Bootstrap broker(s)")
        .multiple(true)
        .takes_value(true);

    let matches = App::new("krust")
        .version("1.0")
        .author("John Tinetti <john@tinetti.net>")
        .about("Kafka command line client")
        .arg(&broker_arg)
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("Sets the verbosity level\n(use -vv for even more verbosity)")
        )
        .subcommand(
            SubCommand::with_name("consumer")
                .about("kafka consumer")
                .arg(&broker_arg)
                .arg(
                    Arg::with_name("topic")
                        .short("t")
                        .long("topic")
                        .value_name("topic")
                        .help("Topic(s) from which to consume")
                        .takes_value(true)
                        .multiple(true)
                        .required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("topic")
                .alias("topics")
                .about("Get information about one or more topics")
                .subcommand(
                    SubCommand::with_name("list")
                        .about("List topics (this is the default subcommand)")
                )
        )
        .get_matches();

    if let Some(brokers) = matches.value_of("brokers") {
        println!("Value for brokers: {}", brokers);
    }

    match matches.occurrences_of("verbose") {
        0 => println!("Verbose mode: off"),
        1 => println!("Verbose mode: kind of on"),
        2 => println!("Verbose mode: on"),
        _ => println!("Don't be crazy"),
    }


    match matches.subcommand() {
        ("consumer", Some(consumer_matches)) => {
            let topics = consumer_matches.values_of("topic").unwrap().collect::<Vec<_>>();
            println!("TODO: topics: {:?}", topics);

            if let Some(b) = consumer_matches.values_of("broker") {
                let brokers = b.collect::<Vec<_>>();
                println!("TODO: brokers: {:?}", brokers);
            }
        },
        ("topic", Some(topic_matches)) => {
            match topic_matches.subcommand() {
                // list is the default
                (_, _) => {
                    println!("TODO: list topics");
                },
            }
        },
        _ => unreachable!()
    }

    // Continued program logic goes here...
}
