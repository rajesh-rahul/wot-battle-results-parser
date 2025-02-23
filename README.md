# World of Tanks Replay Parser

This project attempts to parse almost all of the useful information in `.wotreplay` files from WoT `0.9.12` - `1.26.1` (latest version). The goal is to have a CLI that when given a replay file, output all useful events in JSON format.

Currently I am making major changes to this repo and is therefore a work in progress (more so than before because some versions are broken)

## Technical Overview
A `.wotreplay` file contains two sections:
   - JSON section: contains the information used when showing battle results
   - Binary section: contains the information used when playing the replay with Wot client

The JSON section is easily accessible and quite useful and is used by major sites (such as wotreplays.eu) to generate
the battle results view. 

The binary section is a list of packets (similar to network packets) that describe things happening
in a particular battle. This is pretty much a subset of network traffic from WoT servers during battles. Each packet contains:
- Packet Type
- Timestamp
- Payload

TODO: More info.

One of the big goals of this project is to parse as many replay versions as possible. However, as this 
project is still a work in progress, the kind of information that you can get from each replay differs
based on the replay version. 

For example, the `Position` information of each tank have barely changed over
the years and therefore will probably work with all replays from `0.9.13` to today whereas something like `OnStaticCollision` (a tank colliding with a fence for ex.) has changed quite frequently and require more inspection. While this is not easy to do,
the great thing is that there is a mechanism in place (see [here](https://github.com/rajesh-rahul/wot-battle-results-parser/blob/main/replay_parser/src/packet_parser/events/entity_method/vehicle_methods.rs)) that let's us quickly make the necessary changes to support more versions.



## Example Usage (Old version)
### Replay Parser
Currently this project is only available as a Rust library. Perhaps in the future, we can provide Python bindings. To use it, add the following to the `[dependencies]` section of `Cargo.toml`
```
wot_replay_parser = "0.2.1"
```

#### Example 1: Print out JSON portion of the replay
```rust
use wot_replay_parser::ReplayParser;

pub fn main() {
    let path = "/home/dacite/Projects/wot-battle-results-parser/examples/example.wotreplay";

    // ReplayParser can take a path or Vec<u8> 
    let replay_parser = ReplayParser::parse_file(path).unwrap();

    // replay_json_start return serde_json::Value type
    let replay_json_start = replay_parser.replay_json_start().unwrap();
    let json_string_start = serde_json::to_string_pretty(&replay_json_start).unwrap();


    // This portion is only available for complete replays (i.e the player watched the battle to the end)
    let replay_json_end = replay_parser.replay_json_end().unwrap();
    let json_string_end = serde_json::to_string_pretty(&replay_json_end).unwrap();


    println!("{}", json_string_start);
    println!("{}", json_string_end);

    // There are some other methods readily available as well. See docs.rs page for information
    println!(
        "Replay Version: {:?}",
        replay_parser.parse_replay_version().unwrap()
    );
}
```
#### Example 2: Print out the binary section of the replay in the form of packets
The binary section of the replay can be separated into "packets". Each packet has some metadata information
and then the payload. This is useful if you are developing another projects that needs the packet abstraction.

I used this to create https://rajesh-rahul.github.io/wot-packet-analyzer. This is a GUI for analyzing packets and finding 
out what they mean. Useful as a development tool. 

```rust
use wot_replay_parser::ReplayParser;

pub fn main() {
    let path = "/home/dacite/Projects/wot-battle-results-parser/examples/example.wotreplay";

    // ReplayParser can take a path or Vec<u8> 
    let replay_parser = ReplayParser::parse_file(path).unwrap();

    for packet in replay_parser.packet_stream() {
        let packet = packet.unwrap();

        // This will print out the metadata information of the packet
        println!("{:?}", packet);

        // Adding the plus sign will print out the payload information
        println!("{:+?}", packet);
    }
}
```

#### Example 3: Print out the events of the replay
This is where most of the work is left to do. Events is an abstraction over packets. i.e it shows the actual
data that is present in a packet. Some events such as `Position`,`AvatarCreate` works really well as of today.

See [`BattleEvent`](https://docs.rs/wot_replay_parser/latest/wot_replay_parser/enum.BattleEvent.html) to see what kind 
of events are supported. Note that this doesn't mean it will work in all replays
```rust
use wot_replay_parser::{ReplayParser, BattleEvent};

pub fn main() {
    let path = "/home/dacite/Projects/wot-battle-results-parser/examples/example.wotreplay";

    // ReplayParser can take a path or Vec<u8> 
    let replay_parser = ReplayParser::parse_file(path).unwrap();

    for event in replay_parser.event_stream().unwrap() {
        let event = event.unwrap();

        // This will print out the event if is a chat event (Ofcourse, you can print out all event types if needed)
        if let BattleEvent::Chat(chat_event) = event {
            // Print out the event
            println!("{:?}", chat_event);

            // You can also convert these events to any format supported by serde . Here is an example
            // where it is converted to JSON
            println!("{}", serde_json::to_string_pretty(&chat_event).unwrap());
        }


    }
}

```

See the docs.rs link for more thorough documentation: https://docs.rs/wot_replay_parser/latest/wot_replay_parser
## Projects that use this library:
- Wot Packet Analyzer: https://rajesh-rahul.github.io/wot-packet-analyzer
   - An analyzer for packets in a `.wotreplay` files. Useful for development
   - Uses `wot_replay_parser` library
   
## `.dat` file parsing
This project used to parse `.dat` files generated by the game but that development work is archived.

## Credits
 - https://github.com/Monstrofil/replays_unpack
    - Everything on parsing `.def` files and using them comes from here
    - Code generation scripts rely on this library
    - Knowledge on how to parse many packets types (and data structures) also comes from here
 - https://github.com/izeberg/wot-src
    - `.def` file source code for newer wot versions comes from here
 - https://github.com/StranikS-Scan/WorldOfTanks-Decompiled
    - `.def` file source code for old wot versions comes from here
 - Contributors to [vbAddict Wiki](https://web.archive.org/web/20180407110623/http://wiki.vbaddict.net/pages/WoT_Developer_Wiki)
 - http://forum.worldoftanks.eu/index.php?/topic/185348-11011-wot-replay-analyzer-wip-1-08112020/
 - https://github.com/evido/wotreplay-parser
 - https://github.com/Phalynx/WoT-Replay-To-JSON
 - https://github.com/lkolbly/wows-replays
