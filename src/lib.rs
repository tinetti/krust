use clap::{App, Arg, ArgMatches, SubCommand};

fn create_clap_app<'a, 'b>() -> App<'a, 'b> {
    let broker_arg = Arg::with_name("broker")
        .short("b")
        .long("broker")
        .value_name("broker")
        .help("Bootstrap broker(s)")
        .multiple(true)
        .takes_value(true);

    return App::new("krust")
        .version("1.0")
        .about("Krust is a kafka command line client implemented in Rust")
        .arg(&broker_arg)
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("Sets the verbosity level\n(use -vv for even more verbosity)"),
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
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("topic")
                .alias("topics")
                .about("Get information about one or more topics")
                .subcommand(
                    SubCommand::with_name("list")
                        .about("List topics (this is the default subcommand)"),
                ),
        );
}

fn get_command<'a, 'b>(
    app: App<'a, 'b>,
    get_matches: fn(App<'a, 'b>) -> ArgMatches<'a>,
) -> Option<String> {
    let matches = get_matches(app.clone());

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
            let topics = consumer_matches
                .values_of("topic")
                .unwrap()
                .collect::<Vec<_>>();
            println!("TODO: topics: {:?}", topics);

            if let Some(b) = consumer_matches.values_of("broker") {
                let brokers = b.collect::<Vec<_>>();
                println!("TODO: brokers: {:?}", brokers);
            }
        }
        ("topic", Some(topic_matches)) => {
            match topic_matches.subcommand() {
                // list is the default
                (_, _) => {
                    println!("TODO: list topics");
                }
            }
        }
        _ => {
            if let Err(err) = app.clone().print_help() {
                println!("error printing help: {}", err);
            }
        }
    }

    // Continued program logic goes here...

    return None;
}

pub fn run<'a>() {
    let app = create_clap_app();
    let _command = get_command(app.clone(), App::get_matches);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // let arg_vec = vec!["my_prog", "some", "args", "to", "parse"];
        //
        // let mut app = App::new("myprog");
        // // Args and options go here...
        // let matches = app
        //     .get_matches_from_safe_borrow(arg_vec)
        //     .unwrap_or_else(|e| panic!("An error occurs: {}", e));
    }
}
