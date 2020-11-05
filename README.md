# azure-embedded-sdk-rs
This builds upon the crate in https://github.com/markrad/azure-embedded-sdk-sys which is a thin bindgen layer over the Azure Embedded C SDK at https://github.com/azure/azure-sdk-for-c. 

It only addresses the IoT portion of the SDK (and may only ever). Examples are not included since, to make a useful example, it would need to select a MQTT library and TLS library in order for that example to demonstrate how to connect to an IoT hub.

Instead, an example is provided in https://github.com/markrad/azure-embedded-sdk-rs-example. 

To use simply add
```ini
[dependancies]
azure-embedded-sdk-rs = { git = "https://github.com/markrad/azure-embedded-sdk-rs.git", tag = "<specific tag version>"  }
```
I recommend using a tag since this is a work in progress but by omitting the tag keyword and value will cause the latest version will be cloned.
