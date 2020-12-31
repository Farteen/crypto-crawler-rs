use crate::WSClient;

use super::ws_client_internal::WSClientInternal;

pub(super) const EXCHANGE_NAME: &str = "MXC";
pub(super) const SOCKETIO_PREFIX: &str = "42";

const SPOT_WEBSOCKET_URL: &str = "wss://wbs.mxc.com/socket.io/?EIO=3&transport=websocket";
const SWAP_WEBSOCKET_URL: &str = "wss://contract.mxc.com/ws";

/// The WebSocket client for MXC Spot market.
///
/// Official doc: <https://github.com/mxcdevelop/APIDoc/blob/master/websocket/websocket-api.md>.
///
/// ## Channel format
pub struct MXCSpotWSClient<'a> {
    client: WSClientInternal<'a>,
}

/// The WebSocket client for MXC Swap market(<https://github.com/mxcdevelop/APIDoc/blob/master/contract/contract-api.md>).
pub struct MXCSwapWSClient<'a> {
    client: WSClientInternal<'a>,
}

// Example: symbol:BTC_USDT -> 42["sub.symbol",{"symbol":"BTC_USDT"}]
fn spot_channel_to_command(ch: &str, subscribe: bool) -> String {
    let v: Vec<&str> = ch.split(':').collect();
    let channel = v[0];
    let pair = v[1];

    let mut command = String::new();
    command.push_str(
        format!(
            r#"{}["{}.{}","#,
            SOCKETIO_PREFIX,
            if subscribe { "sub" } else { "unsub" },
            channel
        )
        .as_str(),
    );
    command.push_str(format!(r#"{{"symbol":"{}"}}]"#, pair).as_str());

    command
}

fn spot_serialize_command(channels: &[String], subscribe: bool) -> Vec<String> {
    let mut commands = Vec::<String>::new();

    for s in channels.iter() {
        let command = spot_channel_to_command(s, subscribe);
        commands.push(command);
    }

    commands
}

// Example: deal:BTC_USDT -> {"method":"sub.deal","param":{"symbol":"BTC_USDT"}}
fn swap_channel_to_command(ch: &str, subscribe: bool) -> String {
    let v: Vec<&str> = ch.split(':').collect();
    let channel = v[0];
    let pair = v[1];

    let mut command = String::new();
    command.push_str(
        format!(
            r#"{{"method":"{}.{}","#,
            if subscribe { "sub" } else { "unsub" },
            channel
        )
        .as_str(),
    );
    command.push_str(format!(r#""param":{{"symbol":"{}"}}}}"#, pair).as_str());

    command
}

fn swap_serialize_command(channels: &[String], subscribe: bool) -> Vec<String> {
    let mut commands = Vec::<String>::new();

    for s in channels.iter() {
        let command = swap_channel_to_command(s, subscribe);
        commands.push(command);
    }

    commands
}

define_client!(
    MXCSpotWSClient,
    EXCHANGE_NAME,
    SPOT_WEBSOCKET_URL,
    spot_serialize_command
);
define_client!(
    MXCSwapWSClient,
    EXCHANGE_NAME,
    SWAP_WEBSOCKET_URL,
    swap_serialize_command
);

#[cfg(test)]
mod tests {
    #[test]
    fn test_spot_channel_to_command() {
        let channel = "symbol:BTC_USDT";

        let subscribe_command = super::spot_channel_to_command(channel, true);
        assert_eq!(
            r#"42["sub.symbol",{"symbol":"BTC_USDT"}]"#.to_string(),
            subscribe_command
        );

        let unsubscribe_command = super::spot_channel_to_command(channel, false);
        assert_eq!(
            r#"42["unsub.symbol",{"symbol":"BTC_USDT"}]"#.to_string(),
            unsubscribe_command
        );
    }

    #[test]
    fn test_swap_channel_to_command() {
        let channel = "deal:BTC_USDT";

        let subscribe_command = super::swap_channel_to_command(channel, true);
        assert_eq!(
            r#"{"method":"sub.deal","param":{"symbol":"BTC_USDT"}}"#.to_string(),
            subscribe_command
        );

        let unsubscribe_command = super::swap_channel_to_command(channel, false);
        assert_eq!(
            r#"{"method":"unsub.deal","param":{"symbol":"BTC_USDT"}}"#.to_string(),
            unsubscribe_command
        );
    }
}