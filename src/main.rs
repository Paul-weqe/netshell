use std::io::{stdin, stdout, Write};
use clap::{CommandFactory, Error, FromArgMatches, Parser, Subcommand};

#[derive(Parser)]
struct OptCli {
    #[command(subcommand)]
    command: Option<OptCommand>
}

#[derive(Parser)]
struct ConfCli {
    #[command(subcommand)]
    command: Option<ConfCommand>
 }


#[derive(Subcommand)]
enum OptCommand {
    Ping {
        host: String
    }
}

#[derive(Subcommand)]
enum ConfCommand { }

fn netcli_parse<P>(input: &[String]) -> Result<P, Error>  where P: Parser  {
    let mut matches = match <P as CommandFactory>::command().try_get_matches_from_mut(input.clone()){
        Ok(m) => {
            m
        }
        Err(err) => return Err(err)
    };
    let res = match <P as FromArgMatches>::from_arg_matches_mut(&mut matches) {
        Ok(res) => res,
        Err(err) => return Err(err)
    };
    Ok(res)
}

mod ping {
    use pnet::{packet::{icmp::{
        self, destination_unreachable::IcmpCodes, echo_request::MutableEchoRequestPacket, IcmpCode, IcmpType, IcmpTypes, MutableIcmpPacket
    }, Packet}, util::checksum};


    fn send_request() {
        let mut buff = [0u8; 8];
        let mut pkt = MutableEchoRequestPacket::new(&mut buff).unwrap();
        pkt.set_icmp_type(IcmpTypes::EchoRequest);
        // pkt.set_icmp_code(IcmpCodes::);
        pkt.set_identifier(0);
        pkt.set_sequence_number(0);
        pkt.set_checksum(checksum(pkt.packet(), 2));
    }


}


fn main() {
    let mode = CliMode::default();

    'outer: loop {
        match mode {
            
            CliMode::Opr => {
                let input = get_input(">");

                let cli = match netcli_parse::<OptCli>(&input){
                    Ok(cli) => cli,
                    Err(_) => {
                        println!("invalid command: {:?}", input);
                        continue 'outer
                    }
                };

            }
            CliMode::Conf =>  {
                let mut input = get_input("#");

                let cli = match netcli_parse::<ConfCli>(&input){
                    Ok(cli) => cli,
                    Err(_) => {
                        
                        println!("invalid command: {:?}", input.as_slice());
                        continue 'outer
                    }
                };
            }
        }
    }
}


#[derive(Default)]
enum CliMode {
    // operation mode - only allows reading and viewing of certain configs
    #[default]
    Opr,    
    // configuration mode - allows editing of configs
    Conf
}



fn get_input(bash: &str) -> Vec<String> {

    let mut input: String = String::new();

    print!("{bash} ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).expect("Unable to read command");
    let mut args = vec![String::new()];
    input.split(" ").into_iter().for_each(|a| args.push(a.trim().to_string()));
    args
}