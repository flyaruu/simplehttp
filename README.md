# Simplehttp

Simple (and very immature) sync http client. Intended to have multiple implementations abstracting away API differences
between platforms

Current implementations:
 - Reqwest
 - Esp32 IDF (Embedded device with std)
 - Wasm32 Spin (Wrapping the spin http client)