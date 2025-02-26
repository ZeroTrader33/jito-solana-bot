fn main() {
    std::panic::set_hook(Box::new(|info| {
        println!("Caught a panic: {:?}", info);
    }));
    // udp_proxy::UdpProxyClient::run();

    let bundle_sender = udp_proxy::BundleSender::create();
    bundle_sender.send_bundle_udp(
        "{
        \"jsonrpc\": \"2.0\",\
        \"id\": 1,\
        \"method\": \"getTipAccounts\",\
        \"params\": []\
    }"
        .to_string(),
        4,
    );
}
